# Windows 操作、重建與驗證注意事項

## 已完成與未完成的驗證

原始實驗在 Windows 11 的指定魔改 RTX 2080 Ti 22GB 配置上成功執行了解鎖與 Benchmark。本次 GitHub 整理只封存既有資料，沒有重新編譯、重新執行 Unlock 或重新執行 Benchmark；任何重建輸出與效能結果都尚未因本次發布而重新驗證。

## 唯讀審查

1. 先確認 GPU、PCI ID、Geometry 與當次測試條件；BDF `0000:01:00.0` 只是當次位置，不是固定值。
2. 只對已取得且有權限使用的檔案執行 SHA-256 驗證；不要把正式 EFI、NVIDIA 韌體或執行日誌放入公開發布內容。
3. Benchmark 比較必須使用相同矩陣大小、精度、warmup、測量時間、重複次數與 telemetry 方法。JSON 的 `method_note` 說明 FP16 accumulation 設定只是請求的 PyTorch/cuBLAS policy，不是獨立 kernel 證明。
4. 若 Geometry 未確認，立即停止；不得猜測容量或透過 fallback 繼續。

## 重建步驟（僅為歷史配方，未在本次重新驗證）

1. 依 `metadata/git_commit.txt` 取得上游 commit。
2. 依 `metadata/fail_closed_geometry_gate.patch` 套用安全修正，先執行 `git apply --check`。
3. 將 `sampler/` 內的 Cargo workspace、lockfile、`.cargo/config.toml` 與 sampler source 放入工作樹。
4. 依 `sampler/build_command.txt` 建置 read-only sampler；本次發布沒有執行這個命令。

這不是通用解鎖工具，也不提供可直接執行的正式 Unlock EFI、NVIDIA 韌體或 `loader.log`。任何 EFI GPU 暫存器操作都需另行審查與明確授權。

## 硬體風險與相容性

魔改顯卡可能存在非標準 VRAM 配置。UEFI GPU 暫存器操作可能造成無法開機、顯示異常、資料遺失或硬體損壞。本案例只適用於已驗證的 `10DE:1E07 / 22528 MiB` 配置，不保證其他 RTX 2080 Ti 或魔改卡適用。
