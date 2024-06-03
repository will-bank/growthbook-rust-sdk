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
        let flag_state = ctx.growthbook.is_on("lte-flag-not-exist", true, None)?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_is_equals(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.3")])]);

        let flag_state = ctx.growthbook.is_on("lte-flag", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_is_less_then(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.2")])]);

        let flag_state = ctx.growthbook.is_on("lte-flag", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_is_greater_then(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.4")])]);

        let flag_state = ctx.growthbook.is_on("lte-flag", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_attribute_is_missing(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("any"), vec![String::from("1.2.4")])]);

        let flag_state = ctx.growthbook.is_on("lte-flag", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }
}
