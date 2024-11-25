use super::gql::scaler::DateTime;
use super::PullRequests;

use serde::Serialize;

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
struct PullRequestSummary {
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

#[derive(Debug, Serialize, PartialEq)]
struct PullRequestCommentsCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize, PartialEq)]
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
    pub(super) fn new(start_date: String, end_date: String, pull_requests: &PullRequests) -> Self {
        let mut summary = PullRequestsSummary {
            start_date,
            end_date,
            prs_count: pull_requests.count(),
            comments_count: PullRequestCommentsCount {
                sum: pull_requests.comments_count(),
                average: pull_requests.comments_count_average(),
            },
            commits_count: PullRequestCommitsCount {
                sum: pull_requests.commits_count(),
                average: pull_requests.commits_count_average(),
            },
            changed_files_count: PullRequestChangedFilesCount {
                sum: pull_requests.changed_files_count(),
                average: pull_requests.changed_files_count_average(),
            },
            time_to_first_contacted: PullRequestTimeToFirstContacted {
                average: pull_requests.time_to_first_contacted_average(),
            },
            time_to_approved: PullRequestTimeToApproved {
                average: pull_requests.time_to_approved_average(),
            },
            time_to_merged: PullRequestTimeToMerged {
                average: pull_requests.time_to_merged_average(),
            },
            prs_summaries: vec![],
        };
        for pull_request in pull_requests.inner.iter() {
            let url = pull_request.url();
            let author = pull_request.author();
            let comments_count = pull_request.comments_count();
            let commits_count = pull_request.commits_count();
            let changed_files_count = pull_request.changed_files_count();
            let created_at = pull_request.created_at();
            let first_contacted_at = pull_request.first_contacted_at();
            let reviewee_comments_count = pull_request.reviewee_comments_count();
            let reviewer_comments_count = pull_request.reviewer_comments_count();
            let approved_at = pull_request.approved_at();
            let merged_at = pull_request.merged_at();

            summary.prs_summaries.push(PullRequestSummary {
                url,
                author,
                comments_count,
                reviewee_comments_count,
                reviewer_comments_count,
                commits_count,
                changed_files_count,
                created_at,
                first_contacted_at,
                approved_at,
                merged_at,
            })
        }
        summary
    }

    pub(super) fn new_with_by(
        start_date: String,
        end_date: String,
        pull_requests: &PullRequests,
        by: &str,
    ) -> Self {
        let mut summary = PullRequestsSummary {
            start_date,
            end_date,
            prs_count: pull_requests.count_by(by),
            comments_count: PullRequestCommentsCount {
                sum: pull_requests.comments_count_by(by),
                average: pull_requests.comments_count_average_by(by),
            },
            commits_count: PullRequestCommitsCount {
                sum: pull_requests.commits_count_by(by),
                average: pull_requests.commits_count_average_by(by),
            },
            changed_files_count: PullRequestChangedFilesCount {
                sum: pull_requests.changed_files_count_by(by),
                average: pull_requests.changed_files_count_average_by(by),
            },
            time_to_first_contacted: PullRequestTimeToFirstContacted {
                average: pull_requests.time_to_first_contacted_average_by(by),
            },
            time_to_approved: PullRequestTimeToApproved {
                average: pull_requests.time_to_approved_average_by(by),
            },
            time_to_merged: PullRequestTimeToMerged {
                average: pull_requests.time_to_merged_average_by(by),
            },
            prs_summaries: vec![],
        };
        for pull_request in pull_requests.inner.iter() {
            let url = pull_request.url();
            let author = pull_request.author();
            let comments_count = pull_request.comments_count_by(by);
            let commits_count = pull_request.commits_count_by(by);
            let changed_files_count = pull_request.changed_files_count_by(by);
            let created_at = pull_request.created_at();
            let first_contacted_at = pull_request.first_contacted_at_by(by);
            let reviewee_comments_count = pull_request.reviewee_comments_count_by(by);
            let reviewer_comments_count = pull_request.reviewer_comments_count_by(by);
            let approved_at = pull_request.approved_at_by(by);
            let merged_at = pull_request.merged_at_by(by);

            summary.prs_summaries.push(PullRequestSummary {
                url,
                author,
                comments_count,
                reviewee_comments_count,
                reviewer_comments_count,
                commits_count,
                changed_files_count,
                created_at,
                first_contacted_at,
                approved_at,
                merged_at,
            })
        }
        summary
    }
}
