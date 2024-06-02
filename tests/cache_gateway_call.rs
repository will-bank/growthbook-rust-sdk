mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
    use reqwest::StatusCode;
    use rstest::rstest;
    use std::thread;
    use std::time::Duration;
    use test_context::test_context;
    use uuid::Uuid;

    use crate::commons::TestContext;
    use crate::growthbook_mocks::GrowthbookGatewayMock;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_cache_when_gateway_respond_as_success(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::expect_times(&ctx.mock_server, gb_sdk, 1, StatusCode::OK).await;

        ctx.growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        ctx.growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_ignore_cache_when_request_respond_as_failure(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::expect_times(
            &ctx.mock_server,
            gb_sdk,
            2,
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        ctx.growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        ctx.growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_ignore_cache_when_it_is_expired(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::expect_times(&ctx.mock_server, gb_sdk, 2, StatusCode::OK).await;

        ctx.growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        thread::sleep(Duration::from_millis(150));

        ctx.growthbook
            .is_on(&gb_sdk.to_string(), "flag", false, None)
            .await?;

        Ok(())
    }
}
