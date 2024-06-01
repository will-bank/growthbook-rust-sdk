mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use reqwest::StatusCode;
    use rstest::rstest;
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

        GrowthbookGatewayMock::gt_rule(
            &ctx.mock_server,
            gb_sdk,
            false,
            "1",
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, None)
            .await?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_is_equals(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::gt_rule(&ctx.mock_server, gb_sdk, true, "1.2.3", StatusCode::OK)
            .await;

        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.3")])]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_is_less_then(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::gt_rule(&ctx.mock_server, gb_sdk, true, "1.2.3", StatusCode::OK)
            .await;

        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.2")])]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_is_greater_then(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::gt_rule(&ctx.mock_server, gb_sdk, true, "1.2.3", StatusCode::OK)
            .await;

        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.4")])]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::gt_rule(&ctx.mock_server, gb_sdk, true, "1.2.3", StatusCode::OK)
            .await;

        let map = HashMap::from([(String::from("any"), vec![String::from("1.2.4")])]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
