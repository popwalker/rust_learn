use anyhow::{Result};
use serde_yaml::Value;
use tokio::{fs, try_join};


#[tokio::main]
async fn main() -> Result<()> {
    let f1 = fs::read_to_string("./Cargo.toml");
    let f2 = fs::read_to_string("./Cargo.lock");

    let (content1, content2) = try_join!(f1, f2)?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    let f3 = fs::write("./bak_Cargo.toml", &yaml1);
    let f4 = fs::write("./bak_Cargo.lock", &yaml2);

    try_join!(f3, f4)?;

    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}