//! Configuration module for the server.

use std::{net::SocketAddr, path::PathBuf, sync::LazyLock};

use lerpz_utils::{
    env::{get_env, get_env_parse},
    generate_config,
};

/// The main configuration struct for the server.
///
/// Lazy loaded using [`std::sync::LazyLock`] to ensure that the configuration is only loaded
/// once.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_env().unwrap());

generate_config!(
    ENV: String = get_env,
    ADDR: SocketAddr = get_env_parse,
    DATABASE_URL: String = get_env,
    REDIS_URL: String = get_env,
    OAUTH_ASSETS_PATH: PathBuf = get_env_parse
);
