use serde::de::DeserializeOwned;

pub async fn fetch_and_decrypt<T: DeserializeOwned>(url :String, key:String) ->T{
    let mut response = gloo_net::http::Request::get(&url)
        .send()
        .await
        .expect("Failed fetching config from remote").binary().await.expect("Failed, remote json file parsing error");

     if !key.is_empty() {
          response = aes_wasm::aes256ctr::decrypt(response,key.as_bytes().try_into().expect("error,the key is invalid, maybe not long enough"
         ),key.as_bytes().try_into().expect(""));
     }
     serde_json::from_slice::<T>(response.as_slice()).unwrap()
}