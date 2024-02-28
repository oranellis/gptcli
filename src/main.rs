use std::{env, io::{self, Read, stdout, Write}};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use openai::set_key;
use tokio;

fn remove_backtick_lines(input_text: &str) -> String {
    let mut lines = input_text.lines().collect::<Vec<&str>>();
    if lines.first().map_or(false, |line| line.trim().starts_with("```")) {
        lines.remove(0);
    }
    if lines.last().map_or(false, |line| line.trim().starts_with("```")) {
        lines.pop();
    }
    lines.join("\n")
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

    // Check for OPENAI_DEFAULT_MODEL environment variable to set the model dynamically
    let model = env::var("OPENAI_DEFAULT_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
    println!("Model: {}", &model);

    let system_message = "You are a helpful general assistant being run from an arch linux command line. When asked a question please provide concise responses. When providing code samples, include only the code block with no additional text. Additional context for the user question may be provided in triple quotes.".to_string();
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
        user_prompt.push_str("\n\"\"\"\n");
        user_prompt.push_str(&value);
        user_prompt.push_str("\n\"\"\"");
    }

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(user_prompt),
        name: None,
        function_call: None
    });

    // Call the OpenAI API with the dynamic model selection
    let chat_completion = ChatCompletion::builder(&model, messages)
        .create()
        .await
        .expect("Failed to create chat completion.");
    let response_message = chat_completion.choices.first().expect("No response from chat completion.").message.clone();
    let formatted_message = remove_backtick_lines(&response_message.content.unwrap_or_else(|| "Error getting response.".to_string()).trim());

    println!("{}", formatted_message);
}
