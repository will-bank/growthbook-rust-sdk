use growthbook_rust_sdk::client::GrowthBookClient;
use growthbook_rust_sdk::model_public::{FeatureResult, GrowthBookAttribute};
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "<your_growthbook_url>";
    let sdk_key = "some-test-api-key";
    let gb_client = GrowthBookClient::new(api_url, sdk_key, None, None).await?;

    loop {
        {
            let feature_name = "rust-sdk-test-feature";
            println!("total features {:?}", gb_client.total_features());

            let on = gb_client.is_on(feature_name, None);
            println!("feature: {} on {:?}", feature_name, on);

            let feature = gb_client.feature_result(feature_name, None);
            println!("feature: {} string value {:?}", feature_name, feature.value); // value is serde serde_json::Value

            let string_feature = gb_client.feature_result(feature_name, None);
            let string = string_feature.value_as::<String>()?;
            println!("feature: {} string value {:?}", feature_name, string);

            let custom_feature = gb_client.feature_result(feature_name, None);
            let custom = custom_feature.value_as::<Custom>()?;
            println!("feature: {} custom first value {:?}", feature_name, custom.first);
            println!("feature: {} custom second value {:?}", feature_name, custom.second);
        }

        // Example of creating FeatureResult for testing/mocking
        let test_result = FeatureResult::new(json!("test-value"), true, "test".to_string());
        println!("Test FeatureResult: on={}, value={}", test_result.on, test_result.value);

        sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Deserialize)]
pub struct Custom {
    pub first: String,
    pub second: i64,
}
