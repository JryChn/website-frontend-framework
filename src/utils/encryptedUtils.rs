use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

use crate::model::{CommonConfig, ConfigurationTemplate};
use crate::utils::cache::Cache;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::MP4;

lazy_static! {
    static ref CONFIG: CommonConfig = serde_json::from_str(include_str!("../config.json"))
        .expect("ERROR When Loading Configuration");
    static ref CONFIGURATION: Mutex<Vec<ConfigurationTemplate>> =
        Mutex::from(vec![serde_json::from_str::<ConfigurationTemplate>(
            include_str!("../defaultConfig/commonPage.json")
        )
        .expect("Error When loading Configuration")]);
}

pub async fn fetch_and_decrypt<T: DeserializeOwned>(url: &str) -> serde_json::Result<T> {
    let key = CONFIG.encrypted_key;
    let mut response = gloo::net::http::Request::get(&url)
        .send()
        .await
        .expect("Failed fetching config from remote")
        .binary()
        .await
        .expect("Failed, remote json file parsing error");
    if !key.is_empty() {
        response = aes_wasm::aes256ctr::decrypt(
            response,
            key.as_bytes()
                .try_into()
                .expect("error,the key is invalid, maybe not long enough"),
            key.as_bytes().try_into().expect(""),
        );
    }
    serde_json::from_slice::<T>(response.as_slice())
}

pub async fn fetch_configuration() -> ConfigurationTemplate {
    if !CONFIG.configuration_fetching_url.is_empty() {
        return Cache::fetch_or_cache(CONFIG.configuration_fetching_url,|| async {
            let mut configuration =  fetch_and_decrypt::<ConfigurationTemplate>(CONFIG.configuration_fetching_url)
                .await
                .expect("fetch configuration error");
            configuration.welcome.animation_url.dark =
                parse_to_data_url(configuration.welcome.animation_url.dark, MP4).await;
            configuration.welcome.animation_url.light =
                parse_to_data_url(configuration.welcome.animation_url.light, MP4).await;
            serde_json::to_vec(&configuration).unwrap()
        }).await.unwrap();
    }
    CONFIGURATION.lock().unwrap()[0].clone()
}
