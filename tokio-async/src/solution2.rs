use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    for _ in 1..=10 {
        Command::new("echo").arg("Hello, world!").spawn()?;
    }
    Ok(())
}
