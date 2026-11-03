#![no_main]
#![no_std]

use core::ffi::c_void;
use uefi::boot::{self, OpenProtocolAttributes, OpenProtocolParams};
use uefi::prelude::*;
use uefi::proto::unsafe_protocol;
use uefi::{println, Status};

const NVIDIA_VENDOR: u16 = 0x10de;
const TARGET_DEVICE: u16 = 0x1e07;
const EXPECTED_SEG: usize = 0;
const EXPECTED_BUS: usize = 1;
const EXPECTED_DEV: usize = 0;
const EXPECTED_FUN: usize = 0;
const BAR0: u8 = 0;
const LOCAL_MEMORY_RANGE: u64 = 0x0010_0ce0;

#[derive(Clone, Copy)]
#[repr(u32)]
enum Width {
    Uint16 = 1,
    Uint32 = 2,
}

type AccessFn =
    unsafe extern "efiapi" fn(*mut PciIoRaw, Width, u8, u64, usize, *mut c_void) -> Status;
type ConfigFn = unsafe extern "efiapi" fn(*mut PciIoRaw, Width, u32, usize, *mut c_void) -> Status;

#[repr(C)]
struct MemAccess {
    read: AccessFn,
    _write_unreachable: *const c_void,
}

#[repr(C)]
struct ConfigAccess {
    read: ConfigFn,
    _write_unreachable: *const c_void,
}

#[repr(C)]
struct PciIoRaw {
    _poll_mem: *const c_void,
    _poll_io: *const c_void,
    mem: MemAccess,
    _io_read: *const c_void,
    _io_write: *const c_void,
    pci: ConfigAccess,
    _copy_mem: *const c_void,
    _map: *const c_void,
    _unmap: *const c_void,
    _allocate_buffer: *const c_void,
    _free_buffer: *const c_void,
    _flush: *const c_void,
    get_location: unsafe extern "efiapi" fn(
        *mut PciIoRaw,
        *mut usize,
        *mut usize,
        *mut usize,
        *mut usize,
    ) -> Status,
    _attributes: *const c_void,
    _get_bar_attributes: *const c_void,
    _set_bar_attributes: *const c_void,
    _rom_size: u64,
    _rom_image: *const c_void,
}

const _: () = assert!(core::mem::offset_of!(PciIoRaw, pci) == 6 * 8);
const _: () = assert!(core::mem::offset_of!(PciIoRaw, get_location) == 14 * 8);
const _: () = assert!(core::mem::size_of::<PciIoRaw>() == 20 * 8);

#[repr(transparent)]
#[unsafe_protocol("4cf5b200-68b8-4ca5-9eec-b23e3f50029a")]
struct PciIo(PciIoRaw);

impl PciIo {
    fn config_read_u16(&self, offset: u32) -> Result<u16, Status> {
        let mut value = 0u16;
        let status = unsafe {
            (self.0.pci.read)(
                &self.0 as *const _ as *mut _,
                Width::Uint16,
                offset,
                1,
                &mut value as *mut _ as *mut c_void,
            )
        };
        (status == Status::SUCCESS).then_some(value).ok_or(status)
    }

    fn location(&self) -> Result<(usize, usize, usize, usize), Status> {
        let (mut seg, mut bus, mut dev, mut fun) = (0, 0, 0, 0);
        let status = unsafe {
            (self.0.get_location)(
                &self.0 as *const _ as *mut _,
                &mut seg,
                &mut bus,
                &mut dev,
                &mut fun,
            )
        };
        (status == Status::SUCCESS)
            .then_some((seg, bus, dev, fun))
            .ok_or(status)
    }

    fn bar0_read_u32(&self, offset: u64) -> Result<u32, Status> {
        let mut value = 0u32;
        let status = unsafe {
            (self.0.mem.read)(
                &self.0 as *const _ as *mut _,
                Width::Uint32,
                BAR0,
                offset,
                1,
                &mut value as *mut _ as *mut c_void,
            )
        };
        (status == Status::SUCCESS).then_some(value).ok_or(status)
    }
}

fn decode_lmr(raw: u32) -> Option<u32> {
    if matches!(raw, 0 | u32::MAX) || raw >> 16 == 0xbadf {
        return None;
    }
    let magnitude = (raw >> 4) & 0x3f;
    let scale = raw & 0xf;
    let bytes = (magnitude as u64).checked_shl(scale + 20)?;
    let bytes = if raw & (1 << 30) != 0 {
        bytes.checked_div(16)?.checked_mul(15)?
    } else {
        bytes
    };
    let mib = u32::try_from(bytes >> 20).ok()?;
    (256..=64 * 1024).contains(&mib).then_some(mib)
}

#[entry]
fn main() -> Status {
    println!("RTX 2080 Ti cold LOCAL_MEMORY_RANGE sampler");
    println!("READ-ONLY: PCI config reads + one BAR0 MMIO read; no transaction");

    let handles = match boot::find_handles::<PciIo>() {
        Ok(handles) => handles,
        Err(status) => {
            println!("FAIL: PCI I/O enumeration: {status:?}");
            return Status::NOT_FOUND;
        }
    };

    let mut exact_matches = 0usize;
    for handle in handles {
        let pci = match unsafe {
            boot::open_protocol::<PciIo>(
                OpenProtocolParams {
                    handle,
                    agent: boot::image_handle(),
                    controller: None,
                },
                OpenProtocolAttributes::GetProtocol,
            )
        } {
            Ok(pci) => pci,
            Err(_) => continue,
        };
        let Ok(vendor) = pci.config_read_u16(0x00) else {
            continue;
        };
        let Ok(device) = pci.config_read_u16(0x02) else {
            continue;
        };
        if vendor != NVIDIA_VENDOR || device != TARGET_DEVICE {
            continue;
        }
        exact_matches += 1;
        let Ok((seg, bus, dev, fun)) = pci.location() else {
            println!("FAIL: target found but BDF query failed");
            continue;
        };
        println!("TARGET: {seg:04x}:{bus:02x}:{dev:02x}.{fun} 10de:1e07");
        if (seg, bus, dev, fun) != (EXPECTED_SEG, EXPECTED_BUS, EXPECTED_DEV, EXPECTED_FUN) {
            println!("STOP: BDF differs from reviewed 0000:01:00.0; BAR0 not read");
            continue;
        }
        match pci.bar0_read_u32(LOCAL_MEMORY_RANGE) {
            Ok(raw) => {
                println!("LOCAL_MEMORY_RANGE[0x100ce0] raw = {raw:#010x}");
                match decode_lmr(raw) {
                    Some(mib) => println!("DECODED_USABLE_FB = {mib} MiB"),
                    None => {
                        println!("STOP: value inaccessible or implausible; no capacity inferred")
                    }
                }
            }
            Err(status) => println!("FAIL: BAR0 read status {status:?}"),
        }
    }
    if exact_matches == 0 {
        println!("STOP: no 10de:1e07 device found");
        return Status::NOT_FOUND;
    }
    if exact_matches != 1 {
        println!("STOP: expected exactly one 10de:1e07, found {exact_matches}");
        return Status::ABORTED;
    }
    println!("DONE. Power off or reset manually; this program performs no chain-load.");
    loop {
        core::hint::spin_loop();
    }
}
