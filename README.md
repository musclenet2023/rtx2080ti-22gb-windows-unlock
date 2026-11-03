# RTX 2080 Ti 22GB Windows UEFI Unlock — archival release

This private repository is an archival, reproducibility-oriented package for a validated RTX 2080 Ti 22GB test environment.

It contains the read-only UEFI sampler source, the fail-closed geometry safety patch, benchmark JSON, hashes, and build notes. NVIDIA firmware, the production Unlock EFI, runtime logs, and other USB payload files are intentionally excluded because redistribution rights were not independently confirmed.

Hardware record:

- GPU: NVIDIA GeForce RTX 2080 Ti 22GB
- PCI ID: `10DE:1E07`
- BDF: `0000:01:00.0`
- Geometry: `22528 MiB`
- Upstream commit: `dcfb5352f0ee3797de97ea0156f56f1145176d22`

The sampler is read-only: it enumerates PCI configuration space and reads the documented BAR0 sample register. It is not the production transaction loader and must not be substituted for one.

## Windows verification

Use the benchmark JSON as the historical record. Re-run a benchmark only in an explicitly isolated test environment and compare matrix size, precision policy, warmup, duration, repeats, and telemetry before drawing conclusions.

Do not execute an EFI payload or change firmware, BIOS, drivers, or GPU state based solely on this archive. Confirm all licensing and hardware-safety requirements first.

## Reproducibility

See `sampler/build_command.txt`, `sampler/provenance.txt`, and `metadata/fail_closed_geometry_gate.patch`. The saved sampler source and Cargo configuration are sufficient to reproduce the sampler artifact with the recorded upstream revision, subject to toolchain availability.

## License and attribution

The upstream license is preserved in `upstream/LICENSE`. Upstream project attribution remains in the saved commit and patch metadata. This archive does not grant additional rights to NVIDIA firmware or other third-party assets.
