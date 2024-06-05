mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use growthbook_rust_sdk::model_public::GrowthBookAttribute;
    use rstest::rstest;
    use serde_json::json;
    use test_context::test_context;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_regex_not_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "version": "1.2.3",
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("regex-rule", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_regex_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "version": "3",
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("regex-rule", true, Some(&vec))?;

        assert!(flag_state.enabled);

        Ok(())
    }
}
