use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};


#[derive(Serialize)]
struct ChatRequest{
    model: String,
    prompt: String,
    stream:bool,
}

#[derive(Deserialize)]
struct ChatResponse {
    response: String,
    #[allow(dead_code)]
    done:bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = Client::new();
    let mut conversation = String::new();

    println!("Intelligent Expense Tracker: AI Mode Enabled. \n You are now talking to an Agent");

    loop {
        println!("you: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falied to read input");
        let input = input.trim().to_string();

        if input.starts_with("quit") {
            break;
        }

        conversation.push_str(&format!("User: {}\n Assistant:", input));

        let payload = ChatRequest{
            model:"gemma3:12b".into(),
            prompt:conversation.into(),
            stream:true,
        };

        println!("Agnet: ");
        io::stdout().flush()?;

        let stream = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await?
        .bytes_stream();


        let mut full_response = String::new();
        
    }


    Ok(())
}