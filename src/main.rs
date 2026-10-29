use anyhow::Result;
use clap::{Parser, Subcommand};
use siteforge::{
    crawl::{Config, audit},
    network::Network,
    report::{Format, render},
};
use std::{
    io::{self, Write},
    time::Duration,
};
use url::Url;
#[derive(Parser)]
#[command(version, about = "Respectful technical SEO and website auditing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Audit {
        url: Url,
        #[arg(long, default_value="100", value_parser=clap::value_parser!(u32).range(1..=100000))]
        max_pages: u32,
        #[arg(long, default_value="4", value_parser=clap::value_parser!(u32).range(1..=64))]
        concurrency: u32,
        #[arg(long, default_value="500", value_parser=clap::value_parser!(u64).range(0..=86400000))]
        delay_ms: u64,
        #[arg(long, default_value="15", value_parser=clap::value_parser!(u64).range(1..=300))]
        timeout_secs: u64,
        /// Explicitly authorize access to private, loopback, and internal destinations.
        #[arg(long)]
        allow_private: bool,
        #[arg(long, value_enum, default_value = "terminal")]
        format: Format,
    },
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .init();
    let Commands::Audit {
        url,
        max_pages,
        concurrency,
        delay_ms,
        timeout_secs,
        allow_private,
        format,
    } = Cli::parse().command;
    tracing::info!("starting audit");
    let report = audit(
        url,
        Config {
            max_pages: max_pages as usize,
            concurrency: concurrency as usize,
            delay: Duration::from_millis(delay_ms),
            network: Network {
                allow_private,
                timeout: Duration::from_secs(timeout_secs),
                max_bytes: 2 * 1024 * 1024,
            },
        },
    )
    .await?;
    let output = render(&report, format)?;
    match writeln!(io::stdout().lock(), "{output}") {
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => Ok(result?),
    }
}
