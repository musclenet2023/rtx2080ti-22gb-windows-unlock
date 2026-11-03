# Windows 操作與驗證注意事項

1. 先確認 GPU、PCI ID、BDF 與記錄的硬體條件一致。
2. 僅以唯讀方式驗證檔案 SHA-256；不要把正式 EFI、韌體或執行日誌放入公開發布內容。
3. Benchmark 必須使用相同矩陣大小、精度、warmup、測量時間與重複次數；歷史 JSON 的 `method_note` 說明了 FP16 accumulation 設定的限制。
4. 將本專案視為測試與封存資料，不是通用解鎖工具。任何 EFI 執行、韌體操作或 BIOS 變更都需要另行審查與明確授權。

目前封存不包含可直接執行的正式 Unlock EFI、NVIDIA 韌體或 loader.log。
