use tokio::net::TcpStream;
use tokio::prelude::*;

#[macro_use]
extern crate log;

/// Bind to IP and port and starts listening
///
/// It read socket and return same data as response
///
/// # Bash test
///
/// `nc localhost 8080` -> it will open connection -> possible to sand any data.alloc
///
///  1. When connection is estabilished -> "Connection estabilished" is printed
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug,tokioclient=trace");
    env_logger::init();

    let matches = clap::App::new("Tokio Client")
        .version("1.0")
        .author("Filip Bucek <fbucek@invloop.cz>")
        .about("Sends data to specific IP address")
        .arg(
            clap::Arg::with_name("DATA")
                .help("Data to send")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("ip")
                .short("i")
                .help("IP address")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .help("port")
                .takes_value(true),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8080");
    let data = matches.value_of("DATA").unwrap_or("");

    let address = format!("{}:{}", ip, port);
    info!("Running client on: {}", &address);

    let mut stream = TcpStream::connect(&address).await?;

    // trace!("Keep alive: {:?}", stream.keepalive()?);
    trace!("Connecting to server");
    stream.write_all(data.as_bytes()).await.unwrap();

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    trace!(
        "returned data -> size: {} data: {}",
        n,
        String::from_utf8_lossy(&buf[0..n])
    );
    Ok(())
}
