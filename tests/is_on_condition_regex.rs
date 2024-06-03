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
    async fn should_return_enabled_false_when_regex_not_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("version"), vec![String::from("1.2.3")])]);

        let flag_state = ctx.growthbook.is_on("regex-rule", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_regex_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([(String::from("version"), vec![String::from("3")])]);

        let flag_state = ctx.growthbook.is_on("regex-rule", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }
}
