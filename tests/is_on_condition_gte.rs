mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use rstest::rstest;
    use std::collections::HashMap;
    use serde_json::json;
    use test_context::test_context;
    use growthbook_rust_sdk::model_public::GrowthBookAttribute;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flag_state = ctx.growthbook.is_on("flag-not-exists", true, None)?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_is_equals(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "version": "1.2.3"
        })).expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("gte-rule", true, Some(&vec))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_is_less_then(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "version": "1.2.2"
        })).expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("gte-rule", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_is_greater_then(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "version": "1.2.4"
        })).expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("gte-rule", true, Some(&vec))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any": "1.2.4"
        })).expect("Failed to create attributes");

        let flag_state = ctx.growthbook.is_on("gte-rule", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
