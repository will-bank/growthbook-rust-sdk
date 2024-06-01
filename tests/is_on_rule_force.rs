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

        GrowthbookGatewayMock::simple_flag(
            &ctx.mock_server,
            gb_sdk,
            false,
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_flag_id_disabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::simple_flag(&ctx.mock_server, gb_sdk, false, StatusCode::OK).await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, None)
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_flag_id_disabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::simple_flag(&ctx.mock_server, gb_sdk, true, StatusCode::OK).await;

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_rule_not_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::simple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf12-0c45-7811-9c71-d6264ba729b1")],
        )]);

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
    async fn should_return_enabled_true_when_rule_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::simple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf11-bb67-7789-8d10-fcbb7de4ff7b")],
        )]);

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
    async fn should_return_enabled_false_when_user_has_not_required_attribute(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::simple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([(
            String::from("any-other-id"),
            vec![String::from("018fcf12-0c45-7811-9c71-d6264ba729b1")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
