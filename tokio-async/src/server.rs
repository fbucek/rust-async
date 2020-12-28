use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

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
    std::env::set_var("RUST_LOG", "debug,tokio-=trace");
    env_logger::init();

    let matches = clap::App::new("Tokio Client")
        .version("1.0")
        .author("Filip Bucek <fbucek@invloop.cz>")
        .about("Sends data to specific IP address")
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
    let address = format!("{}:{}", ip, port);

    let listener = TcpListener::bind(&address).await?;

    match listener.accept().await {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }

    loop {
        let (mut socket, addr) = listener.accept().await?;

        info!("Connection estabilished with: {:?}", addr);

        // Spawn async in ThreadPool ( default executor )
        tokio::spawn(async move {
            // In a loop, read data from the socket and write the data back.
            loop {
                let mut buf = [0; 1024];
                let bytes_read = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(bytes_read) if bytes_read == 0 => return,
                    Ok(bytes_read) => bytes_read,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                info!("data: {}", String::from_utf8_lossy(&buf));

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..bytes_read]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
