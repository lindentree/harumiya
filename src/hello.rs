use std::env;

pub fn hello() {
    let api_endpoint = env::var("API_ENDPOINT2").unwrap();

    println!("{:}", api_endpoint);
}
