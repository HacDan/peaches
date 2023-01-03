use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::Deserialize;
use std::{collections::HashMap, env, fs};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub projects: HashMap<String, Project>,
    pub dotfiles: Dotfiles,
    pub ssh: HashMap<String, SSH>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub session_name: String,
    pub directory: String,
    pub max_depth: u8,
    pub min_depth: u8,
    pub include_hidden: bool,
}

#[derive(Debug, Deserialize)]
pub struct Dotfiles {
    pub repo: String,
    pub location: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct SSH {
    pub host: String,
    pub auth_method: String,
    pub password: String,
}

impl SSH {
    pub fn decrypt(encrypted: &str) -> String {
        if !encrypted.contains("crypt:") {
            return encrypted.to_string();
        }

        if env::var("PEACHES_KEY").is_ok() {
            let mc = new_magic_crypt!(env::var("PEACHES_KEY").ok().unwrap(), 256);
            return mc.decrypt_base64_to_string(encrypted).unwrap();
        } else {
            panic!("Please set encryption key as 'PEACHES_KEY' in environment variables.");
        }
    }

    pub fn encrypt(raw: &str) -> String {
        if env::var("PEACHES_KEY").is_ok() {
            let mc = new_magic_crypt!(env::var("PEACHES_KEY").ok().unwrap(), 256);
            return mc.encrypt_str_to_base64(raw);
        } else {
            panic!("Please set encryption key as 'PEACHES_KEY' in environment variables.");
        }
    }
}

fn get_peaches_path() -> String {
    let mut peaches_path: String = env::var("HOME").ok().unwrap();
    peaches_path.push_str("/.peaches");
    if env::var("PEACHES_PATH").is_ok() {
        peaches_path = env::var("PEACHES_PATH").ok().unwrap();
    }

    return peaches_path;
}

pub fn generate_config() {
    const DEFAULT_CONFIG: &str = r#"
[projects]
    [projects.default]
    session_name = "default"
    directory = "/"
    min_depth = 1
    max_depth = 1
    include_hidden = false

[dotfiles]
repo = ""
location = ""
command = ""

[ssh]

    [ssh.default]
    host = "127.0.0.1"
    auth_method = "password"
    encrypted = "crypt:"
"#;

    let peaches_path = get_peaches_path();
    fs::write(peaches_path, DEFAULT_CONFIG).expect("Unable to write file");
}

pub fn load_config() -> Config {
    let peaches_path = get_peaches_path();

    let config_string: String = fs::read_to_string(peaches_path).ok().unwrap();

    let config: Config = toml::from_str(&config_string).unwrap();
    return config;
}
