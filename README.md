# GPT CLI

A command line application for getting ChatGPT responses. The program can accept both command arguments and standard input. Standard input is placed after command arguments in the prompt allowing for seamless cli or vim integration.

## Usage

Ensure you have an environment variable named `OPENAI_API_KEY` present to make api calls.

```
[echo "additional text" |] chat [prompt]
```

## CLI Integration

It's possible to pipe the output of commands into this program to enrich the information available to ChatGPT. For example the following can print information about this repo's structure:
```bash
ls | chat explain the following files
```

## Vim Integration

It's also possible to use the tool directly from inside vim using the shell command features.

In visual mode simply select text to be included with the prompt, then press colon to bring up the command window, the format is then
```
:'<,'>!chat improve this rust function
```
to replace the function in place with the ChatGPT response, or
```
:'<,'>w !chat what does this function do
```
to only pass the text and not replace it.

**Build Status**

[![Rust](https://github.com/oranellis/gptcli/actions/workflows/rust.yml/badge.svg)](https://github.com/oranellis/gptcli/actions/workflows/rust.yml)
