#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

use std::io::prelude::*; // .write

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let body: String = reqwest::get("https://enter-address")
    .await?
    .text()
    .await?;

    let document = Document::from(body.as_str());

    let path = std::env::current_dir()?;

    let vec = document
        .find(Name("a"))
        .filter(|node| node.attr("data-url").is_some())
        .map(|node| { // map data from <a href data-url=... data-nazev=...>
            let mut name = node.attr("data-nazev").expect("Not possible to get attribute data-nazev").to_string();
            name.push_str(".jpg");
            let raw_url = node.attr("data-url").unwrap();
            let url = &raw_url[2..];
            let url = format!("https://{}", &url);
            let filepath = format!("{}/data/{}", &path.to_str().unwrap(), &name);
            (url, filepath)
        });

    for pair in vec {
        let url = pair.0;
        let path = pair.1;

        println!("{}, {}", &url, &path);

        let data = reqwest::get(&url)
            .await?
            .bytes()
            .await?;
        
        let mut file = std::fs::File::create(&path).expect("Not possible to create file");
        match file.write(&data) {
            Ok(size) => println!("Bytes written: {}", size),
            Err(err) => eprintln!("Error writing file: {:?}", err)
        }
    }

//     let data = reqwest::get(url)
//     .await
//     .unwrap()
//     .bytes()
//     .await
//     .unwrap();

// 










    //     // .find(Name("a")).collect();

    // for item in document.find(Name("a")) {

    //     if item.attr("data-url").is_some() {

    //         println!("{}", item.attr("data-url").unwrap());
    //     }
    // }
        // .filter(|n| n.attr("href"))
        // .for_each(|x| println!("{}", x));

    Ok(())
}