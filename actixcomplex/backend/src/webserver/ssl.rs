use std::env;


/// Return correct path to certificat and private_key
pub fn ssl_certificates() -> Result<(String, String), std::io::Error> {
    // Enabled SSL
    let out_dir = env::var("PWD").expect("not possible to get current working directory");
    let certificate = format!("{}/actixcomplex/keys/actixcomplex.crt", &out_dir);
    // let custom_error = Error::new(ErrorKind::Other, "oh no!");
    if !std::path::Path::new(&certificate).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("certificate {} not found", &certificate),
        ));
    }
    let private_key = format!("{}/actixcomplex/keys/actixcomplex.key", &out_dir);
    if !std::path::Path::new(&private_key).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("certificate {} not found", &private_key),
        ));
    }
    Ok((certificate, private_key))
}
