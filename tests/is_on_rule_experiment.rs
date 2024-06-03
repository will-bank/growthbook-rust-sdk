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
        let flag_state = ctx.growthbook.is_on("flag-not-exist", false, None).await?;

        assert!(!flag_state.enabled);
        assert!(flag_state.experiment_key.is_none());

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_experiment_variant_is_disabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fdc6a-d694-73c4-8445-68b24bfb0f73")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on("experiment-rule-condition-flag", true, Some(&map))
            .await?;

        assert!(!flag_state.enabled);
        assert_eq!(
            "0",
            flag_state.experiment_key.unwrap_or(String::from("failed"))
        );

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_experiment_variant_is_enabled(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fcf36-d39b-705c-a800-dc8bdc5964be")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(
                "experiment-rule-condition-ninety-coverage-flag",
                true,
                Some(&map),
            )
            .await?;

        assert!(flag_state.enabled);
        assert_eq!(
            "2",
            flag_state.experiment_key.unwrap_or(String::from("failed"))
        );

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_attribute_is_on_disabled_range(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-id"),
            vec![String::from("018fd040-de77-72c7-af6e-6a67d430c0e6")],
        )]);

        let flag_state = ctx
            .growthbook
            .is_on(
                "experiment-rule-condition-zero-coverage-flag",
                true,
                Some(&map),
            )
            .await?;

        assert!(!flag_state.enabled);
        assert!(flag_state.experiment_key.is_none());

        Ok(())
    }
}
