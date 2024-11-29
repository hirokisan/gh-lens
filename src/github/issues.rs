use super::issue::Issue;

pub struct Issues {
    pub inner: Vec<Issue>,
}

impl Issues {
    pub(super) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(super) fn add(&mut self, item: Issue) {
        self.inner.push(item);
    }

    pub fn count(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .filter(|issue| issue.author() == by)
            .count() as i64
    }

    pub fn assigns_count(&self) -> i64 {
        self.inner
            .iter()
            .filter(|issue| {
                issue
                    .assignees()
                    .map_or(false, |assignees| !assignees.is_empty())
            })
            .count() as i64
    }

    pub fn assigns_count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .filter(|issue| {
                issue
                    .assignees()
                    .map_or(false, |assignees| assignees.contains(&by.to_string()))
            })
            .count() as i64
    }

    pub fn comments_count(&self) -> i64 {
        self.inner.iter().map(|issue| issue.comments_count()).sum()
    }

    pub fn comments_count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .map(|issue| issue.comments_count_by(by))
            .sum()
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

    pub fn time_to_closed_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for issue in self.inner.iter() {
            if issue.closed_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += issue.closed_at().unwrap().diff_seconds(&issue.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_closed_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for issue in self.inner.iter() {
            if issue.closed_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += issue
                .closed_at_by(by)
                .unwrap()
                .diff_seconds(&issue.created_at());
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
    use crate::github::gql::issues_query::tests::*;
    use crate::github::gql::scaler::DateTime;

    #[test]
    fn test_count() {
        let inner = vec![
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        ..Default::default()
                    },
                ),
            },
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let issues = Issues { inner };

        let want = 2;
        let got = issues.count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_count_by() {
        let by_name = "by".to_string();
        let inner = vec![
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        author: Some(get_dummy_issues_query_search_nodes_on_issue_author(
                            IssuesQuerySearchNodesOnIssueAuthorParam {
                                login: Some(by_name.clone()),
                            },
                        )),
                        ..Default::default()
                    },
                ),
            },
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let issues = Issues { inner };

        let want = 1;
        let got = issues.count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_assigns_count() {
        let mut inner = vec![];
        {
            let assignee = "a".to_string();
            let assignees = vec![
                get_dummy_issues_query_search_nodes_on_issue_assignees_nodes(
                    IssuesQuerySearchNodesOnIssueAssigneesNodesParam {
                        login: Some(assignee.clone()),
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        assignees: Some(get_dummy_issues_query_search_nodes_on_issue_assignees(
                            IssuesQuerySearchNodesOnIssueAssigneesParam { assignees },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        {
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        ..Default::default()
                    },
                ),
            });
        }
        let issues = Issues { inner };

        let want = 1;
        let got = issues.assigns_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_assigns_count_by() {
        let by_name = "by".to_string();
        let mut inner = vec![];
        {
            let assignee = by_name.clone();
            let assignees = vec![
                get_dummy_issues_query_search_nodes_on_issue_assignees_nodes(
                    IssuesQuerySearchNodesOnIssueAssigneesNodesParam {
                        login: Some(assignee.clone()),
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        assignees: Some(get_dummy_issues_query_search_nodes_on_issue_assignees(
                            IssuesQuerySearchNodesOnIssueAssigneesParam { assignees },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        {
            let assignees = vec![
                get_dummy_issues_query_search_nodes_on_issue_assignees_nodes(
                    IssuesQuerySearchNodesOnIssueAssigneesNodesParam {
                        login: Some("other".to_string()),
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        assignees: Some(get_dummy_issues_query_search_nodes_on_issue_assignees(
                            IssuesQuerySearchNodesOnIssueAssigneesParam { assignees },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        let issues = Issues { inner };

        let want = 1;
        let got = issues.assigns_count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count() {
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        author: Some(
                            get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                                IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                    login: Some("someone_a".to_string()),
                                },
                            ),
                        ),
                    },
                ),
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        author: Some(
                            get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                                IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                    login: Some("someone_b".to_string()),
                                },
                            ),
                        ),
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        {
            let comments = vec![get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                    author: Some(
                        get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                            IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                login: Some("someone_b".to_string()),
                            },
                        ),
                    ),
                },
            )];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        let issues = Issues { inner };

        let want = 3;
        let got = issues.comments_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_by() {
        let by_name = "by".to_string();
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        author: Some(
                            get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                                IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                    login: Some(by_name.clone()),
                                },
                            ),
                        ),
                    },
                ),
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        author: Some(
                            get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                                IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                    login: Some("someone_b".to_string()),
                                },
                            ),
                        ),
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        {
            let comments = vec![get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                    author: Some(
                        get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                            IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                login: Some(by_name.clone()),
                            },
                        ),
                    ),
                },
            )];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        let issues = Issues { inner };

        let want = 2;
        let got = issues.comments_count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_average() {
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        ..Default::default()
                    },
                ),
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        ..Default::default()
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        {
            let comments = vec![get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                    ..Default::default()
                },
            )];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        let issues = Issues { inner };

        let want = 1.5;
        let got = issues.comments_count_average();
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_average_by() {
        let by_name = "by".to_string();
        let mut inner = vec![];
        {
            let comments = vec![
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        author: Some(
                            get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                                IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                    login: Some("other".to_string()),
                                },
                            ),
                        ),
                    },
                ),
                get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                    IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                        author: Some(
                            get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                                IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                    login: Some(by_name.clone()),
                                },
                            ),
                        ),
                    },
                ),
            ];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        {
            let comments = vec![get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
                IssuesQuerySearchNodesOnIssueCommentsNodesParam {
                    author: Some(
                        get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
                            IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
                                login: Some(by_name.clone()),
                            },
                        ),
                    ),
                },
            )];
            inner.push(Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                            IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                        )),
                        ..Default::default()
                    },
                ),
            });
        }
        let issues = Issues { inner };

        let want = 1.0;
        let got = issues.comments_count_average_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_time_to_closed_average() {
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let closed_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();
        let inner = vec![
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        created_at: Some(created_at.clone()),
                        closed_at: Some(closed_at.clone()),
                        ..Default::default()
                    },
                ),
            },
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let issues = Issues { inner };

        let got = issues.time_to_closed_average();
        assert_eq!(want as f64, got);
    }

    #[test]
    fn test_time_to_closed_average_by() {
        let by_name = "by".to_string();
        let now = chrono::Utc::now();
        let created_at: DateTime = (&now).into();
        let want = 10;
        let closed_at: DateTime = (&now
            .checked_add_signed(chrono::TimeDelta::seconds(10))
            .unwrap())
            .into();

        let timeline_items = vec![
            get_dummy_issues_query_search_nodes_on_issue_timeline_items_nodes_closed_event(
                IssuesQuerySearchNodesOnIssueTimelineItemsNodesClosedEventParam {
                    actor: Some(
                        get_dummy_issues_query_search_nodes_on_issue_timeline_items_nodes_closed_event_actor(
                            IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActorParam {
                                login: Some(by_name.clone()),
                            },
                        ),
                    ),
                    created_at: Some(closed_at.clone()),
                },
            ),
        ];
        let inner = vec![
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        created_at: Some(created_at.clone()),
                        timeline_items: Some(
                            get_dummy_issues_query_search_nodes_on_issue_timeline_items(
                                IssuesQuerySearchNodesOnIssueTimelineItemsParam { timeline_items },
                            ),
                        ),
                        ..Default::default()
                    },
                ),
            },
            Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        ..Default::default()
                    },
                ),
            },
        ];
        let issues = Issues { inner };

        let got = issues.time_to_closed_average_by(&by_name);
        assert_eq!(want as f64, got);
    }
}
