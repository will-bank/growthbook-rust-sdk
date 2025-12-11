#[cfg(test)]
mod test {
    use growthbook_rust_sdk::client::{GrowthBookClient, GrowthBookClientTrait};
    use growthbook_rust_sdk::dto::GrowthBookResponse;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[tokio::test]
    async fn should_create_client_from_features() -> Result<(), Box<dyn std::error::Error>> {
        let response: GrowthBookResponse = serde_json::from_value(json!({
            "features": {
                "simple-flag": {
                    "defaultValue": true
                },
                "disabled-flag": {
                    "defaultValue": false
                }
            }
        }))?;

        let client = GrowthBookClient::from_features(response);

        assert!(client.is_on("simple-flag", None));
        assert!(!client.is_on("disabled-flag", None));
        assert!(!client.is_on("non-existent", None));

        Ok(())
    }
}
