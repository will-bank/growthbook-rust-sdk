mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
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
        let expected_value = Uuid::now_v7().to_string();

        GrowthbookGatewayMock::string_value(
            &ctx.mock_server,
            gb_sdk,
            Uuid::now_v7().to_string(),
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        let string_flag = ctx
            .growthbook
            .get_string_value(&gb_sdk.to_string(), "flag", &expected_value.clone(), None)
            .await?;

        assert_eq!(expected_value, string_flag.value);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_value(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();
        let expected_value = Uuid::now_v7().to_string();

        GrowthbookGatewayMock::string_value(
            &ctx.mock_server,
            gb_sdk,
            expected_value.clone(),
            StatusCode::OK,
        )
        .await;

        let string_flag = ctx
            .growthbook
            .get_string_value(
                &gb_sdk.to_string(),
                "flag",
                &Uuid::now_v7().to_string(),
                None,
            )
            .await?;

        assert_eq!(expected_value, string_flag.value);

        Ok(())
    }
}
