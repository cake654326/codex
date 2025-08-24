# i18n 與 UI 攔截

此文件說明如何翻譯 TUI 中的介面文字，以及在啟用 `CODEX_JSON` 時如何攔截並輸出結構化事件。

## 介面文字翻譯策略

- `SlashCommand::description`、`Line::from` 等介面文字請直接撰寫英文字串，不需使用 `tr!()` 或 `t!()` 包裝。
- Codex 會在 `Line::from` 之後依 `LANG` 判斷是否存在對應的翻譯檔；若找到即顯示翻譯，若沒有：
  - 字串會被記錄於 `codex-rs/tui/src/i18n/zh-Hant.missing.json`。
  - 當設定 `CODEX_JSON=1` 時，會額外輸出 `missing_i18n` 事件到 JSON 流，方便後續擷取。

### `SlashCommand::description`

```rust
pub fn description(&self) -> &'static str {
    match self {
        SlashCommand::Init => "create an AGENTS.md file with instructions for Codex",
        SlashCommand::Status => "show current session configuration and token usage",
        // ...
    }
}
```

### `Line::from`

```rust
lines.push(Line::from("Requested:"));
lines.push(Line::from(format!("patch approval decision: {}", decision)));
```

## `CODEX_JSON` 事件格式

設定環境變數 `CODEX_JSON` 後，CLI 會改以 JSON 輸出。前兩行為設定摘要與提示，之後每行都是一個事件：

```json
{"cwd":"/path/to/repo","model":"gpt-4o-mini"}
{"prompt":"hello"}
{"id":"0","msg":{"type":"session_configured"}}
{"id":"0","msg":{"type":"agent_message_delta","content":"hi"}}
{"id":"0","msg":{"type":"task_complete","last_agent_message":"hi"}}
{"id":"0","msg":{"type":"missing_i18n","content":"Requested:"}}
```

每行都是獨立的 JSON 物件，可逐行解析。

## 新增語系字串與快照更新流程

1. 在 `codex-rs/tui/src/i18n/zh-Hant.json` 補上缺少的鍵值。
2. 執行測試產生新的快照：
   ```shell
   cargo test -p codex-tui
   ```
3. 檢查並接受快照：
   ```shell
   cargo insta pending-snapshots -p codex-tui
   cargo insta accept -p codex-tui  # 若變更正確
   ```
4. 格式化與修正 lints：
   ```shell
   just fmt
   just fix -p codex-tui
   ```

請在新增或更新可翻譯 UI 文字時遵循上述流程。

