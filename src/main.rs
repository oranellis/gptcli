use std::{env, io::{self, Read, stdout, Write}};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use openai::set_key;
use tokio;

fn remove_backtick_lines(input_text: &str) -> String {
    input_text.lines()
        .filter(|line| !line.trim_start().starts_with("```"))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn get_std_in() -> io::Result<Option<String>> {
    use std::os::unix::io::AsRawFd;
    use libc::{poll, pollfd, POLLIN};

    let stdin_fd = io::stdin().as_raw_fd();
    let mut fds = [pollfd {
        fd: stdin_fd,
        events: POLLIN,
        revents: 0,
    }];

    let retval = unsafe { poll(fds.as_mut_ptr(), 1, 0) };

    if retval > 0 {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        Ok(Some(input))
    } else if retval == 0 {
        Ok(None)
    } else {
        Err(io::Error::last_os_error())
    }
}

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("Couldn't read OPENAI_API_KEY");

    set_key(api_key);

    let system_message = "You are a helpful general assistant operating in an arch linux command line aiming to provide concise responses. When asked to provide code samples, include only the code block with no additional text.".to_string();
    let mut messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(system_message),
            name: None,
            function_call: None
        },
    ];

    stdout().flush().expect("Failed to flush stdout.");

    let args: Vec<String> = env::args().skip(1).collect();
    let mut user_prompt = args.join(" ");

    if let Ok(Some(value)) = get_std_in() {
        user_prompt.push('\n');
        user_prompt.push_str(&value);
    }

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(user_prompt),
        name: None,
        function_call: None
    });

    // Call the OpenAI API and process the response.
    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages)
        .create()
        .await
        .expect("Failed to create chat completion.");

    let response_message = chat_completion.choices.first().expect("No response from chat completion.").message.clone();
    let formatted_message = remove_backtick_lines(&response_message.content.unwrap_or_else(|| "Error getting response.".to_string()).trim());

    println!("{}", formatted_message);
}
