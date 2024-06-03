use growthbook_rust_sdk::client::GrowthBookClient;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let api_url = "https://growthbook-proxy.owill.com.br";
    let sdk_key = "some-test-api-key";
    let gb_client = GrowthBookClient::new(api_url, sdk_key, None, None)
        .await
        .expect("cannot create gb client");

    loop {
        {
            let feature = "rust-sdk-test-feature";
            println!("total features {:?}", gb_client.total_features().await);
            let result = gb_client
                .is_on(feature, false, None)
                .await
                .expect("problem to check feature");
            println!("feature: {} result {:?}", feature, result.enabled)
        }

        sleep(Duration::from_secs(5)).await;
    }
}
