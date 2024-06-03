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
        let map = HashMap::from([(
            String::from("any-data"),
            vec![String::from("1"), String::from("2")],
        )]);

        let flag_state = ctx.growthbook.is_on("elem-match-eq", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_one_data_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(
            String::from("any-data"),
            vec![String::from("1"), String::from("2"), String::from("3")],
        )]);

        let flag_state = ctx.growthbook.is_on("elem-match-eq", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_only_one_data_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("any-data"), vec![String::from("3")])]);

        let flag_state = ctx.growthbook.is_on("elem-match-eq", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_required_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("version"), vec![String::from("3.0")])]);

        let flag_state = ctx.growthbook.is_on("elem-match-eq", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
