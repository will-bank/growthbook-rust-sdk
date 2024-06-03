mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use rstest::rstest;
    use test_context::test_context;
    use uuid::Uuid;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expected_value = Uuid::now_v7().to_string();

        let string_flag =
            ctx.growthbook
                .get_string_value("not-found", &expected_value.clone(), None)?;

        assert_eq!(expected_value, string_flag.value);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_value(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let expected_value = "018fcf11-bb67-7789-8d10-fcbb7de4ff7b";
        let string_flag =
            ctx.growthbook
                .get_string_value("fixed-value", &Uuid::now_v7().to_string(), None)?;

        assert_eq!(expected_value, string_flag.value);

        Ok(())
    }
}
