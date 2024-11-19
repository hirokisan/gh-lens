use clap::*;

use crate::daterange::get_monthly_date_ranges;
use crate::github::client::{Client, PullRequestsSummary};
use anyhow::Result;
use chrono::NaiveDate;
use std::collections::HashMap;

use serde_json;

mod daterange;
mod github;

#[derive(Parser)]
#[command(name = "gh-lens")]
#[command(about = "CLI to analyze your activity on GitHub")]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "prs", about = "Analyze pull requests")]
    PullRequests {
        #[arg(long, required = true, help = "USERNAME/REPOSITORY")]
        repo: String,
        #[arg(long, required = true, help = "%Y-%m-%d")]
        start_date: String,
        #[arg(long, required = true, help = "%Y-%m-%d")]
        end_date: String,
        #[arg(long, default_value = "team")]
        scope: Scope,
        #[arg(
            long,
            help = "Specify when scope is individual e.g. USERNAME1,USERNAME2",
            value_delimiter = ','
        )]
        members: Vec<String>,
        #[arg(long, default_value = "all")]
        period: Period,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Scope {
    Team,
    Individual,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Period {
    All,
    Monthly,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = Client::new(std::env::var("GITHUB_TOKEN").unwrap());

    match cli.command {
        SubCommand::PullRequests {
            repo,
            start_date,
            end_date,
            scope,
            period,
            members,
        } => {
            let from = NaiveDate::parse_from_str(start_date.as_ref(), "%Y-%m-%d")
                .expect("start_date should be %Y-%m-%d");
            let to = NaiveDate::parse_from_str(end_date.as_ref(), "%Y-%m-%d")
                .expect("end_date should be %Y-%m-%d");
            match (scope, period) {
                (Scope::Team, Period::All) => {
                    let result = client
                        .get_pull_requests_summary(repo, start_date, end_date)
                        .await;
                    println!("{}", serde_json::to_string(&result)?);
                }
                (Scope::Team, Period::Monthly) => {
                    let drs = get_monthly_date_ranges(from, to)?;
                    let mut result: Vec<PullRequestsSummary> = Vec::with_capacity(drs.len());
                    for (start_date, end_date) in drs.iter() {
                        result.push(
                            client
                                .get_pull_requests_summary(
                                    repo.clone(),
                                    start_date.to_string(),
                                    end_date.to_string(),
                                )
                                .await,
                        );
                    }
                    println!("{}", serde_json::to_string(&result)?);
                }
                (Scope::Individual, Period::All) => {
                    if members.len() == 0 {
                        return Err(anyhow::anyhow!("members must be specified for individual"));
                    }
                    let result = client
                        .get_pull_requests_summary_on_individuals(
                            repo, start_date, end_date, members,
                        )
                        .await;
                    println!("{}", serde_json::to_string(&result)?);
                }
                (Scope::Individual, Period::Monthly) => {
                    if members.len() == 0 {
                        return Err(anyhow::anyhow!("members must be specified for individual"));
                    }
                    let drs = get_monthly_date_ranges(from, to)?;
                    let mut result: Vec<HashMap<String, PullRequestsSummary>> =
                        Vec::with_capacity(drs.len());
                    for (start_date, end_date) in drs.iter() {
                        result.push(
                            client
                                .get_pull_requests_summary_on_individuals(
                                    repo.clone(),
                                    start_date.to_string(),
                                    end_date.to_string(),
                                    members.clone(),
                                )
                                .await,
                        );
                    }
                    println!("{}", serde_json::to_string(&result)?);
                }
            };
        }
    };

    Ok(())
}
