mod config;

use kovi::{
    Message, PluginBuilder as plugin, RuntimeBot,
    bot::runtimebot::kovi_api::SetAccessControlList,
    event::GroupMsgEvent,
    log::{error, info},
    serde_json::{Value, json},
};
use std::sync::{Arc, OnceLock};

static BOT: OnceLock<Arc<RuntimeBot>> = OnceLock::new();
static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    BOT.get_or_init(|| bot.clone());
    CLIENT.get_or_init(|| reqwest::ClientBuilder::new().build().unwrap());

    let config = config::init(&bot).await.unwrap();

    bot.set_plugin_access_control("pet-cat", true).unwrap();
    bot.set_plugin_access_control_list(
        "pet-cat",
        true,
        SetAccessControlList::Adds(config.allow_groups.clone()),
    )
    .unwrap();
    plugin::on_group_msg(on_group_msg);

    info!("Ready to pet cats!");
}

async fn on_group_msg(event: Arc<GroupMsgEvent>) {
    let imgs = event.message.get("image");

    for img in imgs {
        let map = img.data.as_object();
        if let None = img.data.as_object() {
            info!("No data provided by image segment. (Strange!)");
            continue;
        }

        let url = map.unwrap().get("url");
        if let None = url {
            info!("No url provided by image segment. (Strange!)");
            continue;
        }

        let mut url = url.unwrap().as_str().unwrap().to_string();
        if url.starts_with("https") {
            url = url.replace("https", "http");
        }
        info!("Image url: {}", url);

        if predict_cat(&url).await {
            info!("Cat detected, sending pet cat meme...");
            send_pet_cat(event.group_id).await;
        } else {
            info!("No cat detected.")
        }
    }
}

async fn predict_cat(url: &str) -> bool {
    let config = config::CONFIG.get().unwrap();
    let client = CLIENT.get().unwrap();

    info!("Predicting cat for image: {}", url);

    let req = match client
        .post(&config.api_url)
        .bearer_auth(&config.api_key)
        .json(&json!({
            "model": config.model,
            "messages": [
                {
                    "role": "system",
                    "content": [
                        {
                            "type": "text",
                            "text": "你是一个专业的图片分辨专家，可以精确地依据用户的指示，分辨图片中是否包含某一特定物体。**你只能回答 是 或 否，不要做出多余的回答或进行解释**。"
                        }
                    ]
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": url
                            }
                        },
                        {
                            "type": "text",
                            "text": config.prompt
                        }
                    ]
                }
            ]
        }))
        .build(){
            Ok(req) => req,
            Err(e) => {
                error!("Failed to build request: {}", e);
                return false;
            }
        };

    let resp = match client.execute(req).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to get response: {}", e);
            return false;
        }
    };

    let resp: Value = match resp.json().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to parse response: {}", e);
            return false;
        }
    };

    let resp = resp.as_object().unwrap();
    
    let Some(result) = resp.get("choices") else {
        info!("Invalid response: {:?}", resp);
        return false;
    };

    let Some(result) = result.as_array().unwrap().get(0) else {
        info!("No choice provided: {:?}", resp);
        return false;
    };

    let result = result["message"]["content"].as_str();

    if let Some(s) = result {
        return s.trim() == "是";
    }

    false
}

async fn send_pet_cat(group: i64) {
    let bot = BOT.get().unwrap();
    let config = config::CONFIG.get().unwrap();
    bot.send_group_msg(
        group,
        Message::from_value(json!([
            {
                "type":"image",
                "data": {
                    "file": config.pet_cat_img,
                }
            }
        ]))
        .unwrap(),
    );
}
