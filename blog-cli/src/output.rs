use anyhow::Context;

pub fn print_json(value: &impl serde::Serialize) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(value).context("Failed to serialize to JSON")?;
    println!("{}", json);
    Ok(())
}
