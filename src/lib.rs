pub use temp_env_vars_core::TestEnvScope;
pub use temp_env_vars_macro::temp_env_vars;

#[cfg(test)]
mod test {
    use super::temp_env_vars;

    use assertor::{assert_that, ResultAssertion};
    use serial_test::serial;
    use temp_env_vars_core::TestEnvScope;

    #[test]
    #[serial] // Advised to use serial_test if other env-tests are not used with annotated with "temp_env_vars"
    #[temp_env_vars]
    fn test_with_macro() {
        assert_that!(std::env::var("FOO")).is_err();
        std::env::set_var("FOO", "MACRO_TEST");
    }

    #[test]
    #[serial] // Advised to use the serial_test create to avoid env mixup
    fn test_with_env_scope() {
        assert_that!(std::env::var("FOO")).is_err();

        let env_scope = TestEnvScope::new();

        std::env::set_var("FOO", "ENV_SCOPE");
        assert_that!(std::env::var("FOO")).is_ok();

        drop(env_scope); // Env vars should be cleaned here

        assert_that!(std::env::var("FOO")).is_err();
    }
}
