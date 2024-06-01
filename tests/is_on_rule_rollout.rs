mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::vec;

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

        GrowthbookGatewayMock::rollout_rule_condition_by_attribute(
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
            .await?;

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

        GrowthbookGatewayMock::rollout_rule_condition_by_attribute(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.0,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(String::from("any-id"), vec![Uuid::now_v7().to_string()])]);

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
    async fn should_return_enabled_true_when_percentage_is_100(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition_by_attribute(
            &ctx.mock_server,
            gb_sdk,
            true,
            1.0,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(String::from("any-id"), vec![Uuid::now_v7().to_string()])]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, Some(&map))
            .await?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_percentage_is_50_and_attribute_is_inside_range(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition_by_attribute(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.5,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf33-6362-7d67-aef6-e3ef57a2dcba")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, Some(&map))
            .await?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_percentage_is_50_and_attribute_is_outside_range(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition_by_attribute(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.5,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf3b-99f1-76e3-80a5-6e220e1ce4f2")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, Some(&map))
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_required_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::rollout_rule_condition_by_attribute(
            &ctx.mock_server,
            gb_sdk,
            true,
            0.0,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-other-id"),
            vec![Uuid::now_v7().to_string()],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
