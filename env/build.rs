use std::fs;

use toml::Value;

pub const BACKENDS: [&str; 1] = ["postgres"];

pub fn main() {
    let result = fs::read_to_string("../buildconf.toml");
    if let Err(error) = &result {
        println!(
            "cargo:warning={}",
            std::env::current_dir().unwrap().display()
        );
        println!("cargo:warning=cannot open build configuration file: `{error}`; the `env` crate will not compile");
        return;
    }

    let config = result.unwrap();
    let config = config.trim_end().to_string();

    let result = toml::from_str::<Value>(&config);
    if let Err(error) = &result {
        println!(
            "cargo:warning=invalid build configuration file: `{error}`; the `env` crate will not compile"
        );
        return;
    }
    let value = result.unwrap();

    let backend_value = value["cache"]["backend"].clone();
    if let Value::String(backend) = backend_value {
        if !BACKENDS.contains(&backend.as_str()) {
            println!(
                "cargo:warning=invalid backend; must be one of: {}; the `env` crate will not compile",
                BACKENDS.join(", ")
            );
            return;
        }

        println!(
            "cargo:rustc-env=ENABLE_PGSQL_CACHE_BACKEND={}",
            backend == "postgres"
        );
    } else {
        println!("cargo:warning=invalid value for `backend` value in build configuration; the `env` crate will not compile");
    }
}
