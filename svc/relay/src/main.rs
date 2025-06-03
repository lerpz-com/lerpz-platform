use crate::config::CONFIG;

mod config;

fn main() {
    println!("Environment: {}", CONFIG.ENV);
}
