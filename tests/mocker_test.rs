#[cfg(all(test, feature = "mocker"))]
mod tests {
    use growthbook_rust_sdk::mocker::GrowthBookTest;
    use growthbook_rust_sdk::model_public::{GrowthBookAttribute, GrowthBookAttributeValue};
    use serde_json::json;

    #[tokio::test]
    async fn test_simple_flag() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");

        gb_test.add_simple_flag("my-feature", true).await;

        let client = gb_test.client();

        // Testa se a flag está ativa
        assert!(client.is_on("my-feature", None));
        assert!(!client.is_off("my-feature", None));
    }

    #[tokio::test]
    async fn test_fixed_value_flag() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");
        gb_test.add_fixed_value_flag("config-value", "production-config").await;

        let client = gb_test.client();
        let result = client.feature_result("config-value", None);
        assert_eq!(result.value.as_str().unwrap(), "production-config");
    }

    #[tokio::test]
    async fn test_conditional_flag() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");
        let condition = json!({
            "user-id": "12345"
        });

        gb_test.add_conditional_flag("premium-feature", false, condition, true).await;

        let client = gb_test.client();

        // user without correct attribute - must receive default value
        assert!(!client.is_on("premium-feature", None));

        // user with correct attribute - must receive enforcement value
        let user_attributes = vec![GrowthBookAttribute::new("user-id".to_string(), GrowthBookAttributeValue::String("12345".to_string()))];
        assert!(client.is_on("premium-feature", Some(user_attributes)));
    }

    #[tokio::test]
    async fn test_rollout_flag() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");

        // flag with 100% of coverage
        gb_test.add_rollout_flag("gradual-rollout", false, 1.0, "user-id", true).await;

        let client = gb_test.client();

        let user_attributes = vec![GrowthBookAttribute::new("user-id".to_string(), GrowthBookAttributeValue::String("test-user".to_string()))];

        // with 100% of coverage, must be active
        assert!(client.is_on("gradual-rollout", Some(user_attributes)));
    }

    #[tokio::test]
    async fn test_experiment_flag() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");

        // experiment A/B
        let variations = vec!["control", "variation-a", "variation-b"];
        let weights = vec![0.33, 0.33, 0.34];

        gb_test.add_experiment_flag("ab-test", "control", "user-id", variations, weights, 1.0).await;

        let client = gb_test.client();

        let user_attributes = vec![GrowthBookAttribute::new("user-id".to_string(), GrowthBookAttributeValue::String("test-user".to_string()))];

        let result = client.feature_result("ab-test", Some(user_attributes));

        // check the returns of variations
        let value = result.value.as_str().unwrap();
        assert!(["control", "variation-a", "variation-b"].contains(&value));
    }

    #[tokio::test]
    async fn test_multiple_flags() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");

        // add many flag scenarios
        gb_test
            .add_simple_flag("feature-1", true)
            .await
            .add_simple_flag("feature-2", false)
            .await
            .add_fixed_value_flag("config", 42)
            .await;

        let client = gb_test.client();

        assert!(client.is_on("feature-1", None));
        assert!(!client.is_on("feature-2", None));

        let config_result = client.feature_result("config", None);
        assert_eq!(config_result.value.as_i64().unwrap(), 42);
    }

    #[tokio::test]
    async fn test_remove_and_clear_flags() {
        let mut gb_test = GrowthBookTest::new().await.expect("failed to create GrowthBookTest");

        gb_test.add_simple_flag("temp-feature", true).await.add_simple_flag("permanent-feature", true).await;

        {
            let client = gb_test.client();
            assert!(client.is_on("temp-feature", None));
            assert!(client.is_on("permanent-feature", None));
        }

        gb_test.remove_flag("temp-feature").await;

        {
            let client = gb_test.client();
            assert!(!client.is_on("temp-feature", None));
            assert!(client.is_on("permanent-feature", None));
        }

        gb_test.clear_flags().await;

        {
            let client = gb_test.client();
            assert!(!client.is_on("temp-feature", None));
            assert!(!client.is_on("permanent-feature", None));
        }
    }
}
