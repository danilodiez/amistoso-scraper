use reqwest::blocking::Client;
use tokio::time::interval;
use std::error::Error;
use std::{time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    const TICKET_URL: &str = "https://www.deportick.com/";
    println!("------ Starting scraping -----");
    
    let mut interval = interval(Duration::from_secs(3));

    loop {
        
        let client = Client::new();
        let response = client.get(TICKET_URL).send()?;
        let body_first = response.text()?;
        interval.tick().await;
        let response_newer = client.get(TICKET_URL).send()?;
    
        let body_newer = response_newer.text()?;
        let mut answer = "SI";
        if (body_first == body_newer) {
            answer = "SI"
        } else {
            answer = "\x1b[93mNOOOOOO\x1b[0m"
        }
        println!("La UI sigue igual???? {}", answer);
    }
    return Ok(())
}
