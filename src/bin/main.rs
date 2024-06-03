use growthbook_rust_sdk::client::GrowthBookClient;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let api_url = "https://growthbook-proxy.owill.com.br";
    let sdk_key = "sdk-whNveFXU9vGt9LFp";
    let update_interval = Duration::from_secs(20); // Atualiza a cada 60 segundos
    let gb_client = GrowthBookClient::new(api_url, sdk_key, update_interval)
        .await
        .expect("cannot create gb client");

    loop {
        {
            // Lê a configuração atualizada
            let feature = "rust-sdk-test-feature";
            println!("total de features {:?}", gb_client.total_features().await);
            let result = gb_client
                .is_on(feature, false, None)
                .await
                .expect("problem to check feature");
            println!("feature: {} resultado {:?}", feature, result.enabled)
        }

        sleep(Duration::from_secs(5)).await; // Intervalo entre verificações
    }
}
