mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use rstest::rstest;
    use std::collections::HashMap;
    use std::vec;
    use test_context::test_context;
    use uuid::Uuid;

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
    async fn should_return_enabled_false_when_percentage_is_0(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("any-id"), vec![Uuid::now_v7().to_string()])]);

        let flag_state = ctx.growthbook.is_on(
            "rollout-zero-percentage-flag-condition-by-attribute",
            true,
            Some(&map),
        )?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_percentage_is_100(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("any-id"), vec![Uuid::now_v7().to_string()])]);

        let flag_state = ctx.growthbook.is_on(
            "rollout-one-hundred-percentage-flag-condition-by-attribute",
            false,
            Some(&map),
        )?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_percentage_is_50_and_attribute_is_inside_range(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fde8a-77e6-7c15-93d3-d4cc4f018442")],
        )]);

        let flag_state =
            ctx.growthbook
                .is_on("rollout-flag-condition-by-attribute", false, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_percentage_is_50_and_attribute_is_outside_range(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf3b-99f1-76e3-80a5-6e220e1ce4f2")],
        )]);

        let flag_state =
            ctx.growthbook
                .is_on("rollout-flag-condition-by-attribute", false, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_required_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-other-id"),
            vec![Uuid::now_v7().to_string()],
        )]);

        let flag_state =
            ctx.growthbook
                .is_on("rollout-flag-condition-by-attribute", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
