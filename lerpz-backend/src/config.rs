//! Configuration module for the server.

use std::sync::LazyLock;

use lerpz_utils::{env::get_env, generate_config};

/// The main configuration struct for the server.
/// 
/// Lazy loaded using `LazyLock` to ensure that the configuration is only loaded
/// once.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_env().unwrap());

generate_config!(
    ENV: String = get_env
);
