use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, OnceLock},
};

use anyhow::{Context, Result};
use kovi::{RuntimeBot, event::id::ID, tokio::fs};
use serde::Deserialize;
use crate::consts::*;

pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize)]
pub(crate) struct Condition {
    pub(crate) name: String,
    pub(crate) prompt: String,
    pub(crate) prediction: String,
}

#[derive(Deserialize)]
struct ConfigFile {
    api_url: String,
    api_key: String,
    model: String,

    conditions: Vec<Condition>,
    pet_cat_img: String,

    allow_groups: Option<Vec<ID>>,
}

pub(crate) struct Config {
    pub(crate) api_url: String,
    pub(crate) api_key: String,
    pub(crate) model: String,

    pub(crate) conditions: Vec<Condition>,
    pub(crate) pet_cat_img: String,

    pub(crate) allow_groups: Option<Vec<ID>>,
}

pub(crate) async fn init(bot: &Arc<RuntimeBot>) -> Result<&Config> {
    let config_path = bot.get_data_path().join(CONFIG_PATH);
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

    if config_file.conditions.is_empty() {
        return Err(anyhow::anyhow!("No conditions provided in config file"));
    }

    Ok(CONFIG.get_or_init(|| Config {
        api_key: config_file.api_key,
        api_url: config_file.api_url,
        model: config_file.model,
        conditions: config_file.conditions,
        pet_cat_img: pet_cat.to_string_lossy().into(),
        allow_groups: config_file.allow_groups,
    }))
}
