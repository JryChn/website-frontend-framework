use crate::utils::resourceType::ResourceType;

pub async fn parse_to_data_url(url: String, reource_type: ResourceType) -> String {
    if url.starts_with("data") {
        url
    } else {
        let data = gloo::net::http::Request::get(url.as_str()).send().await;
        let data = async {
            match data {
                Err(_) => url,
                Ok(data) => {
                    match data.binary().await {
                        Err(_) => url,
                        Ok(data) => {
                            gloo::file::futures::read_as_data_url(&gloo::file::Blob::new_with_options(data.as_slice(),Some(&reource_type.get_reources_type()))).await.unwrap_or_else(|_| url)
                        }
                    }
                }
            }
        }.await;
        data
    }
}
