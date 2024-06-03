use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use growthbook_rust_sdk::dto::Feature;
use growthbook_rust_sdk::nb_gateway::GBGateway;


#[derive(Clone, Deserialize)]
struct GrowthBookConfig {
    pub features: HashMap<String, Feature>,
}


async fn periodically_update_config(sdk_key: &str, gbgateway: GBGateway, config: Arc<RwLock<GrowthBookConfig>>, interval: Duration) {
    loop {
        match gbgateway.get_features(sdk_key).await {
            Ok(new_config) => {
                let mut writable_config = config.write().unwrap();
                let updated_features = GrowthBookConfig {
                    features: new_config.features
                };
                *writable_config = updated_features;
                println!("features from growthbook was updated.");
            },
            Err(e) => eprintln!("Failed to fetch configuration: {}", e),
        }
        sleep(interval).await;
    }
}
#[tokio::main]
async fn main() {
    let api_url = "https://growthbook-proxy.owill.com.br";
    let sdk_key = "sdk-whNveFXU9vGt9LFp";
    let gb_gatewway = GBGateway::new(api_url).expect("ops");
    let resp = gb_gatewway.get_features(sdk_key).await.unwrap().features;
    let update_interval = Duration::from_secs(60); // Atualiza a cada 60 segundos

    // Inicializa uma configuração vazia
    let config = Arc::new(RwLock::new(GrowthBookConfig {
        features: resp.clone(),
    }));

    // Clona a referência Arc para a tarefa de atualização
    let config_clone = Arc::clone(&config);

    // Lança a tarefa de atualização
    tokio::spawn(async move {
        periodically_update_config(sdk_key, gb_gatewway.clone(), config_clone, update_interval).await;
    });


    loop {
        {
            // Lê a configuração atualizada
            let readable_config = config.read().unwrap();
            println!("total de features {}", readable_config.features.len())
            // Exemplo de uso da configuração
            // if readable_config.is_feature_enabled("new_feature") {
            //     println!("New feature is enabled!");
            // } else {
            //     println!("New feature is disabled.");
            // }
        }

        sleep(Duration::from_secs(5)).await; // Intervalo entre verificações
    }
}