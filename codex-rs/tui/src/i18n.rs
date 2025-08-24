use once_cell::sync::Lazy;
use std::collections::HashMap;

static ZH_HANT: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let data = include_str!("i18n/zh-Hant.json");
    serde_json::from_str::<HashMap<String, String>>(data).unwrap_or_default()
});

fn current_lang() -> String {
    std::env::var("LANG").unwrap_or_default()
}

pub fn tr(key: &str) -> String {
    if current_lang().starts_with("zh")
        && let Some(v) = ZH_HANT.get(key) {
            return v.clone();
        }
    key.to_string()
}
