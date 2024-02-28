# GPT CLI

GPT CLI is a versatile command-line tool that enables ChatGPT integration into CLI and Vim environments. It facilitates executing ChatGPT queries from anywhere cli programs can be run, making it easier to analyze or modify code, generate documentation, and more.

## Features

- **Environment Variables**: Leverages `OPENAI_API_KEY` for secure API configuration and allows setting `OPENAI_DEFAULT_MODEL` to choose the desired language model.
- **CLI and Vim Integration**: Enhances command-line workflows and Vim editing with direct ChatGPT integration for insights and content generation.

## Getting Started

### Prerequisites

Set the `OPENAI_API_KEY` environment variable to authenticate API requests. Optionally, set `OPENAI_DEFAULT_MODEL` to specify the default language model.

### Installation

For Unix-like environments, you can use the `install.sh` script located in the root of this project:

```bash
./install.sh
```

### Usage

#### Command Line

Direct CLI usage:

```bash
chat [prompt]
```

Piping command output to ChatGPT:

```bash
ls | chat explain the following files
```

#### Vim Integration

In Visual Mode, you can:

Replace selected text with ChatGPT suggestions:

```
:'<,'>!chat improve this rust function
```

Query ChatGPT about selected text without replacement:

```
:'<,'>w !chat what does this function do
```

## Stay Informed

For the latest build information, see the GitHub Actions build status:

[![Rust](https://github.com/oranellis/gptcli/actions/workflows/rust.yml/badge.svg)](https://github.com/oranellis/gptcli/actions/workflows/rust.yml)

Leverage GPT CLI to streamline your CLI and Vim interactions through ChatGPT's capabilities.

