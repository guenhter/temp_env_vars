//! # temp_env_vars
//!
//! `temp_env_vars` allows to to manipulate enviornment variables during a test and reset
//!  all changes when the test is done.
//!
//! ## Usage
//!
//! `temp_env_vars` can be used in two different forms:
//!
//! 1. as macro `#[temp_env_vars]`
//! 2. with `TestEnvScope::new()`
//!
//! ### Use as macro
//!
//! `#[temp_env_vars]` is the preferred way to use the `temp_env_vars` crate.
//! Every change to envionrment variables within the execution of the test function
//! will be reset after the test has ended.
//!
//! If more tests are used with this macro, those tests will be executed sequentially
//! to avoid an enviornment variable mixup.
//!
//! ```rust
//! #[test]
//! #[temp_env_vars]
//! fn test_some() {
//!     std::env::set_var("FOO", "BAR");
//!     assert_eq!(std::env::var("FOO").unwrap(), "BAR");
//!
//!     // Env vars get reset when test is done
//! }
//! ```
//!
//! ### Use with TestEnvScope
//!
//!
//! If resetting the environment variables after the test execution is not sufficient,
//! but the reset must happen somewhere within the test, the `TestEnvScope` can be
//! used to have better control.
//!
//! Whenever the created `TestEnvScope` goes out of scope, all env vars are reset.
//!
//! ```rust
//! #[test]
//! #[serial] // Advices to use serial, alse parallel tests could mix up envs
//! fn test_some() {
//!     let _env_scope = TestEnvScope::new();
//!     std::env::set_var("FOO", "BAR");
//!     assert_eq!(std::env::var("FOO").unwrap(), "BAR");
//!
//!     // After "_env_scope" goes out of scope, all vars are restored
//! }
//!
//! #[test]
//! #[serial] // Advices to use serial, alse parallel tests could mix up envs
//! fn test_bar() {
//!     let _env_scope = TestEnvScope::new();
//!     std::env::set_var("FOO", "BAR");
//!     assert_eq!(std::env::var("FOO").unwrap(), "BAR");
//!
//!     drop(_env_scope); // After "_env_scope" goes out of scope, all vars are restored
//!
//!
//!     // "FOO" is not longer set here.
//! }
//! ```
pub use temp_env_vars_core::TestEnvScope;
pub use temp_env_vars_macro::temp_env_vars;

#[cfg(test)]
mod test {
    use super::temp_env_vars;

    use assertor::{assert_that, ResultAssertion};
    use serial_test::serial;
    use temp_env_vars_core::TestEnvScope;

    #[test]
    #[temp_env_vars]
    #[serial] // Advised to use serial_test if other env-tests are not used with annotated with "temp_env_vars"
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
