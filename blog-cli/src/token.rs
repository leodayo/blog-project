use anyhow::Context;
use std::fs;
use std::path::PathBuf;

fn token_file_path() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".blog_token")
    } else {
        PathBuf::from(".blog_token")
    }
}

pub fn load_token() -> anyhow::Result<Option<String>> {
    let path = token_file_path();
    if !path.exists() {
        return Ok(None);
    }

    let token = fs::read_to_string(path)
        .context("Failed to read token file")?
        .trim()
        .to_string();
    Ok(Some(token))
}

pub fn save_token(token: &str) -> anyhow::Result<()> {
    let path = token_file_path();
    fs::write(path, token).context("Failed to write token file")?;
    Ok(())
}
