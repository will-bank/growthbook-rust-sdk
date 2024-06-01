use serde::Deserialize;

mod commons;
mod growthbook_mocks;

#[cfg(test)]
mod test {
    use reqwest::StatusCode;
    use rstest::rstest;
    use serde_json::json;
    use test_context::test_context;
    use uuid::Uuid;

    use crate::commons::TestContext;
    use crate::growthbook_mocks::GrowthbookGatewayMock;
    use crate::ObjectValue;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::object_value(
            &ctx.mock_server,
            gb_sdk,
            json!({}),
            StatusCode::GATEWAY_TIMEOUT,
        )
        .await;

        let object_flag = ctx
            .growthbook
            .get_object_value(
                &gb_sdk.to_string(),
                "flag",
                &json!({
                    "a":"string",
                    "b":"int",
                }),
                None,
            )
            .await?;

        let value: ObjectValue = object_flag.value()?;

        assert_eq!("string", value.a);
        assert_eq!("int", value.b);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_value(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let gb_sdk = Uuid::now_v7();

        GrowthbookGatewayMock::object_value(
            &ctx.mock_server,
            gb_sdk,
            json!({
                "a":"potato",
                "b":"tomato",
            }),
            StatusCode::OK,
        )
        .await;

        let object_flag = ctx
            .growthbook
            .get_object_value(&gb_sdk.to_string(), "flag", &json!({}), None)
            .await?;

        let value: ObjectValue = object_flag.value()?;

        assert_eq!("potato", value.a);
        assert_eq!("tomato", value.b);

        Ok(())
    }
}

#[derive(Deserialize)]
struct ObjectValue {
    a: String,
    b: String,
}
