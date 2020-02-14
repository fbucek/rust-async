use std::env;
use std::fs;
use std::path;

use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn main() -> Result<()> {
    // Prepare path
    // let out_dir = env::var("PWD")?;
    // let pkg_name = env::var("CARGO_PKG_NAME")?;
    // let mut target = path::PathBuf::from(&out_dir);
    // if !out_dir.ends_with(&pkg_name) {
    //     target.push(&pkg_name);
    // }
    // target.push("keys");

    // // Have to create dir "keys"
    // match std::fs::create_dir(&target) {
    //     Err(err) => println!("Not possible to create dir {:?}", err.kind()),
    //     Ok(_) => {},
    // }

    // let config = "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth".to_string();
    // let mut config_path = target.clone();
    // config_path.push("config");
    // let mut config_file = fs::File::create(config_path)?;
    // config_file.write_all(config.as_bytes())?;

    Ok(())
}

// macro_rules! err {
//     ($version:expr, $date:expr, $msg:expr) => (
//         eprintln!("{} {}", Red.paint("Error:").bold(), Paint::new($msg).bold());
//         eprintln!("Installed version: {}", Yellow.paint(format!("{} ({})", $version, $date)));
//         eprintln!("Minimum required:  {}", Yellow.paint(format!("{} ({})", MIN_VERSION, MIN_DATE)));
//     )
// }
