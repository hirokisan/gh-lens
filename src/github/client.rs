use graphql_client::GraphQLQuery;

use crate::github::gql::query::{pull_requests_query, PullRequestsQuery};
use crate::github::pull_requests_summary::{PullRequestSummary, PullRequestsSummary};

use std::collections::HashMap;

pub struct Client {
    inner: octocrab::Octocrab,
}

struct PullRequests {
    inner: Vec<PullRequest>,
}

struct PullRequest {
    inner: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest,
}

impl PullRequest {
    fn new(inner: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest) -> Self {
        Self { inner }
    }
}

impl PullRequests {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn add(&mut self, item: PullRequest) {
        self.inner.push(item);
    }
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
        let mut summary = PullRequestsSummary::new(start_date.clone(), end_date.clone());

        let pull_requests = self
            .get_pull_requests(&repo, &start_date, &end_date)
            .await?;

        summary.prs_count = pull_requests.inner.len() as i64;
        for pull_request in pull_requests.inner.iter() {
            let pr = &pull_request.inner;

            let author = match pr.author.as_ref() {
                Some(author) => author.login.clone(),
                None => "no-author".to_string(),
            };

            let reviews = pr.reviews.as_ref().unwrap().nodes.as_ref();
            let comments = pr.comments.nodes.as_ref();
            let first_review = reviews.unwrap().iter().find(|review| {
                if review.as_ref().unwrap().author.as_ref().is_none() {
                    return false;
                }
                review.as_ref().unwrap().author.as_ref().unwrap().login != author.as_ref()
            });
            let first_reviewed_at =
                first_review.map(|review| review.as_ref().unwrap().created_at.clone());
            let first_comment = comments.unwrap().iter().find(|comment| {
                if comment.as_ref().unwrap().author.as_ref().is_none() {
                    return false;
                }
                comment.as_ref().unwrap().author.as_ref().unwrap().login != author.as_ref()
            });
            let first_commented_at =
                first_comment.map(|comment| comment.as_ref().unwrap().created_at.clone());
            let first_contacted_at = match (first_reviewed_at, first_commented_at) {
                (None, Some(commented_at)) => Some(commented_at),
                (Some(reviewed_at), None) => Some(reviewed_at),
                (Some(reviewed_at), Some(commented_at)) => {
                    if reviewed_at > commented_at {
                        Some(commented_at)
                    } else {
                        Some(reviewed_at)
                    }
                }
                (None, None) => None,
            };

            let mut reviewee_comments_count = 0;
            {
                reviewee_comments_count += reviews
                    .unwrap()
                    .iter()
                    .filter(|item| match item {
                        Some(item) => {
                            if item.author.as_ref().is_none() {
                                return false;
                            }
                            item.author.as_ref().unwrap().login == author.as_ref()
                        }
                        _ => false,
                    })
                    .count() as i64;
                reviewee_comments_count += comments
                    .unwrap()
                    .iter()
                    .filter(|item| match item {
                        Some(item) => {
                            if item.author.as_ref().is_none() {
                                return false;
                            }
                            item.author.as_ref().unwrap().login == author.as_ref()
                        }
                        _ => false,
                    })
                    .count() as i64;
            }
            let mut reviewer_comments_count = 0;
            {
                reviewer_comments_count += reviews
                    .unwrap()
                    .iter()
                    .filter(|item| match item {
                        Some(item) => {
                            if item.state == pull_requests_query::PullRequestReviewState::APPROVED
                                && item.body.is_empty()
                            {
                                return false;
                            }
                            if item.author.as_ref().is_none() {
                                return false;
                            }
                            item.author.as_ref().unwrap().login != author.as_ref()
                        }
                        _ => false,
                    })
                    .count() as i64;
                reviewer_comments_count += comments
                    .unwrap()
                    .iter()
                    .filter(|item| match item {
                        Some(item) => {
                            if item.author.as_ref().is_none() {
                                return false;
                            }
                            item.author.as_ref().unwrap().login != author.as_ref()
                        }
                        _ => false,
                    })
                    .count() as i64;
            }

            let approved_at = reviews
                .unwrap()
                .iter()
                .find(|review| {
                    review.as_ref().unwrap().state
                        == pull_requests_query::PullRequestReviewState::APPROVED
                })
                .map(|approved| approved.as_ref().unwrap().created_at.clone());

            let merged_at = pr.merged_at.clone();
            summary.prs_summaries.push(PullRequestSummary {
                url: pr.url.clone(),
                author,
                comments_count: pr.total_comments_count.unwrap(),
                reviewee_comments_count,
                reviewer_comments_count,
                commits_count: pr.commits.total_count,
                changed_files_count: pr.changed_files,
                created_at: pr.created_at.clone(),
                first_contacted_at,
                approved_at,
                merged_at,
            })
        }

        summary.aggregate_summary();

        Ok(summary)
    }

    pub async fn get_pull_requests_summary_on_individuals(
        &self,
        repo: String,
        start_date: String,
        end_date: String,
        individuals: Vec<String>,
    ) -> Result<HashMap<String, PullRequestsSummary>, anyhow::Error> {
        let mut summaries: HashMap<String, PullRequestsSummary> = HashMap::new();

        let pull_requests = self
            .get_pull_requests(&repo, &start_date, &end_date)
            .await?;

        for pull_request in pull_requests.inner.iter() {
            let pr = &pull_request.inner;

            for individual in individuals.iter() {
                let author = match pr.author.as_ref() {
                    Some(author) => author.login.clone(),
                    None => "no-author".to_string(),
                };
                summaries
                    .entry(individual.clone())
                    .and_modify(|summary| {
                        if *individual == author {
                            summary.prs_count += 1
                        }
                    })
                    .or_insert(PullRequestsSummary::new(
                        start_date.clone(),
                        end_date.clone(),
                    ));

                let reviews = pr.reviews.as_ref().unwrap().nodes.as_ref();
                let comments = pr.comments.nodes.as_ref();
                let first_review = reviews.as_ref().unwrap().iter().find(|review| {
                    if review.as_ref().unwrap().author.as_ref().is_none() {
                        return false;
                    }
                    review.as_ref().unwrap().author.as_ref().unwrap().login != author
                        && review.as_ref().unwrap().author.as_ref().unwrap().login
                            == individual.as_ref()
                });

                let first_reviewed_at =
                    first_review.map(|review| review.as_ref().unwrap().created_at.clone());
                let first_comment = comments.as_ref().unwrap().iter().find(|comment| {
                    if comment.as_ref().unwrap().author.as_ref().is_none() {
                        return false;
                    }
                    comment.as_ref().unwrap().author.as_ref().unwrap().login != author
                        && comment.as_ref().unwrap().author.as_ref().unwrap().login
                            == individual.as_ref()
                });
                let first_commented_at =
                    first_comment.map(|comment| comment.as_ref().unwrap().created_at.clone());
                let first_contacted_at = match (first_reviewed_at, first_commented_at) {
                    (None, Some(commented_at)) => Some(commented_at),
                    (Some(reviewed_at), None) => Some(reviewed_at),
                    (Some(reviewed_at), Some(commented_at)) => {
                        if reviewed_at > commented_at {
                            Some(commented_at)
                        } else {
                            Some(reviewed_at)
                        }
                    }
                    (None, None) => None,
                };

                let mut comments_count = 0;
                {
                    comments_count += reviews
                        .as_ref()
                        .unwrap()
                        .iter()
                        .filter(|item| match item {
                            Some(item) => {
                                if item.author.as_ref().is_none() {
                                    return false;
                                }
                                item.author.as_ref().unwrap().login == individual.as_ref()
                            }
                            _ => false,
                        })
                        .count() as i64;
                    comments_count += comments
                        .as_ref()
                        .unwrap()
                        .iter()
                        .filter(|item| match item {
                            Some(item) => {
                                if item.author.as_ref().is_none() {
                                    return false;
                                }
                                item.author.as_ref().unwrap().login == individual.as_ref()
                            }
                            _ => false,
                        })
                        .count() as i64;
                }

                let reviewee_comments_count = if author == individual.as_ref() {
                    comments_count
                } else {
                    0
                };
                let reviewer_comments_count = if author != individual.as_ref() {
                    comments_count
                } else {
                    0
                };

                let commits_count = if author == *individual {
                    pr.commits.total_count
                } else {
                    0
                };
                let changed_files_count = if author == *individual {
                    pr.changed_files
                } else {
                    0
                };

                let approved_at = match reviews.as_ref().unwrap().iter().find(|review| {
                    review.as_ref().unwrap().state
                        == pull_requests_query::PullRequestReviewState::APPROVED
                }) {
                    Some(approved) => {
                        match approved
                            .as_ref()
                            .unwrap()
                            .author
                            .as_ref()
                            .unwrap()
                            .login
                            .clone()
                            == individual.as_ref()
                        {
                            true => Some(approved.as_ref().unwrap().created_at.clone()),
                            false => None,
                        }
                    }
                    None => None,
                };

                let merged_at = match pr.merged_by.as_ref() {
                    Some(merged_by) => match merged_by.login.clone() == individual.as_ref() {
                        true => pr.merged_at.clone(),
                        false => None,
                    },
                    None => None,
                };

                summaries.entry(individual.clone()).and_modify(|summary| {
                    summary.prs_summaries.push(PullRequestSummary {
                        url: pr.url.clone(),
                        author: author.clone(),
                        comments_count,
                        reviewee_comments_count,
                        reviewer_comments_count,
                        commits_count,
                        changed_files_count,
                        created_at: pr.created_at.clone(),
                        first_contacted_at,
                        approved_at,
                        merged_at,
                    })
                });
                summaries.entry(individual.clone()).and_modify(|summary| {
                    summary.aggregate_summary();
                });
            }
        }

        Ok(summaries)
    }
}
