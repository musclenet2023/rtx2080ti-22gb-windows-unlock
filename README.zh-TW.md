# RTX 2080 Ti 22GB Windows UEFI Unlock——封存與重現資料

本私有儲存庫保存已驗證 RTX 2080 Ti 22GB 測試環境的重現資料。

內容包括唯讀 UEFI sampler 原始碼、fail-closed geometry 安全修正 Patch、Benchmark JSON、雜湊與建置紀錄。因 NVIDIA 韌體、正式 Unlock EFI、執行日誌及其他 USB payload 的再散布權限尚未獨立確認，這些檔案刻意不納入。

硬體紀錄：

- GPU：NVIDIA GeForce RTX 2080 Ti 22GB
- PCI ID：`10DE:1E07`
- BDF：`0000:01:00.0`
- Geometry：`22528 MiB`
- 上游 commit：`dcfb5352f0ee3797de97ea0156f56f1145176d22`

Sampler 僅讀取 PCI configuration space 與指定 BAR0 範例暫存器，不執行 GPU transaction，也不是正式 Unlock loader。

## Windows 端驗證

歷史效能結果位於 `benchmarks/`。若要重測，必須保持矩陣大小、精度設定、warmup、測量時間、重複次數與 telemetry 方法一致，並在隔離測試環境中進行。

不要僅依本封存資料執行 EFI、修改韌體、BIOS、驅動程式或 GPU 狀態；先確認授權與硬體安全條件。

## 重建

請閱讀 `sampler/build_command.txt`、`sampler/provenance.txt` 與 `metadata/fail_closed_geometry_gate.patch`。已保存的 sampler 原始碼、Cargo 設定與記錄的上游版本可用於重建 sampler，但本次未重新編譯。

## 授權與著作權

上游授權保留於 `upstream/LICENSE`。上游專案來源與著作權未被移除。本封存不授予 NVIDIA 韌體或其他第三方資產額外的再散布權利。
