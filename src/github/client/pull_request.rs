use crate::github::gql::query::pull_requests_query;
use crate::github::gql::scaler::DateTime;

pub struct PullRequest {
    pub(super) inner: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest,
}

impl PullRequest {
    pub(super) fn new(
        inner: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest,
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::gql::query::tests::*;
    use crate::github::gql::scaler::DateTime;

    #[test]
    fn test_url() {
        let want = "test".to_string();
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    url: Some(want.clone()),
                    ..Default::default()
                },
            ),
        };

        let got = pr.url();
        assert_eq!(want, got);
    }

    #[test]
    fn test_author() {
        struct Case<'a> {
            author: Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthor>,
            want: &'a str,
        }
        let cases = [
            Case {
                author: Some(
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                        PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                            login: Some("test".to_string()),
                        },
                    ),
                ),
                want: "test",
            },
            Case {
                author: None,
                want: "no-author",
            },
        ];
        for mut case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: std::mem::take(&mut case.author),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.author();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_comments_count() {
        let want = 10;
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    total_comments_count: Some(want),
                    ..Default::default()
                },
            ),
        };

        let got = pr.comments_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_by() {
        let by = "test".to_string();
        let comments = vec![
            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(by.clone()),
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
                                        login: Some(by.clone()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other1".to_string()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other2".to_string()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
        ];
        let pr = PullRequest {
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
        };

        let want = 2;
        let got = pr.comments_count_by(&by);
        assert_eq!(want, got);
    }

    #[test]
    fn test_commits_count() {
        let want = 10;
        let commits = get_dummy_pull_requests_query_search_nodes_on_pull_request_commits(
            PullRequestsQuerySearchNodesOnPullRequestCommitsParam {
                total_count: Some(want),
                ..Default::default()
            },
        );
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    commits: Some(commits),
                    ..Default::default()
                },
            ),
        };

        let got = pr.commits_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_commits_count_by() {
        let by = "test".to_string();
        struct Case {
            author: Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthor>,
            commits: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommits,
            want: i64,
        }
        let cases = [
            Case {
                author: Some(
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                        PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                            login: Some(by.clone()),
                        },
                    ),
                ),
                commits: get_dummy_pull_requests_query_search_nodes_on_pull_request_commits(
                    PullRequestsQuerySearchNodesOnPullRequestCommitsParam {
                        total_count: Some(10),
                        ..Default::default()
                    },
                ),
                want: 10,
            },
            Case {
                author: None,
                commits: get_dummy_pull_requests_query_search_nodes_on_pull_request_commits(
                    PullRequestsQuerySearchNodesOnPullRequestCommitsParam {
                        total_count: Some(10),
                        ..Default::default()
                    },
                ),
                want: 0,
            },
        ];
        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        commits: Some(case.commits),
                        author: case.author,
                        ..Default::default()
                    },
                ),
            };

            let got = pr.commits_count_by(&by);
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_changed_files_count() {
        let want = 10;
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    changed_files: Some(want),
                    ..Default::default()
                },
            ),
        };

        let got = pr.changed_files_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_changed_files_count_by() {
        let by = "test".to_string();
        struct Case {
            author: Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthor>,
            changed_files_count: i64,
            want: i64,
        }
        let cases = [
            Case {
                author: Some(
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                        PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                            login: Some(by.clone()),
                        },
                    ),
                ),
                changed_files_count: 10,
                want: 10,
            },
            Case {
                author: None,
                changed_files_count: 10,
                want: 0,
            },
        ];
        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        changed_files: Some(case.changed_files_count),
                        author: case.author,
                        ..Default::default()
                    },
                ),
            };

            let got = pr.changed_files_count_by(&by);
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_reviewee_comments_count() {
        let author_name = "author".to_string();
        let comments = vec![
            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(author_name.clone()),
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
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other1".to_string()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
        ];
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    author: Some(
                        get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                            PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                login: Some(author_name.clone()),
                            },
                        ),
                    ),
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
        };

        let want = 2;
        let got = pr.reviewee_comments_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_reviewee_comments_count_by() {
        let author_name = "author".to_string();
        let not_author_name = "not-author".to_string();
        struct Case<'a> {
            name: &'a str,
            by_name: &'a str,
            comments:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes>,
            reviews:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
            want: i64,
        }
        let cases = [
            Case{
                name: "by is author",
                by_name: &author_name,
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: 2,
            },
            Case{
                name: "by is not author",
                by_name: &not_author_name,
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(author_name.clone()),
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
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: 0,
            },
        ];

        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam {
                                    comments: case.comments,
                                },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
                                    reviews: case.reviews,
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.reviewee_comments_count_by(case.by_name);
            assert_eq!(case.want, got, "{}", case.name);
        }
    }

    #[test]
    fn test_reviewer_comments_count() {
        let author_name = "author".to_string();
        let comments = vec![
            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(author_name.clone()),
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
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other1".to_string()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam{
                    author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other2".to_string()),
                                    },
                            )),
                            ..Default::default()
                }
            ),
        ];
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    author: Some(
                        get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                            PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                login: Some(author_name.clone()),
                            },
                        ),
                    ),
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
        };

        let want = 3;
        let got = pr.reviewer_comments_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_reviewer_comments_count_by() {
        let author_name = "author".to_string();
        let not_author_name_a = "not-author-a".to_string();
        let not_author_name_b = "not-author-b".to_string();
        struct Case<'a> {
            name: &'a str,
            by_name: &'a str,
            comments:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes>,
            reviews:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
            want: i64,
        }
        let cases = [
            Case{
                name: "by is author",
                by_name: &author_name,
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: 0,
            },
            Case{
                name: "by is not-author-a",
                by_name: &not_author_name_a,
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(author_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(not_author_name_a.clone()),
                                    },
                            )),
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(not_author_name_a.clone()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(not_author_name_b.clone()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                ],
                want: 2,
            },
        ];

        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam {
                                    comments: case.comments,
                                },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
                                    reviews: case.reviews,
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.reviewer_comments_count_by(case.by_name);
            assert_eq!(case.want, got, "{}", case.name);
        }
    }

    #[test]
    fn test_created_at() {
        let want: DateTime = (&chrono::Utc::now()).into();
        let pr = PullRequest {
            inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                PullRequestsQuerySearchNodesOnPullRequestParam {
                    created_at: Some(want.clone()),
                    ..Default::default()
                },
            ),
        };

        let got = pr.created_at();
        assert_eq!(want, got);
    }

    #[test]
    fn test_first_contacted_at() {
        let author_name = "author".to_string();
        let want: DateTime = (&chrono::Utc::now()).into();

        struct Case {
            comments:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes>,
            reviews:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
            want: Option<DateTime>,
        }
        let cases = [
            Case{
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            created_at: Some(want.clone()),
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some("other".to_string()),
                                    },
                            )),
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: Some(want.clone()),
            },
            Case{
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            created_at: Some(want.clone()),
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other".to_string()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: Some(want.clone()),
            },
            Case{
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: None,
            },
        ];
        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam {
                                    comments: case.comments,
                                },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
                                    reviews: case.reviews,
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.first_contacted_at();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_first_contacted_at_by() {
        let author_name = "author".to_string();
        let by_name = "by".to_string();
        let now = chrono::Utc::now();
        let want_day_before: DateTime =
            (&now.checked_sub_days(chrono::Days::new(1)).unwrap()).into();
        let want: DateTime = (&now).into();

        struct Case {
            comments:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes>,
            reviews:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
            want: Option<DateTime>,
        }
        let cases = [
            Case{
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            created_at: Some(want_day_before.clone()),
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some("other".to_string()),
                                    },
                            )),
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            created_at: Some(want.clone()),
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam{
                                        login: Some(by_name.clone()),
                                    },
                            )),
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: Some(want.clone()),
            },
            Case{
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            created_at: Some(want_day_before.clone()),
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other".to_string()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            created_at: Some(want.clone()),
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(by_name.clone()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                ],
                want: Some(want.clone()),
            },
            Case{
                comments: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam{
                            ..Default::default()
                        }
                    ),
                ],
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: None,
            },
        ];
        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        comments: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                                PullRequestsQuerySearchNodesOnPullRequestCommentsParam {
                                    comments: case.comments,
                                },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
                                    reviews: case.reviews,
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.first_contacted_at_by(&by_name);
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_approved_at() {
        let author_name = "author".to_string();
        let want: DateTime = (&chrono::Utc::now()).into();

        struct Case<'a> {
            name: &'a str,
            reviews:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
            want: Option<DateTime>,
        }
        let cases = [
            Case {
                name: "approve is not exist",
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: None,
            },
            Case {
                name: "approve exists",
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            created_at: Some(want.clone()),
                            state: Some(pull_requests_query::PullRequestReviewState::APPROVED),
                            ..Default::default()
                        },
                    ),
                ],
                want: Some(want.clone()),
            },
        ];
        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
                                    reviews: case.reviews,
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.approved_at();
            assert_eq!(case.want, got, "{}", case.name);
        }
    }

    #[test]
    fn test_approved_at_by() {
        let author_name = "author".to_string();
        let by_name = "by".to_string();
        let want: DateTime = (&chrono::Utc::now()).into();

        struct Case<'a> {
            name: &'a str,
            reviews:
                Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
            want: Option<DateTime>,
        }
        let cases = [
            Case {
                name: "approve is not exist",
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            ..Default::default()
                        },
                    ),
                ],
                want: None,
            },
            Case {
                name: "approve belongs to by is not exist",
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some("other".to_string()),
                                    },
                            )),
                            ..Default::default()
                        },
                    ),
                ],
                want: None,
            },
            Case {
                name: "approve belongs to by exists",
                reviews: vec![
                    get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
                        PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
                            author: Some(get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
                                    PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam{
                                        login: Some(by_name.clone()),
                                    },
                            )),
                            created_at: Some(want.clone()),
                            state: Some(pull_requests_query::PullRequestReviewState::APPROVED),
                            ..Default::default()
                        },
                    ),
                ],
                want: Some(want.clone()),
            },
        ];
        for case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        author: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
                                PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
                                    login: Some(author_name.clone()),
                                },
                            ),
                        ),
                        reviews: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
                                PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
                                    reviews: case.reviews,
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.approved_at_by(&by_name);
            assert_eq!(case.want, got, "{}", case.name);
        }
    }

    #[test]
    fn test_merged_at() {
        let now: DateTime = (&chrono::Utc::now()).into();

        struct Case {
            merged_at: Option<DateTime>,
            want: Option<DateTime>,
        }
        let cases = [
            Case {
                merged_at: None,
                want: None,
            },
            Case {
                merged_at: Some(now.clone()),
                want: Some(now.clone()),
            },
        ];
        for mut case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        merged_at: std::mem::take(&mut case.merged_at),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.merged_at();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_merged_at_by() {
        let merged_by_name = "by".to_string();
        let now: DateTime = (&chrono::Utc::now()).into();

        struct Case<'a> {
            name: &'a str,
            merged_at: Option<DateTime>,
            by: &'a str,
            want: Option<DateTime>,
        }
        let cases = [
            Case {
                name: "not merged",
                merged_at: None,
                by: &merged_by_name,
                want: None,
            },
            Case {
                name: "merged, but not belongs to by",
                merged_at: Some(now.clone()),
                by: "other",
                want: None,
            },
            Case {
                name: "merged, and belongs to by",
                merged_at: Some(now.clone()),
                by: &merged_by_name,
                want: Some(now.clone()),
            },
        ];
        for mut case in cases {
            let pr = PullRequest {
                inner: get_dummy_pull_requests_query_search_nodes_on_pull_request(
                    PullRequestsQuerySearchNodesOnPullRequestParam {
                        merged_by: Some(
                            get_dummy_pull_requests_query_search_nodes_on_pull_request_merged_by(
                                PullRequestsQuerySearchNodesOnPullRequestMergedByParam {
                                    login: Some(merged_by_name.clone()),
                                },
                            ),
                        ),
                        merged_at: std::mem::take(&mut case.merged_at),
                        ..Default::default()
                    },
                ),
            };

            let got = pr.merged_at_by(case.by);
            assert_eq!(case.want, got, "{}", case.name);
        }
    }
}
