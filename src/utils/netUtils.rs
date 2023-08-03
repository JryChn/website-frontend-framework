pub async fn parse_to_data_url(url: String) -> String {
    let data = gloo_net::http::Request::get(url.as_str()).send().await;
    let data = match data {
        Err(_) => url,
        Ok(data) => {
            match data.binary().await {
                Err(_) => url,
                Ok(data) => {
                    gloo_file::futures::read_as_data_url(&gloo_file::Blob::new(data.as_slice())).await.unwrap_or_else(|_|url)
                }
            }
        }
    };
    data
}
