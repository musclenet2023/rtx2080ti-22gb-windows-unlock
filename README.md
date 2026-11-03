# RTX 2080 Ti 22GB Windows UEFI Unlock — archival case study

This private repository documents a Windows 11, real-hardware validation and safety-correction case based on the upstream [`nv-uefi-unlocker`](https://github.com/physicsdolphin/nv-uefi-unlocker) project. It does not claim to have invented the underlying GPU-unlock technology. The upstream project, its authors, copyright notices, and GPL terms remain applicable; see `upstream/LICENSE` and `metadata/git_commit.txt`.

The archive contains the read-only UEFI sampler source, the fail-closed geometry safety patch, historical benchmark JSON, hashes, and build notes. NVIDIA firmware, the production Unlock EFI, runtime logs, and other USB payload files are intentionally excluded because redistribution rights were not independently confirmed.

## Validated hardware and scope

The original experiment successfully executed the unlock and benchmark on this specific configuration:

- Operating system: Windows 11
- GPU: modified NVIDIA GeForce RTX 2080 Ti 22GB
- PCI ID: `10DE:1E07`
- BDF during that test: `0000:01:00.0` (a BDF is not universal)
- Geometry: `22528 MiB`
- Cold Geometry register: `0x100ce0`
- Cold Geometry value observed: `0x000002c9`
- Upstream commit: `dcfb5352f0ee3797de97ea0156f56f1145176d22`

This is a case study for the verified configuration, not a compatibility guarantee for other RTX 2080 Ti cards or other modified cards. Modified cards may use non-standard VRAM configurations. UEFI GPU register operations can cause failure to boot, display problems, data loss, or hardware damage. If Geometry is not independently confirmed, stop; do not guess a capacity or continue through a fallback path.

The sampler is read-only: it enumerates PCI configuration space and reads the documented BAR0 sample register. It is not the production transaction loader and must not be substituted for one.

## Historical benchmark result

The following values are the overall medians calculated from the saved JSON records; they are not a claim about every AI, ComfyUI, or LLM workload:

| Mode | Before | After | Change |
|---|---:|---:|---:|
| FP16 input / FP32 accumulate | 54.584 TFLOPS | 80.931 TFLOPS | +48.27% |
| FP16 input / FP16 accumulate | 83.452 TFLOPS | 83.265 TFLOPS | -0.22% |

The modes use the same recorded matrix size (`4096`), PyTorch/CUDA environment, duration target, repeats, and telemetry policy in the paired JSON files. The accumulation policy is requested through the saved PyTorch setting; the record is not an independent CUPTI or cuBLAS proof of every selected kernel's accumulator type. Different precision and accumulation modes produce different results, so the FP16/FP32 result must not be generalized to all workloads.

The unlock and benchmark were successfully performed during the original experiment. This repository整理/packaging operation did not recompile, re-unlock, or rerun the benchmark; all public measurements come from the previously saved test records.

## Windows verification and reproducibility

Use the benchmark JSON as the historical record. Any future rerun must be isolated and must match matrix size, precision policy, warmup, duration, repeats, and telemetry before comparison. `WINDOWS.md` records the safe review procedure and limitations.

See `sampler/build_command.txt`, `sampler/provenance.txt`, and `metadata/fail_closed_geometry_gate.patch` for the historical sampler rebuild recipe. The recipe was not recompiled as part of this publication update; toolchain availability and resulting output must be independently verified before relying on it.

Do not execute an EFI payload or change firmware, BIOS, drivers, or GPU state based solely on this archive. Confirm licensing, hardware safety, and the exact Geometry/BDF conditions first.

## License and attribution

The upstream GPL license and attribution are preserved in `upstream/LICENSE`. The upstream source is [`physicsdolphin/nv-uefi-unlocker`](https://github.com/physicsdolphin/nv-uefi-unlocker), recorded at the commit listed above. This archive does not grant additional rights to NVIDIA firmware or any other third-party asset.
