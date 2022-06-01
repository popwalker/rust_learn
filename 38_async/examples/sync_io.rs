use anyhow::{Result};
use serde_yaml::Value;
use std::fs;

fn main() -> Result<()>{
    let content1 = fs::read_to_string("./Cargo.toml")?;
    let content2 = fs::read_to_string("./Cargo.lock")?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    fs::write("./bak_Cargo.yml", &yaml1)?;
    fs::write("./bak_Cargo.lock", &yaml2)?;

    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}