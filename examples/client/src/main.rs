use growthbook_rust_sdk::client::GrowthBookClient;
use std::time::Duration;
use tokio::time::sleep;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "<your_growthbook_url>";
    let sdk_key = "some-test-api-key";
    let gb_client = GrowthBookClient::new(api_url, sdk_key, None, None).await?;

    loop {
        {
            let feature_name = "rust-sdk-test-feature";
            println!("total features {:?}", gb_client.total_features().await);

            let on = gb_client.is_on(feature_name, None);
            println!("feature: {} on {:?}", feature_name, on);

            let feature = gb_client.feature_result(feature_name, None);
            println!("feature: {} string value {:?}", feature_name, feature.value); // value is serde serde_json::Value

            let string_feature = gb_client.feature_result(feature_name, None);
            let string = string_feature.value_as::<String>()?;
            println!("feature: {} string value {:?}", feature_name, string)

            let custom_feature = gb_client.feature_result(feature_name, None);
            let custom = custom_feature.value_as::<Custom>()?;
            println!("feature: {} custom first value {:?}", feature_name, custom.first);
            println!("feature: {} custom second value {:?}", feature_name, custom.second);
        }

        sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Deserialize)]
pub struct Custom {
    pub first: String,
    pub second: i64,
}
