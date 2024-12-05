mod commons;

#[cfg(test)]
mod test {
    use rstest::rstest;
    use test_context::test_context;

    use crate::commons::TestContext;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_off_when_value_is_string_value_off(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let feature = ctx.growthbook.feature_result("is-off-string-flag", None);

        assert!(!feature.on);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_off_when_value_is_string_value_is_empty(ctx: &mut TestContext) -> Result<(), Box<dyn std::error::Error>> {
        let feature = ctx.growthbook.feature_result("is-off-empty-string-flag", None);

        assert!(!feature.on);

        Ok(())
    }
}
