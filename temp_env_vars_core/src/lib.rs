use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

// Makes the mutex available for the `temp_env_vars` macro. Unfortunately, Macro traits cannot
// export other types than macros, so this is the least bad place to export this then.
pub static TEMP_ENV_VAR_MACRO_MUTEX: LazyLock<Arc<Mutex<()>>> = LazyLock::new(Arc::default);

pub struct TestEnvScope {
    original_vars: HashMap<String, String>,
}

impl TestEnvScope {
    pub fn new() -> TestEnvScope {
        TestEnvScope {
            original_vars: std::env::vars().collect(),
        }
    }
}

impl Drop for TestEnvScope {
    fn drop(&mut self) {
        let mut after: HashMap<String, String> = std::env::vars().collect();

        self.original_vars.keys().for_each(|key| {
            after.remove(key);
        });
        after.keys().for_each(|key| {
            std::env::remove_var(key);
        });
        self.original_vars.iter().for_each(|(k, v)| {
            std::env::set_var(k, v);
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use assertor::{assert_that, EqualityAssertion, ResultAssertion};
    use serial_test::serial;

    use super::TestEnvScope;

    #[test]
    #[serial]
    fn test_nothing_is_changed() {
        let original: HashMap<String, String> = std::env::vars().collect();

        {
            let _env_scope = TestEnvScope::new();
        }

        let after: HashMap<String, String> = std::env::vars().collect();
        assert_that!(after).is_equal_to(original);
    }

    #[test]
    #[serial]
    fn test_new_vars_are_removed() {
        std::env::remove_var("FOO");
        let original: HashMap<String, String> = std::env::vars().collect();

        {
            let _env_scope = TestEnvScope::new();
            std::env::set_var("FOO", "BAR1");
        }

        let after: HashMap<String, String> = std::env::vars().collect();
        assert_that!(std::env::var("FOO")).is_err();
        assert_that!(after).is_equal_to(original);
    }

    #[test]
    #[serial]
    fn test_changed_vars_are_reset() {
        std::env::set_var("FOO", "BAR2");
        let original: HashMap<String, String> = std::env::vars().collect();

        {
            let _env_scope = TestEnvScope::new();
            std::env::set_var("FOO", "123");
        }

        let after: HashMap<String, String> = std::env::vars().collect();
        assert_that!(std::env::var("FOO")).has_ok("BAR2".to_string());
        assert_that!(after).is_equal_to(original);
    }

    #[test]
    #[serial]
    fn test_env_vars_are_restored() {
        std::env::set_var("FOO", "BAR3");
        let original: HashMap<String, String> = std::env::vars().collect();

        {
            let _env_scope = TestEnvScope::new();
            std::env::remove_var("FOO");
        }

        let after: HashMap<String, String> = std::env::vars().collect();
        assert_that!(std::env::var("FOO")).has_ok("BAR3".to_string());
        assert_that!(after).is_equal_to(original);
    }

    #[test]
    #[serial]
    fn test_two_scopes_active_at_same_time() {
        std::env::remove_var("FOO");

        {
            let _env_scope_1 = TestEnvScope::new();
            let _env_scope_2 = TestEnvScope::new();

            std::env::set_var("FOO", "BAR4");
            assert_that!(std::env::var("FOO")).is_ok();
        }

        assert_that!(std::env::var("FOO")).is_err();
    }

    #[test]
    #[serial]
    fn test_sequential_test_scopes() {
        std::env::remove_var("FOO");

        {
            let _env_scope = TestEnvScope::new();

            std::env::set_var("FOO", "BAR5");
            assert_that!(std::env::var("FOO")).is_ok();
        }
        assert_that!(std::env::var("FOO")).is_err();

        {
            let _env_scope = TestEnvScope::new();

            std::env::set_var("FOO", "BAR6");
            assert_that!(std::env::var("FOO")).is_ok();
        }
        assert_that!(std::env::var("FOO")).is_err();
    }
}
