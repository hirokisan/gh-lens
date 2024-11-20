use graphql_client::GraphQLQuery;
use serde::Serialize;

use crate::github::gql::query::{pull_requests_query, PullRequestsQuery};
use crate::github::gql::scaler::DateTime;

use std::collections::HashMap;

pub struct Client {
    inner: octocrab::Octocrab,
}

#[derive(Debug, Serialize)]
pub struct PullRequestsSummary {
    start_date: String,
    end_date: String,

    prs_count: i64,
    comments_count: PullRequestCommentsCount,
    commits_count: PullRequestCommitsCount,
    changed_files_count: PullRequestChangedFilesCount,
    time_to_first_contacted: PullRequestTimeToFirstContacted,
    time_to_approved: PullRequestTimeToApproved,
    time_to_merged: PullRequestTimeToMerged,

    prs_summaries: Vec<PullRequestSummary>,
}

#[derive(Debug, Serialize)]
pub struct PullRequestSummary {
    url: String,
    author: String,
    comments_count: i64,
    reviewee_comments_count: i64,
    reviewer_comments_count: i64,
    commits_count: i64,
    changed_files_count: i64,
    created_at: DateTime,
    first_contacted_at: Option<DateTime>,
    approved_at: Option<DateTime>,
    merged_at: Option<DateTime>,
}

#[derive(Debug, Serialize)]
pub struct PullRequestCommentsCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
pub struct PullRequestCommitsCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
pub struct PullRequestChangedFilesCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
pub struct PullRequestTimeToFirstContacted {
    average: f64, // sec
}

#[derive(Debug, Serialize)]
pub struct PullRequestTimeToApproved {
    average: f64, // sec
}

#[derive(Debug, Serialize)]
pub struct PullRequestTimeToMerged {
    average: f64, // sec
}

impl PullRequestsSummary {
    fn aggregate_summary(&mut self) {
        self.aggregate_comments_count();
        self.aggregate_commits_count();
        self.aggregate_changed_files_count();
        self.aggregate_time_to_first_contacted();
        self.aggregate_time_to_approved();
        self.aggregate_time_to_merged();
    }

    fn aggregate_comments_count(&mut self) {
        self.comments_count.sum = self
            .prs_summaries
            .iter()
            .map(|summary| summary.comments_count)
            .sum();
        self.comments_count.average = if self.prs_summaries.is_empty() {
            0.0
        } else {
            self.comments_count.sum as f64 / self.prs_summaries.len() as f64
        };
    }

    fn aggregate_commits_count(&mut self) {
        self.commits_count.sum = self
            .prs_summaries
            .iter()
            .map(|summary| summary.commits_count)
            .sum();
        self.commits_count.average = if self.prs_count == 0 {
            0.0
        } else {
            self.commits_count.sum as f64 / self.prs_count as f64
        };
    }

    fn aggregate_changed_files_count(&mut self) {
        self.changed_files_count.sum = self
            .prs_summaries
            .iter()
            .map(|summary| summary.changed_files_count)
            .sum();
        self.changed_files_count.average = if self.prs_count == 0 {
            0.0
        } else {
            self.changed_files_count.sum as f64 / self.prs_count as f64
        };
    }

    fn aggregate_time_to_first_contacted(&mut self) {
        let mut count = 0;
        let mut total_seconds = 0;
        for summary in self.prs_summaries.iter() {
            if summary.first_contacted_at.is_none() {
                continue;
            };
            count += 1;
            total_seconds += summary
                .first_contacted_at
                .as_ref()
                .unwrap()
                .diff_seconds(&summary.created_at);
        }
        self.time_to_first_contacted.average = if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        };
    }

    fn aggregate_time_to_approved(&mut self) {
        let mut count = 0;
        let mut total_seconds = 0;
        for summary in self.prs_summaries.iter() {
            if summary.approved_at.is_none() {
                continue;
            };
            count += 1;
            total_seconds += summary
                .approved_at
                .as_ref()
                .unwrap()
                .diff_seconds(&summary.created_at);
        }
        self.time_to_approved.average = if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        };
    }

    fn aggregate_time_to_merged(&mut self) {
        let mut count = 0;
        let mut total_seconds = 0;
        for summary in self.prs_summaries.iter() {
            if summary.merged_at.is_none() {
                continue;
            };
            count += 1;
            total_seconds += summary
                .merged_at
                .as_ref()
                .unwrap()
                .diff_seconds(&summary.created_at);
        }
        self.time_to_merged.average = if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        };
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

    pub async fn get_pull_requests_summary(
        &self,
        repo: String,
        start_date: String,
        end_date: String,
    ) -> PullRequestsSummary {
        let offset = 10;
        let query = format!("repo:{repo} is:pull-request created:{start_date}..{end_date}");
        let mut variables = pull_requests_query::Variables {
            first: offset,
            query: query.to_string(),
            threshold: 50,
            after: None,
        };

        let mut summary = PullRequestsSummary {
            start_date,
            end_date,
            prs_count: 0,
            comments_count: PullRequestCommentsCount {
                sum: 0,
                average: 0.0,
            },
            commits_count: PullRequestCommitsCount {
                sum: 0,
                average: 0.0,
            },
            changed_files_count: PullRequestChangedFilesCount {
                sum: 0,
                average: 0.0,
            },
            time_to_first_contacted: PullRequestTimeToFirstContacted { average: 0.0 },
            time_to_approved: PullRequestTimeToApproved { average: 0.0 },
            time_to_merged: PullRequestTimeToMerged { average: 0.0 },
            prs_summaries: vec![],
        };

        loop {
            let response: octocrab::Result<
                graphql_client::Response<pull_requests_query::ResponseData>,
            > = self
                .inner
                .graphql(&PullRequestsQuery::build_query(variables.clone()))
                .await;

            match response {
                Ok(response) => {
                    let prs = &response.data.as_ref().unwrap().search;
                    summary.prs_count = prs.issue_count;

                    for item in prs.nodes.as_ref().unwrap().iter().flatten() {
                        let pr = match item {
                            pull_requests_query::PullRequestsQuerySearchNodes::PullRequest(pr) => {
                                pr
                            }
                            _ => continue,
                        };
                        let author = match pr.author.as_ref() {
                            Some(author) => author.login.clone(),
                            None => "no-author".to_string(),
                        };

                        let reviews = pr.reviews.as_ref().unwrap().nodes.as_ref();
                        let comments = pr.comments.nodes.as_ref();
                        let first_review = match reviews.unwrap().iter().find(|review| {
                            if review.as_ref().unwrap().author.as_ref().is_none() {
                                return false;
                            }
                            review.as_ref().unwrap().author.as_ref().unwrap().login
                                != author.as_ref()
                        }) {
                            Some(review) => Some(review),
                            None => None,
                        };
                        let first_reviewed_at = match first_review {
                            Some(review) => Some(review.as_ref().unwrap().created_at.clone()),
                            None => None,
                        };
                        let first_comment = match comments.unwrap().iter().find(|comment| {
                            if comment.as_ref().unwrap().author.as_ref().is_none() {
                                return false;
                            }
                            comment.as_ref().unwrap().author.as_ref().unwrap().login
                                != author.as_ref()
                        }) {
                            Some(comment) => Some(comment),
                            None => None,
                        };
                        let first_commented_at = match first_comment {
                            Some(comment) => Some(comment.as_ref().unwrap().created_at.clone()),
                            None => None,
                        };
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
                                .count()
                                as i64;
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
                                .count()
                                as i64;
                        }
                        let mut reviewer_comments_count = 0;
                        {
                            reviewer_comments_count += reviews
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
                                .count()
                                as i64;
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
                                .count()
                                as i64;
                        }

                        let approved_at = match reviews.unwrap().iter().find(|review| {
                            review.as_ref().unwrap().state
                                == pull_requests_query::PullRequestReviewState::APPROVED
                        }) {
                            Some(approved) => Some(approved.as_ref().unwrap().created_at.clone()),
                            None => None,
                        };

                        let merged_at = match pr.merged_at.clone() {
                            Some(at) => Some(at),
                            None => None,
                        };
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

                    if !prs.page_info.has_next_page {
                        break;
                    }
                    variables.after = prs.page_info.end_cursor.clone();
                }
                Err(error) => {
                    println!("{error:#?}");
                }
            };
        }

        summary.aggregate_summary();

        summary
    }

    pub async fn get_pull_requests_summary_on_individuals(
        &self,
        repo: String,
        start_date: String,
        end_date: String,
        individuals: Vec<String>,
    ) -> HashMap<String, PullRequestsSummary> {
        let offset = 10;
        let query = format!("repo:{repo} is:pull-request created:{start_date}..{end_date}");
        let mut variables = pull_requests_query::Variables {
            first: offset,
            query: query.to_string(),
            threshold: 50,
            after: None,
        };

        let mut summaries: HashMap<String, PullRequestsSummary> = HashMap::new();

        loop {
            let response: octocrab::Result<
                graphql_client::Response<pull_requests_query::ResponseData>,
            > = self
                .inner
                .graphql(&PullRequestsQuery::build_query(variables.clone()))
                .await;

            match response {
                Ok(response) => {
                    let prs = &response.data.as_ref().unwrap().search;

                    for individual in individuals.iter() {
                        for item in prs.nodes.as_ref().unwrap().iter().flatten() {
                            let pr = match item {
                                pull_requests_query::PullRequestsQuerySearchNodes::PullRequest(
                                    pr,
                                ) => pr,
                                _ => continue,
                            };
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
                                .or_insert(PullRequestsSummary {
                                    start_date: start_date.clone(),
                                    end_date: end_date.clone(),
                                    prs_count: 0,
                                    comments_count: PullRequestCommentsCount {
                                        sum: 0,
                                        average: 0.0,
                                    },
                                    commits_count: PullRequestCommitsCount {
                                        sum: 0,
                                        average: 0.0,
                                    },
                                    changed_files_count: PullRequestChangedFilesCount {
                                        sum: 0,
                                        average: 0.0,
                                    },
                                    time_to_first_contacted: PullRequestTimeToFirstContacted {
                                        average: 0.0,
                                    },
                                    time_to_approved: PullRequestTimeToApproved { average: 0.0 },
                                    time_to_merged: PullRequestTimeToMerged { average: 0.0 },
                                    prs_summaries: vec![],
                                });

                            let reviews = pr.reviews.as_ref().unwrap().nodes.as_ref();
                            let comments = pr.comments.nodes.as_ref();
                            let first_review =
                                match reviews.as_ref().unwrap().iter().find(|review| {
                                    if review.as_ref().unwrap().author.as_ref().is_none() {
                                        return false;
                                    }
                                    review.as_ref().unwrap().author.as_ref().unwrap().login
                                        != author
                                        && review.as_ref().unwrap().author.as_ref().unwrap().login
                                            == individual.as_ref()
                                }) {
                                    Some(review) => Some(review),
                                    None => None,
                                };
                            let first_reviewed_at = match first_review {
                                Some(review) => Some(review.as_ref().unwrap().created_at.clone()),
                                None => None,
                            };
                            let first_comment =
                                match comments.as_ref().unwrap().iter().find(|comment| {
                                    if comment.as_ref().unwrap().author.as_ref().is_none() {
                                        return false;
                                    }
                                    comment.as_ref().unwrap().author.as_ref().unwrap().login
                                        != author
                                        && comment.as_ref().unwrap().author.as_ref().unwrap().login
                                            == individual.as_ref()
                                }) {
                                    Some(comment) => Some(comment),
                                    None => None,
                                };
                            let first_commented_at = match first_comment {
                                Some(comment) => Some(comment.as_ref().unwrap().created_at.clone()),
                                None => None,
                            };
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
                                            item.author.as_ref().unwrap().login
                                                == individual.as_ref()
                                        }
                                        _ => false,
                                    })
                                    .count()
                                    as i64;
                                comments_count += comments
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .filter(|item| match item {
                                        Some(item) => {
                                            if item.author.as_ref().is_none() {
                                                return false;
                                            }
                                            item.author.as_ref().unwrap().login
                                                == individual.as_ref()
                                        }
                                        _ => false,
                                    })
                                    .count()
                                    as i64;
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

                            let approved_at =
                                match reviews.as_ref().unwrap().iter().find(|review| {
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
                                            true => {
                                                Some(approved.as_ref().unwrap().created_at.clone())
                                            }
                                            false => None,
                                        }
                                    }
                                    None => None,
                                };

                            let merged_at = match pr.merged_by.as_ref() {
                                Some(merged_by) => {
                                    match merged_by.login.clone() == individual.as_ref() {
                                        true => match pr.merged_at.clone() {
                                            Some(at) => Some(at),
                                            None => None,
                                        },
                                        false => None,
                                    }
                                }
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
                        }
                        summaries.entry(individual.clone()).and_modify(|summary| {
                            summary.aggregate_summary();
                        });
                    }

                    if !prs.page_info.has_next_page {
                        break;
                    }
                    variables.after = prs.page_info.end_cursor.clone();
                }
                Err(error) => {
                    println!("{error:#?}");
                }
            };
        }

        summaries
    }
}
