use std::path::PathBuf;

use clap::Parser;
use graphql_cli_client::cli_types::{HttpHeaderParser, KeyValueParser};
use reqwest::header::{HeaderName, HeaderValue};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short('e'),
        long("server-endpoint"),
        help("Endpoint where the server accepts the connections (e.g., http://localhost:8000/api/graphql)"),
    )]
    pub server_endpoint: String,

    #[arg(
        short('q'),
        long("query-path"),
        help("Path of the query that has to be executed")
    )]
    pub query_path: PathBuf,

    #[arg(
        short('o'),
        long("operation-name"),
        help("Name of the operation that has to be executed")
    )]
    pub operation_name: Option<String>,

    #[arg(
        long("variables-from-json"),
        help("Json file containing variables to be sent to the server")
    )]
    pub variables_from_json: Option<PathBuf>,

    #[arg(
        short('v'),
        long("variable"),
        value_parser(KeyValueParser),
        help("Variable to be sent to the server")
    )]
    pub variables: Vec<(String, serde_json::Value)>,

    #[arg(
        long("http-header"),
        value_parser(HttpHeaderParser),
        help("HTTP header to be sent to the server")
    )]
    pub headers: Vec<(HeaderName, HeaderValue)>,

    #[arg(
        short('r'),
        long("try-reconnect-duration"),
        help("When in subscription mode, the client will try to reconnect to the server if there is no connection (e.g., 500ms"),
    )]
    pub try_reconnect_duration: Option<humantime::Duration>,
}
