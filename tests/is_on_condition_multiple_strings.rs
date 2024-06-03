mod commons;

#[cfg(test)]
mod test {
    use crate::commons::TestContext;
    use rstest::rstest;
    use std::collections::HashMap;
    use test_context::test_context;
    use uuid::Uuid;

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_default_when_fail_to_call_growthbook(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flag_state = ctx.growthbook.is_on("flag-not-found", true, None)?;

        assert!(flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_none_attribute_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([
            (String::from("any-id"), vec![Uuid::now_v7().to_string()]),
            (String::from("any-key"), vec![Uuid::now_v7().to_string()]),
        ]);

        let flag_state = ctx.growthbook.is_on("flag", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_only_id_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([
            (
                String::from("any-id"),
                vec![String::from("018fcf11-bb67-7789-8d10-fcbb7de4ff7b")],
            ),
            (String::from("any-key"), vec![Uuid::now_v7().to_string()]),
        ]);

        let flag_state = ctx.growthbook.is_on("flag", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_false_when_only_key_match(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([
            (String::from("any-id"), vec![Uuid::now_v7().to_string()]),
            (
                String::from("any-key"),
                vec![String::from("018fcf64-1827-709a-a8ae-7d206aafb5e2")],
            ),
        ]);

        let flag_state = ctx.growthbook.is_on("flag", true, Some(&map))?;

        assert!(!flag_state.enabled);

        Ok(())
    }

    #[test_context(TestContext)]
    #[rstest]
    #[tokio::test]
    async fn should_return_enabled_true_when_all_attributes_matches(
        ctx: &mut TestContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let map = HashMap::from([
            (
                String::from("any-id"),
                vec![String::from("018fcf11-bb67-7789-8d10-fcbb7de4ff7b")],
            ),
            (
                String::from("any-key"),
                vec![String::from("018fcf64-1827-709a-a8ae-7d206aafb5e2")],
            ),
        ]);

        let flag_state = ctx.growthbook.is_on("flag", true, Some(&map))?;

        assert!(flag_state.enabled);

        Ok(())
    }
}
