use std::env;
use std::net::{SocketAddr, TcpListener};

use growthbook_rust_sdk::growthbook::Growthbook;
use rand::Rng;
use test_context::AsyncTestContext;
use wiremock::MockServer;

pub struct TestContext {
    pub mock_server: MockServer,
    pub growthbook: Growthbook,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> TestContext {
        let mock_server = create_mock_server().await;

        env::set_var("GROWTHBOOK_URL", mock_server.uri());

        let growthbook = Growthbook::new().expect("Failed to create growthbook gateway");

        TestContext {
            mock_server,
            growthbook,
        }
    }
}

#[allow(dead_code)]
pub async fn create_mock_server() -> MockServer {
    for _ in 1..10 {
        // try to start mock server in a random port 10 times
        let port = rand::thread_rng().gen_range(51000..54000);
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        if let Ok(listener) = TcpListener::bind(addr) {
            let mock_server = MockServer::builder().listener(listener).start().await;
            return mock_server;
        }
    }
    panic!("Failed to create mock server");
}
