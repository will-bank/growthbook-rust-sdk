mod commons;

#[cfg(test)]
mod test {
    use growthbook_rust_sdk::client::GrowthBookClient;
    use std::time::Duration;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_should_update_feature_in_runtime() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let sdk_key = Uuid::now_v7();
        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk_key}")))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
            {
                "features": {
                    "new_feature": {
                        "defaultValue": false,
                        "rules": []
                    }
                }
            }
            "#,
                "application/json",
            ))
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path(format!("/api/features/{sdk_key}")))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
            {
                "features": {
                    "new_feature": {
                        "defaultValue": true,
                        "rules": []
                    },
                    "another_feature": {
                        "defaultValue": true,
                        "rules": []
                    }
                }
            }
            "#,
                "application/json",
            ))
            .mount(&mock_server)
            .await;

        let api_url = &mock_server.uri();
        let _sdk_h = sdk_key.to_string();
        let update_interval = Duration::from_secs(1);

        let client = GrowthBookClient::new(
            api_url,
            sdk_key.to_string().as_str(),
            Some(update_interval),
            None,
        )
        .await?;

        let first_result = client.is_on("new_feature", false, None)?;
        assert!(!first_result.enabled);

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = client.is_on("another_feature", false, None)?;
        assert!(result.enabled);
        Ok(())
    }
}
