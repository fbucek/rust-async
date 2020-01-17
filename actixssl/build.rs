use std::*;
use std::path::Path;
use std::fs;

use std::process::Command;

fn main() {
    let cur_dir = std::env::current_dir().expect("not possible to get current dir");
    println!("Current dir: {:?}", cur_dir);
    let dest_path = Path::new(&cur_dir).join("keys");
    fs::create_dir_all(&dest_path).expect("not possible to create directory");

    // this is from official tutorial
    let args = "openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost";
    let args: Vec<&str> = args.split(' ').collect();

    match Command::new(&args.first().unwrap())
        .current_dir(&dest_path)
        .args(&args[1..])
        //    .arg(&format!("{}/hello.o", out_dir))
        .status() {
            Ok(status) => println!("Status is: {}", status),
            Err(err) => eprintln!("Error is: {:?}", err)
        }

    match Command::new("build.sh")
        .status() {
            Ok(status) => println!("Status is: {}", status),
            Err(err) => eprintln!("Error is: {:?}", err)
        }

    println!("Build finished");
}