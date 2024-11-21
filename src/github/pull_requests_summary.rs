use crate::github::gql::scaler::DateTime;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct PullRequestsSummary {
    start_date: String,
    end_date: String,

    pub(super) prs_count: i64,
    comments_count: PullRequestCommentsCount,
    commits_count: PullRequestCommitsCount,
    changed_files_count: PullRequestChangedFilesCount,
    time_to_first_contacted: PullRequestTimeToFirstContacted,
    time_to_approved: PullRequestTimeToApproved,
    time_to_merged: PullRequestTimeToMerged,

    pub(super) prs_summaries: Vec<PullRequestSummary>,
}

#[derive(Debug, Serialize)]
pub(super) struct PullRequestSummary {
    pub url: String,
    pub author: String,
    pub comments_count: i64,
    pub reviewee_comments_count: i64,
    pub reviewer_comments_count: i64,
    pub commits_count: i64,
    pub changed_files_count: i64,
    pub created_at: DateTime,
    pub first_contacted_at: Option<DateTime>,
    pub approved_at: Option<DateTime>,
    pub merged_at: Option<DateTime>,
}

#[derive(Debug, Serialize)]
struct PullRequestCommentsCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
struct PullRequestCommitsCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
struct PullRequestChangedFilesCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
struct PullRequestTimeToFirstContacted {
    average: f64, // sec
}

#[derive(Debug, Serialize)]
struct PullRequestTimeToApproved {
    average: f64, // sec
}

#[derive(Debug, Serialize)]
struct PullRequestTimeToMerged {
    average: f64, // sec
}

impl PullRequestsSummary {
    pub(super) fn new(start_date: String, end_date: String) -> Self {
        PullRequestsSummary {
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
        }
    }

    pub(super) fn aggregate_summary(&mut self) {
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
