use core::time;
use std::thread::sleep;

use assertor::{assert_that, ResultAssertion};
use temp_env_vars::temp_env_vars;

#[test]
#[temp_env_vars]
fn test_concurrency_between_two_tests_work_a() {
    assert_that!(std::env::var("FOO")).is_err();
    std::env::set_var("FOO", "1");

    // If the other test is not blocked, this should give the other test it enough time
    // to override the "FOO" env var -> This test will then fail in the last assert
    sleep(time::Duration::from_millis(100));

    assert_that!(std::env::var("FOO")).has_ok("1".to_string());
}

#[test]
#[temp_env_vars]
fn test_concurrency_between_two_tests_work_b() {
    assert_that!(std::env::var("FOO")).is_err();
    std::env::set_var("FOO", "2");

    // If the other test is not blocked, this should give the other test it enough time
    // to override the "FOO" env var -> This test will then fail in the last assert
    sleep(time::Duration::from_millis(100));

    assert_that!(std::env::var("FOO")).has_ok("2".to_string());
}
