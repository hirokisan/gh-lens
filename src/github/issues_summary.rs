use super::gql::scaler::DateTime;
use super::Issues;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IssuesSummary {
    start_date: String,
    end_date: String,

    issues_count: i64,
    assigns_count: i64,
    comments_count: IssueCommentsCount,
    time_to_closed: IssueTimeToClosed,

    issues_summaries: Vec<IssueSummary>,
}

#[derive(Debug, Serialize)]
struct IssueSummary {
    url: String,
    author: String,
    assignees: Option<Vec<String>>,
    participants: Option<Vec<String>>,
    comments_count: i64,
    created_at: DateTime,
    closed_at: Option<DateTime>,
}

#[derive(Debug, Serialize, PartialEq)]
struct IssueCommentsCount {
    sum: i64,
    average: f64,
}

#[derive(Debug, Serialize)]
struct IssueTimeToClosed {
    average: f64, // sec
}

impl IssuesSummary {
    pub(super) fn new(start_date: String, end_date: String, issues: &Issues) -> Self {
        let mut summary = IssuesSummary {
            start_date,
            end_date,
            issues_count: issues.count(),
            assigns_count: issues.assigns_count(),
            comments_count: IssueCommentsCount {
                sum: issues.comments_count(),
                average: issues.comments_count_average(),
            },
            time_to_closed: IssueTimeToClosed {
                average: issues.time_to_closed_average(),
            },
            issues_summaries: vec![],
        };
        for issue in issues.inner.iter() {
            let url = issue.url();
            let author = issue.author();
            let assignees = issue.assignees();
            let participants = issue.participants();
            let comments_count = issue.comments_count();
            let created_at = issue.created_at();
            let closed_at = issue.closed_at();

            summary.issues_summaries.push(IssueSummary {
                url,
                author,
                assignees,
                participants,
                comments_count,
                created_at,
                closed_at,
            })
        }
        summary
    }

    pub(super) fn new_with_by(
        start_date: String,
        end_date: String,
        issues: &Issues,
        by: &str,
    ) -> Self {
        let mut summary = IssuesSummary {
            start_date,
            end_date,
            issues_count: issues.count_by(by),
            assigns_count: issues.assigns_count_by(by),
            comments_count: IssueCommentsCount {
                sum: issues.comments_count_by(by),
                average: issues.comments_count_average_by(by),
            },
            time_to_closed: IssueTimeToClosed {
                average: issues.time_to_closed_average_by(by),
            },
            issues_summaries: vec![],
        };
        for issue in issues.inner.iter() {
            let url = issue.url();
            let author = issue.author();
            let assignees = issue.assignees();
            let participants = issue.participants();
            let comments_count = issue.comments_count_by(by);
            let created_at = issue.created_at();
            let closed_at = issue.closed_at_by(by);

            summary.issues_summaries.push(IssueSummary {
                url,
                author,
                assignees,
                participants,
                comments_count,
                created_at,
                closed_at,
            })
        }
        summary
    }
}
