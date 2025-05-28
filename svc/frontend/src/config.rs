//! Configuration module for the server.

use std::sync::LazyLock;

use lerpz_utils::{env::get_env, generate_config};

/// The main config for the server.
/// 
/// Loaded using [`LazyLock`] to ensure that the config is only loaded once.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_env().unwrap());

generate_config!(
    ENV: String = get_env
);
