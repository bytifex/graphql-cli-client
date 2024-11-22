mod cli;

use clap::Parser;
use cli::Cli;
use graphql_cli_client::{execute, load_variables};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let variables = load_variables(cli.variables_from_json, cli.variables)?;
    let headers = cli.headers.into_iter().collect();

    execute(
        cli.server_endpoint,
        headers,
        cli.query_path,
        cli.operation_name,
        variables,
        |response| {
            println!("{}", serde_json::to_string_pretty(&response)?);

            Ok(())
        },
        cli.try_reconnect_duration.map(|duration| duration.into()),
    )
    .await
}
