mod commons;

#[cfg(test)]
mod test {
    use rstest::rstest;
    use serde_json::json;
    use test_context::test_context;

    use growthbook_rust_sdk::model_public::GrowthBookAttribute;

    use crate::commons::TestContext;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flag_state = ctx.growthbook.is_on("flag-not-exist", true, None)?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_none_data_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any-data": [1, 2]
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("elem-match-eq", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_one_data_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any-data": [1, 2, 3]
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("elem-match-eq", false, Some(&vec))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_only_one_data_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any-data": [3]
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("elem-match-eq", false, Some(&vec))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_required_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "version": "3.0"
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("elem-match-eq", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
