use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, OnceLock},
};

use anyhow::{Context, Result};
use kovi::{RuntimeBot, tokio::fs};
use serde::Deserialize;

pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize)]
struct ConfigFile {
    api_url: String,
    api_key: String,
    model: String,

    prompt: Option<String>,
    pet_cat_img: String,

    allow_groups: Vec<i64>,
}

pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub model: String,

    pub prompt: String,
    pub pet_cat_img: String,

    pub allow_groups: Vec<i64>,
}

pub(crate) async fn init(bot: &Arc<RuntimeBot>) -> Result<&Config> {
    let config_path = bot.get_data_path().join("config.toml");
    let config_txt = fs::read_to_string(&config_path)
        .await
        .with_context(|| format!("Failed to read config file at {}", config_path.display()))?;
    let config_file: ConfigFile = toml::from_str(&config_txt)?;

    let pet_cat = PathBuf::from_str(&config_file.pet_cat_img)?;
    let pet_cat = if pet_cat.is_absolute() {
        pet_cat
    } else {
        bot.get_data_path().join(pet_cat)
    };

    if !pet_cat.exists() {
        return Err(anyhow::anyhow!(
            "Pet Cat Picture not found from {}",
            config_path.display()
        ));
    }

    return Ok(CONFIG.get_or_init(|| Config {
        api_key: config_file.api_key,
        api_url: config_file.api_url,
        model: config_file.model,
        prompt: config_file
            .prompt
            .unwrap_or("请辨别这张图片是否包含一只真实的猫咪，而非卡通猫咪或表情包。如果这张图片包含**文字**，请回答'否'。".to_string()),
        pet_cat_img: pet_cat.to_string_lossy().into(),
        allow_groups: config_file.allow_groups,
    }));
}
