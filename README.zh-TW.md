# RTX 2080 Ti 22GB Windows UEFI Unlock——封存案例

本私有儲存庫是以上游 [`nv-uefi-unlocker`](https://github.com/physicsdolphin/nv-uefi-unlocker) 為基礎的 Windows 11 實機驗證與安全修正案例；本專案不宣稱原創整個 GPU 解鎖技術。上游專案、作者、著作權聲明與 GPL 授權仍然適用，請參閱 `upstream/LICENSE` 與 `metadata/git_commit.txt`。

內容包括唯讀 UEFI sampler 原始碼、fail-closed geometry 安全修正 Patch、歷史 Benchmark JSON、雜湊與建置紀錄。因 NVIDIA 韌體、正式 Unlock EFI、執行日誌及其他 USB payload 的再散布權限尚未獨立確認，這些檔案刻意不納入。

## 已驗證硬體與適用範圍

原始實驗曾在以下特定配置成功執行解鎖與 Benchmark：

- 作業系統：Windows 11
- GPU：魔改 NVIDIA GeForce RTX 2080 Ti 22GB
- PCI ID：`10DE:1E07`
- 當次測試 BDF：`0000:01:00.0`（BDF 並非所有電腦固定相同）
- Geometry：`22528 MiB`
- Cold Geometry register：`0x100ce0`
- 當次讀值：`0x000002c9`
- 上游 commit：`dcfb5352f0ee3797de97ea0156f56f1145176d22`

這只是已驗證配置的案例，不保證適用於其他 RTX 2080 Ti 或其他魔改卡。魔改顯卡可能有非標準 VRAM 配置；UEFI GPU 暫存器操作可能造成無法開機、顯示異常、資料遺失或硬體損壞。若 Geometry 未獨立確認，必須停止，不可猜測容量或透過 fallback 繼續。

Sampler 僅讀取 PCI configuration space 與指定 BAR0 範例暫存器，不執行 GPU transaction，也不是正式 Unlock loader。

## 歷史 Benchmark 結果

以下是由已保存 JSON 計算的整體中位數，不代表所有 AI、ComfyUI 或 LLM 工作負載都會有相同提升：

| 模式 | 解鎖前 | 解鎖後 | 變化 |
|---|---:|---:|---:|
| FP16 input / FP32 accumulate | 54.584 TFLOPS | 80.931 TFLOPS | +48.27% |
| FP16 input / FP16 accumulate | 83.452 TFLOPS | 83.265 TFLOPS | -0.22% |

兩份 JSON 使用相同記錄矩陣大小（`4096`）、PyTorch/CUDA 環境、目標測量時間、重複次數與 telemetry 方法。累加模式是透過保存的 PyTorch 設定提出要求；紀錄不是獨立的 CUPTI 或 cuBLAS 證據，不能證明每個實際 kernel 的累加型別。不同精度與累加模式會產生不同結果，因此不可將 FP16/FP32 結果泛化至所有工作負載。

解鎖與 Benchmark 曾在原始實驗中成功執行；本次 GitHub 專案整理沒有重新編譯、重新解鎖或重新執行 Benchmark，所有公開數據均來自先前保存的原始測試紀錄。

## Windows 驗證與重建

歷史效能結果位於 `benchmarks/`。未來重測必須在隔離環境中，保持矩陣大小、精度設定、warmup、測量時間、重複次數與 telemetry 方法一致。`WINDOWS.md` 記錄安全檢查程序與限制。

請閱讀 `sampler/build_command.txt`、`sampler/provenance.txt` 與 `metadata/fail_closed_geometry_gate.patch`。建置配方在本次發布更新中未重新編譯；工具鏈與輸出結果仍須另行驗證。

不要僅依本封存資料執行 EFI、修改韌體、BIOS、驅動程式或 GPU 狀態；先確認授權、硬體安全與 Geometry/BDF 條件。

## 授權與著作權

上游 GPL 授權與著作權資訊保留於 `upstream/LICENSE`。上游來源為 [`physicsdolphin/nv-uefi-unlocker`](https://github.com/physicsdolphin/nv-uefi-unlocker)，並記錄於上述 commit。本封存不授予 NVIDIA 韌體或其他第三方資產額外的再散布權利。
