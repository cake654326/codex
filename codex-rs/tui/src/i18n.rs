use once_cell::sync::Lazy;
use serde_json::json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::sync::Mutex;

static ZH_HANT: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let data = include_str!("i18n/zh-Hant.json");
    serde_json::from_str::<HashMap<String, String>>(data).unwrap_or_default()
});

static MISSING: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn log_missing(key: &str) {
    if let Ok(mut missing) = MISSING.lock()
        && missing.insert(key.to_string())
    {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/i18n/zh-Hant.missing.json");
        let mut map: HashMap<String, String> = fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        if !map.contains_key(key) {
            map.insert(key.to_string(), String::new());
            if let Ok(json) = serde_json::to_string_pretty(&map) {
                let _ = fs::write(path, json);
            }
        }
        if std::env::var("CODEX_JSON").is_ok() {
            let event = json!({
                "id": "0",
                "msg": {"type": "missing_i18n", "content": key}
            });
            #[allow(clippy::print_stdout)]
            {
                println!("{}", event);
            }
        }
    }
}

fn current_lang() -> String {
    std::env::var("LANG").unwrap_or_default()
}

pub fn tr(key: &str) -> String {
    if current_lang().starts_with("zh")
        && let Some(v) = ZH_HANT.get(key)
    {
        return v.clone();
    }
    log_missing(key);
    key.to_string()
}
