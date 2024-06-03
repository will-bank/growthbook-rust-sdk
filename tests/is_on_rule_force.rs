mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use rstest::rstest;
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
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf12-0c45-7811-9c71-d6264ba729b1")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on("simple-rule-conditio", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_rule_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf11-bb67-7789-8d10-fcbb7de4ff7b")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on("simple-rule-conditio", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_user_has_not_required_attribute(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-other-id"),
            vec![String::from("018fcf12-0c45-7811-9c71-d6264ba729b1")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on("simple-rule-conditio", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
