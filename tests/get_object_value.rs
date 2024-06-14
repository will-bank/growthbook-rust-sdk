use serde::Deserialize;

mod commons;

#[cfg(test)]
mod test {
    use rstest::rstest;
    use serde_json::json;
    use test_context::test_context;

    use crate::commons::TestContext;
    use crate::ObjectValue;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let object_flag = ctx.growthbook.object_feature(
            "flag-not-exists",
            &json!({
                "a":"string",
                "b":"int",
            }),
            None,
        )?;

        let value: ObjectValue = object_flag.value()?;

        assert_eq!("string", value.a);
        assert_eq!("int", value.b);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_value(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let object_flag = ctx.growthbook.object_feature("object-flag", &json!({}), None)?;

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
