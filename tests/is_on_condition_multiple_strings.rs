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

        GrowthbookGatewayMock::multiple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            false,
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
    async fn should_return_enabled_false_when_none_attribute_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::multiple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([
            (String::from("any-id"), vec![Uuid::now_v7().to_string()]),
            (String::from("any-key"), vec![Uuid::now_v7().to_string()]),
        ]);

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
    async fn should_return_enabled_false_when_only_id_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::multiple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([
            (
                String::from("any-id"),
                vec![String::from("018fcf11-bb67-7789-8d10-fcbb7de4ff7b")],
            ),
            (String::from("any-key"), vec![Uuid::now_v7().to_string()]),
        ]);

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
    async fn should_return_enabled_false_when_only_key_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::multiple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([
            (String::from("any-id"), vec![Uuid::now_v7().to_string()]),
            (
                String::from("any-key"),
                vec![String::from("018fcf64-1827-709a-a8ae-7d206aafb5e2")],
            ),
        ]);

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
    async fn should_return_enabled_true_when_all_attributes_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::multiple_rule_condition(
            &ctx.mock_server,
            gb_sdk,
            true,
            StatusCode::OK,
        )
        .await;

        let map = HashMap::from([
            (
                String::from("any-id"),
                vec![String::from("018fcf11-bb67-7789-8d10-fcbb7de4ff7b")],
            ),
            (
                String::from("any-key"),
                vec![String::from("018fcf64-1827-709a-a8ae-7d206aafb5e2")],
            ),
        ]);

        let flag_state = ctx
            .growthbook
            .is_on(&gb_sdk.to_string(), "flag", true, Some(&map))
            .await?;

        assert!(flag_state.enabled);

        Ok(())
    }
}
