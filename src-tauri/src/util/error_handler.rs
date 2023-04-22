use reqwest::Error;

pub fn error_handler(err: Error) -> String{
    format!("Request Error: {}", err)
}