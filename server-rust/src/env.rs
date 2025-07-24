extern crate dotenv;
use dotenv::dotenv;
use std::env;

pub fn get_env_var(var: &str) -> String {
    dotenv().ok();

    let env_var = env::var(var);
    if let Ok(result) = env_var {
        result
    } else {
        String::from("Env var not found")
    }
}
