mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use crate::growthbook_mocks::GrowthbookGatewayMock;
    use reqwest::StatusCode;
    use rstest::rstest;

    use test_context::test_context;
    use uuid::Uuid;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            false,
            1.0,
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, None)
            .await;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_percentage_is_0(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.0,
            StatusCode::OK,
        )
        .await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, None)
            .await;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_percentage_is_100(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            1.0,
            StatusCode::OK,
        )
        .await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
