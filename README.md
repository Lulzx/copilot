# CLI Copilot

This cli tool acts as an AI assistant, responding with relevant CLI commands for the given user query. It interacts with Cloudflare's API to provide command-line instructions tailored to the user's needs.

## Prerequisites

Before using this tool, ensure you have Rust installed.

## Installation

Clone this repository to your local machine:

```bash
git clone https://github.com/lulzx/copilot.git
cd copilot
```

## Usage

1. Set the required environment variables:

   ```bash
   export CF_API_TOKEN="your_cloudflare_api_token"
   export CF_ACCOUNT_ID="your_cloudflare_account_id"
   ```

2. Run the tool with your query:

   ```bash
   cargo run -- "query"
   ```

   Replace `query` with your actual query.

## Example

Suppose you want to know how to list all DNS records for a domain. You can run:

```bash
cargo run -- "how to get today's date?"
```

## Response

The tool will provide the relevant CLI command and explanation. For example:

```
Command: date +'%Y-%m-%d'
Explanation: The 'date' command with the format option '+%Y-%m-%d' will display the current date in the format 'YYYY-MM-DD'.
```

You'll then have the option to execute the provided command.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Built using Rust.

Inspired by [Akash](https://github.com/akash-joshi)'s [cli-copilot](https://github.com/akash-joshi/cli-copilot)
