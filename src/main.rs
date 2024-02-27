use std::{io::{stdout, Write}, env, process::exit};

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

fn remove_backtick_lines(input_text: &str) -> String {
    input_text.lines()
        .filter(|line| !line.trim_start().starts_with("```"))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn get_std_in() -> Option<String> {

    extern crate libc;

    use libc::{poll, pollfd, POLLIN};
    use std::io::{self, Read};
    use std::os::unix::io::AsRawFd;

    let stdin_fd = io::stdin().as_raw_fd();
    let mut fds = [pollfd {
        fd: stdin_fd,
        events: POLLIN,
        revents: 0,
    }];

    let retval = unsafe { poll(fds.as_mut_ptr(), 1, 0) };

    if retval > 0 {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Some(input)
    } else if retval == 0 {
        None
    } else {
        None
    }
}

#[tokio::main]
async fn main() {

    let key = "OPENAI_API_KEY"; // Replace this with the name of your environment variable
    match env::var(key) {
        Ok(value) => set_key(value),
        Err(e) => {
            println!("Couldn't read {}: {}", key, e);
            exit(1);
        }
    }

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You are a helpful general assistant operating in an arch linux command line aiming to provide concise responses. When asked to provide code samples, include only the code block with no additional text.".to_string()),
        name: None,
        function_call: None,
    }];

    stdout().flush().unwrap();

    let collect: Vec<String> = env::args().skip(1).collect();
    let mut user_promt = collect.join(" ");

    if let Some(value) = get_std_in() {
        user_promt.push_str("\n");
        user_promt.push_str(value.as_str());
    }

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(user_promt),
        name: None,
        function_call: None,
    });

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await
        .unwrap();
    let response_message = chat_completion.choices.first().unwrap().message.clone();

    let formatted_message = remove_backtick_lines(&response_message.content.clone().unwrap_or("Error getting response.".to_string()).trim());

    println!(
        "{}",
        formatted_message
    );
}
