use anyhow::Result;

fn main() -> Result<()> {
    let environment = Environment::builder()
    .with_name("test")
    .with_log_level(LoggingLevel::Verbose)
    .build()?;

    Ok(())
}
