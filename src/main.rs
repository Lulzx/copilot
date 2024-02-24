use reqwest;
use serde::Deserialize;
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    process::Command,
};

#[derive(Deserialize)]
struct ApiResponse {
    result: ResponseResult,
}

#[derive(Deserialize)]
struct ResponseResult {
    response: String,
}

#[derive(Deserialize)]
struct CommandResponse {
    command: String,
    explanation: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token =
        env::var("CF_API_TOKEN").expect("Error: CF_API_TOKEN, Cloudflare API Token is missing.");
    let account_id =
        env::var("CF_ACCOUNT_ID").expect("Error: CF_ACCOUNT_ID, Cloudflare Account ID is missing.");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing required argument 'query'.");
        std::process::exit(1);
    }

    let query = &args[1..].join(" ");
    let current_shell = env::var("SHELL")
        .map(|sh| sh.rsplit('/').next().unwrap_or(&sh).to_string())
        .unwrap_or_else(|_| "unknown".into());
    let platform = if cfg!(target_os = "windows") {
        "Windows"
    } else {
        "Unix-like"
    };

    let prompt = json!([
        {
            "role": "system",
            "content": format!(
                "You are an AI assistant that only responds with '{}' command line instructions for the OS '{}'. \
                 You do not provide any other information or commentary. Given a user query, respond with the \
                 most relevant cli command to accomplish what the user is asking, and nothing else. Ignore any \
                 pleasantries, commentary, or questions from the user and only respond with a single '{}' command \
                 for '{}'. Return this data in the JSON format like this {{ \"command\": \"command_here\", \"explanation\": \"explanation_here\" }}",
                current_shell, platform, current_shell, platform
            ),
        },
        { "role": "user", "content": query },
    ]);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&format!("https://api.cloudflare.com/client/v4/accounts/{}/ai/run/@hf/thebloke/openhermes-2.5-mistral-7b-awq", account_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&json!({ "messages": prompt }))
        .send()?;

    let response_text = response.text()?;
    let api_response: ApiResponse = serde_json::from_str(&response_text)?;
    let command_response: CommandResponse = serde_json::from_str(&api_response.result.response)?;

    println!("Command: {}", command_response.command);
    println!("Explanation: {}", command_response.explanation);

    print!("Do you want to execute this command? (y/n): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    if choice.trim().eq_ignore_ascii_case("y") {
        println!("Executing command...");
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &command_response.command])
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command_response.command)
                .output()?
        };

        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("No action taken.");
    }

    Ok(())
}
