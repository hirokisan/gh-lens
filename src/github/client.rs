use graphql_client::GraphQLQuery;

use crate::github::gql::query::{pull_requests_query, PullRequestsQuery};
use crate::github::gql::scaler::DateTime;
use crate::github::pull_requests_summary::PullRequestsSummary;

use std::collections::HashMap;

pub struct Client {
    inner: octocrab::Octocrab,
}

pub struct PullRequests {
    pub inner: Vec<PullRequest>,
}

impl PullRequests {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn add(&mut self, item: PullRequest) {
        self.inner.push(item);
    }

    pub fn count(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn count_by(&self, by: &str) -> i64 {
        self.inner.iter().filter(|pr| pr.author() == by).count() as i64
    }

    pub fn comments_count(&self) -> i64 {
        self.inner.iter().map(|pr| pr.comments_count()).sum()
    }

    pub fn comments_count_by(&self, by: &str) -> i64 {
        self.inner.iter().map(|pr| pr.comments_count_by(by)).sum()
    }

    pub fn comments_count_average(&self) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.comments_count() as f64 / self.count() as f64
        }
    }

    pub fn comments_count_average_by(&self, by: &str) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.comments_count_by(by) as f64 / self.count() as f64
        }
    }

    pub fn commits_count(&self) -> i64 {
        self.inner.iter().map(|pr| pr.commits_count()).sum()
    }

    pub fn commits_count_by(&self, by: &str) -> i64 {
        self.inner.iter().map(|pr| pr.commits_count_by(by)).sum()
    }

    pub fn commits_count_average(&self) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.commits_count() as f64 / self.count() as f64
        }
    }

    pub fn commits_count_average_by(&self, by: &str) -> f64 {
        if self.count_by(by) == 0 {
            0.0
        } else {
            self.commits_count_by(by) as f64 / self.count_by(by) as f64
        }
    }

    pub fn changed_files_count(&self) -> i64 {
        self.inner.iter().map(|pr| pr.changed_files_count()).sum()
    }

    pub fn changed_files_count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .map(|pr| pr.changed_files_count_by(by))
            .sum()
    }

    pub fn changed_files_count_average(&self) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.changed_files_count() as f64 / self.count() as f64
        }
    }

    pub fn changed_files_count_average_by(&self, by: &str) -> f64 {
        if self.count_by(by) == 0 {
            0.0
        } else {
            self.changed_files_count_by(by) as f64 / self.count_by(by) as f64
        }
    }

    pub fn time_to_first_contacted_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.first_contacted_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr
                .first_contacted_at()
                .unwrap()
                .diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_first_contacted_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.first_contacted_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr
                .first_contacted_at_by(by)
                .unwrap()
                .diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_approved_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.approved_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr.approved_at().unwrap().diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_approved_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.approved_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr
                .approved_at_by(by)
                .unwrap()
                .diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_merged_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.merged_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr.merged_at().unwrap().diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_merged_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.merged_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr.merged_at_by(by).unwrap().diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }
}

pub struct PullRequest {
    inner: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest,
}

impl PullRequest {
    fn new(inner: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest) -> Self {
        Self { inner }
    }

    pub fn url(&self) -> String {
        self.inner.url.clone()
    }

    pub fn author(&self) -> String {
        match self.inner.author.as_ref() {
            Some(author) => author.login.clone(),
            None => "no-author".to_string(),
        }
    }

    pub fn comments_count(&self) -> i64 {
        self.inner.total_comments_count.unwrap()
    }

    pub fn comments_count_by(&self, by: &str) -> i64 {
        let mut comments_count = 0;

        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();
        let comments = self.inner.comments.nodes.as_ref();

        comments_count += reviews
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item| match item {
                Some(item) => {
                    if item.author.as_ref().is_none() {
                        return false;
                    }
                    item.author.as_ref().unwrap().login == by
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
                    item.author.as_ref().unwrap().login == by
                }
                _ => false,
            })
            .count() as i64;

        comments_count
    }

    pub fn commits_count(&self) -> i64 {
        self.inner.commits.total_count
    }

    pub fn commits_count_by(&self, by: &str) -> i64 {
        if self.author() != by {
            return 0;
        }
        self.commits_count()
    }

    pub fn changed_files_count(&self) -> i64 {
        self.inner.changed_files
    }

    pub fn changed_files_count_by(&self, by: &str) -> i64 {
        if self.author() != by {
            return 0;
        }
        self.changed_files_count()
    }

    pub fn created_at(&self) -> DateTime {
        self.inner.created_at.clone()
    }

    pub fn first_contacted_at(&self) -> Option<DateTime> {
        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();
        let comments = self.inner.comments.nodes.as_ref();
        let first_review = reviews.unwrap().iter().find(|review| {
            if review.as_ref().unwrap().author.as_ref().is_none() {
                return false;
            }
            review.as_ref().unwrap().author.as_ref().unwrap().login != self.author()
        });
        let first_reviewed_at =
            first_review.map(|review| review.as_ref().unwrap().created_at.clone());
        let first_comment = comments.unwrap().iter().find(|comment| {
            if comment.as_ref().unwrap().author.as_ref().is_none() {
                return false;
            }
            comment.as_ref().unwrap().author.as_ref().unwrap().login != self.author()
        });
        let first_commented_at =
            first_comment.map(|comment| comment.as_ref().unwrap().created_at.clone());

        match (first_reviewed_at, first_commented_at) {
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
        }
    }

    pub fn first_contacted_at_by(&self, by: &str) -> Option<DateTime> {
        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();
        let comments = self.inner.comments.nodes.as_ref();

        let first_review = reviews.as_ref().unwrap().iter().find(|review| {
            if review.as_ref().unwrap().author.as_ref().is_none() {
                return false;
            }
            review.as_ref().unwrap().author.as_ref().unwrap().login != self.author()
                && review.as_ref().unwrap().author.as_ref().unwrap().login == by
        });

        let first_reviewed_at =
            first_review.map(|review| review.as_ref().unwrap().created_at.clone());
        let first_comment = comments.as_ref().unwrap().iter().find(|comment| {
            if comment.as_ref().unwrap().author.as_ref().is_none() {
                return false;
            }
            comment.as_ref().unwrap().author.as_ref().unwrap().login != self.author()
                && comment.as_ref().unwrap().author.as_ref().unwrap().login == by
        });
        let first_commented_at =
            first_comment.map(|comment| comment.as_ref().unwrap().created_at.clone());

        match (first_reviewed_at, first_commented_at) {
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
        }
    }

    pub fn reviewee_comments_count(&self) -> i64 {
        let mut reviewee_comments_count = 0;

        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();
        let comments = self.inner.comments.nodes.as_ref();

        reviewee_comments_count += reviews
            .unwrap()
            .iter()
            .filter(|item| match item {
                Some(item) => {
                    if item.author.as_ref().is_none() {
                        return false;
                    }
                    item.author.as_ref().unwrap().login == self.author()
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
                    item.author.as_ref().unwrap().login == self.author()
                }
                _ => false,
            })
            .count() as i64;

        reviewee_comments_count
    }

    pub fn reviewee_comments_count_by(&self, by: &str) -> i64 {
        if self.author() != by {
            return 0;
        }
        self.comments_count_by(by)
    }

    pub fn reviewer_comments_count(&self) -> i64 {
        let mut reviewer_comments_count = 0;

        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();
        let comments = self.inner.comments.nodes.as_ref();

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
                    item.author.as_ref().unwrap().login != self.author()
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
                    item.author.as_ref().unwrap().login != self.author()
                }
                _ => false,
            })
            .count() as i64;

        reviewer_comments_count
    }

    pub fn reviewer_comments_count_by(&self, by: &str) -> i64 {
        if self.author() == by {
            return 0;
        }
        self.comments_count_by(by)
    }

    pub fn approved_at(&self) -> Option<DateTime> {
        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();

        reviews
            .unwrap()
            .iter()
            .find(|review| {
                review.as_ref().unwrap().state
                    == pull_requests_query::PullRequestReviewState::APPROVED
            })
            .map(|approved| approved.as_ref().unwrap().created_at.clone())
    }

    pub fn approved_at_by(&self, by: &str) -> Option<DateTime> {
        let reviews = self.inner.reviews.as_ref().unwrap().nodes.as_ref();

        match reviews.as_ref().unwrap().iter().find(|review| {
            review.as_ref().unwrap().state == pull_requests_query::PullRequestReviewState::APPROVED
        }) {
            Some(approved) => {
                match approved.as_ref().unwrap().author.as_ref().unwrap().login == by {
                    true => Some(approved.as_ref().unwrap().created_at.clone()),
                    false => None,
                }
            }
            None => None,
        }
    }

    pub fn merged_at(&self) -> Option<DateTime> {
        self.inner.merged_at.clone()
    }

    pub fn merged_at_by(&self, by: &str) -> Option<DateTime> {
        match self.inner.merged_by.as_ref() {
            Some(merged_by) => match merged_by.login == by {
                true => self.merged_at(),
                false => None,
            },
            None => None,
        }
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
