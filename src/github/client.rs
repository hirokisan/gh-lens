use graphql_client::GraphQLQuery;

use super::gql::query::{pull_requests_query, PullRequestsQuery};
use super::pull_request::PullRequest;
use super::pull_requests::PullRequests;
use super::pull_requests_summary::PullRequestsSummary;

use std::collections::HashMap;

pub struct Client {
    inner: octocrab::Octocrab,
}

impl Client {
    pub fn new(token: String) -> Self {
        let octocrab = octocrab::Octocrab::builder()
            .personal_token(token)
            .build()
            .unwrap_or_else(|e| panic!("Failed to create client: {}", e));
        Self { inner: octocrab }
    }

    async fn get_pull_requests(
        &self,
        repo: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<PullRequests, anyhow::Error> {
        let mut result = PullRequests::new();

        let offset = 10;
        let query = format!("repo:{repo} is:pull-request created:{start_date}..{end_date}");
        let mut variables = pull_requests_query::Variables {
            first: offset,
            query: query.to_string(),
            threshold: 50,
            after: None,
        };

        loop {
            let response: octocrab::Result<
                graphql_client::Response<pull_requests_query::ResponseData>,
            > = self
                .inner
                .graphql(&PullRequestsQuery::build_query(variables.clone()))
                .await;

            match response {
                Ok(res) => {
                    let prs = &res.data.as_ref().unwrap().search;
                    let has_next_page = prs.page_info.has_next_page;
                    let end_cursor = prs.page_info.end_cursor.clone();

                    for item in prs.nodes.as_ref().unwrap().iter().flatten() {
                        match item {
                            pull_requests_query::PullRequestsQuerySearchNodes::PullRequest(pr) => {
                                result.add(PullRequest::new(pr.clone()))
                            }
                            _ => continue,
                        };
                    }

                    if !has_next_page {
                        break;
                    }
                    variables.after.clone_from(&end_cursor);
                }
                Err(err) => return Err(anyhow::anyhow!(err)),
            }
        }

        Ok(result)
    }

    pub async fn get_pull_requests_summary(
        &self,
        repo: String,
        start_date: String,
        end_date: String,
    ) -> Result<PullRequestsSummary, anyhow::Error> {
        let pull_requests = self
            .get_pull_requests(&repo, &start_date, &end_date)
            .await?;

        Ok(PullRequestsSummary::new(
            start_date.clone(),
            end_date.clone(),
            &pull_requests,
        ))
    }

    pub async fn get_pull_requests_summary_on_individuals(
        &self,
        repo: String,
        start_date: String,
        end_date: String,
        individuals: Vec<String>,
    ) -> Result<HashMap<String, PullRequestsSummary>, anyhow::Error> {
        let pull_requests = self
            .get_pull_requests(&repo, &start_date, &end_date)
            .await?;

        let mut summaries: HashMap<String, PullRequestsSummary> = HashMap::new();

        for individual in individuals.iter() {
            summaries
                .entry(individual.clone())
                .or_insert(PullRequestsSummary::new_with_by(
                    start_date.clone(),
                    end_date.clone(),
                    &pull_requests,
                    individual,
                ));
        }

        Ok(summaries)
    }
}
