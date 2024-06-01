mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
    use reqwest::StatusCode;
    use rstest::rstest;
    use std::collections::HashMap;
    use test_context::test_context;
    use uuid::Uuid;

    use crate::commons::TestContext;
    use crate::growthbook_mocks::GrowthbookGatewayMock;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::experiment_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            1.0,
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        assert!(!flag_state.enabled);
        assert!(flag_state.experiment_key.is_none());

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_experiment_variant_is_disabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::experiment_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            1.0,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fd04d-83ce-73c7-af80-77edbf36576d")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);
        assert_eq!(
            "0",
            flag_state.experiment_key.unwrap_or(String::from("failed"))
        );

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_experiment_variant_is_enabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::experiment_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.9,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf36-d39b-705c-a800-dc8bdc5964be")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(flag_state.enabled);
        assert_eq!(
            "1",
            flag_state.experiment_key.unwrap_or(String::from("failed"))
        );

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_attribute_is_on_disabled_range(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::experiment_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.0,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fd040-de77-72c7-af6e-6a67d430c0e6")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);
        assert!(flag_state.experiment_key.is_none());

        Ok(())
    }
}
