# Temporary Environment Variables

[![Version](https://img.shields.io/crates/v/temp_env_vars.svg)](https://crates.io/crates/temp_env_vars)
[![Downloads](https://img.shields.io/crates/d/temp_env_vars)](https://crates.io/crates/temp_env_vars)
[![MIT license](https://img.shields.io/crates/l/temp_env_vars.svg)](./LICENSE)
[![Build Status](https://github.com/guenhter/temp_env_vars/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/guenhter/temp_env_vars/actions)
[![MSRV: 1.80.0](https://flat.badgen.net/badge/MSRV/1.80.0/purple)](https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html)


`temp_env_vars` allows to to manipulate enviornment variables during a test and reset all changes when the test is done.

[!WARNING]

The software currently in the starting phase and will change


## Installation

```bash
cargo add temp_env_vars
```


## Usage

`temp_env_vars` can be used in two different forms:

1. as macro `#[temp_env_vars]`
2. with `TestEnvScope::new()`


### Use as macro

`#[temp_env_vars]` is the preferred way to use the `temp_env_vars` crate.
Every change to envionrment variables within the execution of the test function
will be reset after the test has ended.

If more tests are used with this macro, those tests will be executed sequentially to avoid an enviornment variable mixup.

```rust
#[test]
#[temp_env_vars]
fn test_some() {
    std::env::set_var("FOO", "BAR");
    assert_eq!(std::env::var("FOO").unwrap(), "BAR");

    // Env vars get reset when test is done
}
```


### Use with TestEnvScope

If resetting the environment variables after the test execution is not sufficient, but the reset must happen somewhere within the test, the `TestEnvScope` can be used to have better control.

Whenever the created `TestEnvScope` goes out of scope, all env vars are reset.

```rust
#[test]
#[serial] // Use "serial" (external crate), as parallel tests could mix up envs
fn test_some() {
    let _env_scope = TestEnvScope::new();
    std::env::set_var("FOO", "BAR");
    assert_eq!(std::env::var("FOO").unwrap(), "BAR");

    // After "_env_scope" goes out of scope, all vars are restored
}

#[test]
#[serial] // Use "serial" (external crate), as parallel tests could mix up envs
fn test_bar() {
    let _env_scope = TestEnvScope::new();
    std::env::set_var("FOO", "BAR");
    assert_eq!(std::env::var("FOO").unwrap(), "BAR");

    drop(_env_scope); // After "_env_scope" goes out of scope, all vars are restored


    // "FOO" is not longer set here.
}
```


## Contribution

Contribution are always welcome in any form.

You acknowledge and agree that the owner reserve the right to change the license of the Work, including but not limited to all Contributions previously submitted by You, at any time without the need for approval from You or any other contributor.

## License

This project is licensed under the https://github.com/guenhter/temp_env_vars/blob/main/LICENSE[MIT license].
