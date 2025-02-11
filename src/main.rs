mod schema;
mod types;
mod network;
mod biz;
mod error;
mod data;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    NewAccount {
        #[arg(short, long, default_value_t = String::from("./db/keystore/"))]
        keystore_dir: String,
    },

    Run {
        #[arg(short, long, default_value_t = String::from("config.toml"))]
        config: String,
    },
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
