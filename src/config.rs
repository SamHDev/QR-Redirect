use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) allow_custom: bool,
    pub(crate) users: Vec<ConfigUser>,
    pub(crate) items: Vec<ConfigItem>,
}

#[derive(Deserialize)]
pub(crate) struct ConfigUser {
    pub(crate) username: String,
    pub(crate) hash: String,
}

#[derive(Deserialize)]
pub(crate) struct ConfigItem {
    pub(crate) ident: Option<String>,
    pub(crate) label: String,
    pub(crate) url: String,
}
