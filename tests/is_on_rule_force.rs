mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use growthbook_rust_sdk::model_public::GrowthBookAttribute;
    use rstest::rstest;
    use serde_json::json;
    use std::collections::HashMap;
    use test_context::test_context;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flag_state = ctx.growthbook.is_on("not-exist-flag", false, None)?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_flag_id_is_disabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flag_state = ctx.growthbook.is_on("simple-flag-disabled", true, None)?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_flag_id_is_enabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flag_state = ctx.growthbook.is_on("simple-flag", false, None)?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_rule_not_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any-id": "018fcf12-0c45-7811-9c71-d6264ba729b1",
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx
            .growthbook
            .is_on("simple-rule-conditio", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_rule_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any-id": "018fcf11-bb67-7789-8d10-fcbb7de4ff7b",
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx
            .growthbook
            .is_on("simple-rule-conditio", true, Some(&vec))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_user_has_not_required_attribute(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vec = GrowthBookAttribute::from(json!({
            "any-other-id": "018fcf12-0c45-7811-9c71-d6264ba729b1",
        }))
        .expect("Failed to create attributes");

        let flag_state = ctx
            .growthbook
            .is_on("simple-rule-conditio", true, Some(&vec))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
