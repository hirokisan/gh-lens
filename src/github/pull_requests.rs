use super::pull_request::PullRequest;

pub struct PullRequests {
    pub inner: Vec<PullRequest>,
}

impl PullRequests {
    pub(super) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(super) fn add(&mut self, item: PullRequest) {
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
            match pr.first_contacted_at() {
                Some(first_contacted_at) => {
                    count += 1;
                    total_seconds += first_contacted_at.diff_seconds(&pr.created_at());
                }
                None => continue,
            }
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
            match pr.first_contacted_at_by(by) {
                Some(first_contacted_at_by) => {
                    count += 1;
                    total_seconds += first_contacted_at_by.diff_seconds(&pr.created_at());
                }
                None => continue,
            }
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
            match pr.approved_at() {
                Some(approved_at) => {
                    count += 1;
                    total_seconds += approved_at.diff_seconds(&pr.created_at());
                }
                None => continue,
            }
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
            match pr.approved_at_by(by) {
                Some(approved_at_by) => {
                    count += 1;
                    total_seconds += approved_at_by.diff_seconds(&pr.created_at());
                }
                None => continue,
            }
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
            match pr.merged_at() {
                Some(merged_at) => {
                    count += 1;
                    total_seconds += merged_at.diff_seconds(&pr.created_at());
                }
                None => continue,
            }
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
            match pr.merged_at_by(by) {
                Some(merged_at_by) => {
                    count += 1;
                    total_seconds += merged_at_by.diff_seconds(&pr.created_at());
                }
                None => continue,
            }
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::gql::pull_requests_query::pull_requests_query;
    use crate::github::gql::pull_requests_query::tests::*;
    use crate::github::gql::scaler::DateTime;

    #[test]
    fn test_count() {
        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let want = 2;
        let got = prs.count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_count_by() {
        let by_name = "by".to_string();
        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(by_name.clone()),
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let want = 1;
        let got = prs.count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count() {
        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        total_comments_count: Some(10),
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        total_comments_count: Some(10),
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let want = 20;
        let got = prs.comments_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_by() {
        let by_name = "by".to_string();
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some("other".to_string()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                        ..Default::default()
                    }
                ),
                ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        ..Default::default()
                    }
                ),
                ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        let prs = PullRequests { inner };

        let want = 3;
        let got = prs.comments_count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_average() {
        struct Case {
            prs: PullRequests,
            want: f64,
        }
        let cases = [
            Case {
                prs: PullRequests { inner: vec![] },
                want: 0.0,
            },
            Case {
                prs: PullRequests {
                    inner: vec![
                        PullRequest {
                            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                                PullRequestsQuerySearchNodesOnPullRequestParam {
                                    total_comments_count: Some(10),
                                    ..Default::default()
                                },
                            ),
                        },
                        PullRequest {
                            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                                PullRequestsQuerySearchNodesOnPullRequestParam {
                                    total_comments_count: Some(20),
                                    ..Default::default()
                                },
                            ),
                        },
                    ],
                },
                want: 15.0,
            },
        ];
        for case in cases {
            let got = case.prs.comments_count_average();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_comments_count_average_by() {
        let by_name = "by".to_string();
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some("other".to_string()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                        ..Default::default()
                    }
                ),
                ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                ..Default::default()
                    }
                ),
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        ..Default::default()
                    }
                ),
                ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }

        struct Case {
            prs: PullRequests,
            want: f64,
        }
        let cases = [
            Case {
                prs: PullRequests { inner: vec![] },
                want: 0.0,
            },
            Case {
                prs: PullRequests { inner },
                want: 1.5,
            },
        ];
        for case in cases {
            let got = case.prs.comments_count_average_by(&by_name);
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_changed_files_count() {
        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        changed_files: Some(10),
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        changed_files: Some(10),
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let want = 20;
        let got = prs.changed_files_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_changed_files_count_by() {
        let by_name = "by".to_string();
        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        changed_files: Some(10),
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(by_name.clone()),
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        changed_files: Some(10),
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let want = 10;
        let got = prs.changed_files_count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_changed_files_count_average() {
        struct Case {
            prs: PullRequests,
            want: f64,
        }
        let cases = [
            Case {
                prs: PullRequests { inner: vec![] },
                want: 0.0,
            },
            Case {
                prs: PullRequests {
                    inner: vec![
                        PullRequest {
                            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                                PullRequestsQuerySearchNodesOnPullRequestParam {
                                    changed_files: Some(10),
                                    ..Default::default()
                                },
                            ),
                        },
                        PullRequest {
                            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                                PullRequestsQuerySearchNodesOnPullRequestParam {
                                    changed_files: Some(20),
                                    ..Default::default()
                                },
                            ),
                        },
                    ],
                },
                want: 15.0,
            },
        ];
        for case in cases {
            let got = case.prs.changed_files_count_average();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_changed_files_count_average_by() {
        let by_name = "by".to_string();
        struct Case {
            prs: PullRequests,
            want: f64,
        }
        let cases = [
            Case {
                prs: PullRequests { inner: vec![] },
                want: 0.0,
            },
            Case {
                prs: PullRequests {
                    inner: vec![
                        PullRequest {
                            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                                PullRequestsQuerySearchNodesOnPullRequestParam {
                                    changed_files: Some(10),
                                    author: Some(
                                        get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                            PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                                login: Some(by_name.clone()),
                                            },
                                        ),
                                    ),
                                    ..Default::default()
                                },
                            ),
                        },
                        PullRequest {
                            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                                PullRequestsQuerySearchNodesOnPullRequestParam {
                                    changed_files: Some(20),
                                    ..Default::default()
                                },
                            ),
                        },
                    ],
                },
                want: 10.0,
            },
        ];
        for case in cases {
            let got = case.prs.changed_files_count_average_by(&by_name);
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_time_to_first_contacted_average() {
        let author_name = "author".to_string();
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let contacted_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some("other".to_string()),
                                        },
                                )),
                                created_at: Some(contacted_at),
                                ..Default::default()
                    }
                ),
            ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        created_at: Some(created_at),
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        let prs = PullRequests { inner };

        let got = prs.time_to_first_contacted_average();
        assert_eq!(want as f64, got);
    }

    #[test]
    fn test_time_to_first_contacted_average_by() {
        let author_name = "author".to_string();
        let by_name = "by".to_string();
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let contacted_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                            login: Some(by_name.clone()),
                                        },
                                )),
                                created_at: Some(contacted_at),
                                ..Default::default()
                    }
                ),
            ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        created_at: Some(created_at),
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        {
            let comments = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam { comments },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        let prs = PullRequests { inner };

        let got = prs.time_to_first_contacted_average_by(&by_name);
        assert_eq!(want as f64, got);
    }

    #[test]
    fn test_time_to_approved_average() {
        let author_name = "author".to_string();
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let approved_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();
        let mut inner = vec![];
        {
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        created_at: Some(approved_at),
                        state: Some(pull_requests_query::PullRequestReviewState::APPROVED),
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        created_at: Some(created_at),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        {
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        let prs = PullRequests { inner };

        let got = prs.time_to_approved_average();
        assert_eq!(want as f64, got);
    }

    #[test]
    fn test_time_to_approved_average_by() {
        let author_name = "author".to_string();
        let by_name = "by".to_string();
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let approved_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();
        let mut inner = vec![];
        {
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        created_at: Some(approved_at),
                        author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                    login: Some(by_name.clone()),
                                },
                        )),
                        state: Some(pull_requests_query::PullRequestReviewState::APPROVED),
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        created_at: Some(created_at),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        {
            let reviews = vec![
                get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam { reviews },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            })
        }
        let prs = PullRequests { inner };

        let got = prs.time_to_approved_average_by(&by_name);
        assert_eq!(want as f64, got);
    }

    #[test]
    fn test_time_to_merged_average() {
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let merged_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();

        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        created_at: Some(created_at),
                        merged_at: Some(merged_at),
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let got = prs.time_to_merged_average();
        assert_eq!(want as f64, got);
    }

    #[test]
    fn test_time_to_merged_average_by() {
        let merged_by_name = "by".to_string();
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let merged_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();

        let inner = vec![
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        created_at: Some(created_at),
                        merged_at: Some(merged_at),
                        merged_by: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_merged_by(
                                PullRequestsQuerySearchNodesOnPullRequestMergedByParam {
                                    login: Some(merged_by_name.clone()),
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            },
            PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let prs = PullRequests { inner };

        let got = prs.time_to_merged_average_by(&merged_by_name);
        assert_eq!(want as f64, got);
    }
}
