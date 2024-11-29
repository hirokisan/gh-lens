use super::gql::issues_query::issues_query;
use super::gql::scaler::DateTime;

pub struct Issue {
    pub(super) inner: issues_query::IssuesQuerySearchNodesOnIssue,
}

impl Issue {
    pub(super) fn new(inner: issues_query::IssuesQuerySearchNodesOnIssue) -> Self {
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

    pub fn assignees(&self) -> Option<Vec<String>> {
        let nodes = self.inner.assignees.nodes.as_ref();
        if nodes.is_none() {
            return None;
        }
        let result: Vec<String> = nodes
            .unwrap()
            .iter()
            .map(|assignee| assignee.as_ref().unwrap().login.clone())
            .collect();
        if result.len() == 0 {
            return None;
        }
        Some(result)
    }

    pub fn participants(&self) -> Option<Vec<String>> {
        let nodes = self.inner.participants.nodes.as_ref();
        if nodes.is_none() {
            return None;
        }

        let result: Vec<String> = nodes
            .unwrap()
            .iter()
            .map(|participant| participant.as_ref().unwrap().login.clone())
            .collect();
        if result.len() == 0 {
            return None;
        }
        Some(result)
    }

    pub fn created_at(&self) -> DateTime {
        self.inner.created_at.clone()
    }

    pub fn comments_count(&self) -> i64 {
        let comments = self.inner.comments.nodes.as_ref();

        comments.unwrap().len() as i64
    }

    pub fn comments_count_by(&self, by: &str) -> i64 {
        let comments = self.inner.comments.nodes.as_ref();

        comments
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
            .count() as i64
    }

    pub fn closed_at(&self) -> Option<DateTime> {
        self.inner.closed_at.clone()
    }

    pub fn closed_at_by(&self, by: &str) -> Option<DateTime> {
        let nodes = self.inner.timeline_items.nodes.as_ref();
        if nodes.is_none() {
            return None;
        }

        let mut timeline_items = vec![];
        for item in nodes.unwrap().iter().flatten() {
            match item {
                issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodes::ClosedEvent(
                    event,
                ) => timeline_items.push(event),
                _ => continue,
            }
        }
        if timeline_items.len() == 0 {
            return None;
        }

        let closed_event = timeline_items.first().unwrap();

        if closed_event.actor.as_ref().is_none() {
            return None;
        }

        if closed_event.actor.as_ref().unwrap().login != by {
            return None;
        }

        Some(closed_event.created_at.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::gql::issues_query::tests::*;
    use crate::github::gql::scaler::DateTime;

    #[test]
    fn test_url() {
        let want = "test".to_string();
        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    url: Some(want.clone()),
                    ..Default::default()
                },
            ),
        };

        let got = issue.url();
        assert_eq!(want, got);
    }

    #[test]
    fn test_author() {
        struct Case<'a> {
            author: Option<issues_query::IssuesQuerySearchNodesOnIssueAuthor>,
            want: &'a str,
        }
        let cases = [
            Case {
                author: Some(get_dummy_issues_query_search_nodes_on_issue_author(
                    IssuesQuerySearchNodesOnIssueAuthorParam {
                        login: Some("test".to_string()),
                    },
                )),
                want: "test",
            },
            Case {
                author: None,
                want: "no-author",
            },
        ];
        for mut case in cases {
            let issue = Issue {
                inner: get_dummy_issues_query_search_nodes_on_issue(
                    IssuesQuerySearchNodesOnIssueParam {
                        author: std::mem::take(&mut case.author),
                        ..Default::default()
                    },
                ),
            };

            let got = issue.author();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn test_assignees() {
        let assignee_a = "a".to_string();
        let assignee_b = "b".to_string();
        let assignees = vec![
            get_dummy_issues_query_search_nodes_on_issue_assignees_nodes(
                IssuesQuerySearchNodesOnIssueAssigneesNodesParam {
                    login: Some(assignee_a.clone()),
                },
            ),
            get_dummy_issues_query_search_nodes_on_issue_assignees_nodes(
                IssuesQuerySearchNodesOnIssueAssigneesNodesParam {
                    login: Some(assignee_b.clone()),
                },
            ),
        ];
        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    assignees: Some(get_dummy_issues_query_search_nodes_on_issue_assignees(
                        IssuesQuerySearchNodesOnIssueAssigneesParam { assignees },
                    )),
                    ..Default::default()
                },
            ),
        };

        let want = Some(vec![assignee_a.clone(), assignee_b.clone()]);
        let got = issue.assignees();
        assert_eq!(want, got);
    }

    #[test]
    fn test_participants() {
        let participant_a = "a".to_string();
        let participant_b = "b".to_string();
        let participants = vec![
            get_dummy_issues_query_search_nodes_on_issue_participants_nodes(
                IssuesQuerySearchNodesOnIssueParticipantsNodesParam {
                    login: Some(participant_a.clone()),
                },
            ),
            get_dummy_issues_query_search_nodes_on_issue_participants_nodes(
                IssuesQuerySearchNodesOnIssueParticipantsNodesParam {
                    login: Some(participant_b.clone()),
                },
            ),
        ];
        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    participants: Some(get_dummy_issues_query_search_nodes_on_issue_participants(
                        IssuesQuerySearchNodesOnIssueParticipantsParam { participants },
                    )),
                    ..Default::default()
                },
            ),
        };

        let want = Some(vec![participant_a.clone(), participant_b.clone()]);
        let got = issue.participants();
        assert_eq!(want, got);
    }

    #[test]
    fn test_created_at() {
        let want: DateTime = (&chrono::Utc::now()).into();
        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    created_at: Some(want.clone()),
                    ..Default::default()
                },
            ),
        };

        let got = issue.created_at();
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count() {
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

        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                        IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                    )),
                    ..Default::default()
                },
            ),
        };

        let want = 2;
        let got = issue.comments_count();
        assert_eq!(want, got);
    }

    #[test]
    fn test_comments_count_by() {
        let by_name = "by".to_string();
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
                                login: Some(by_name.clone()),
                            },
                        ),
                    ),
                },
            ),
        ];

        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    comments: Some(get_dummy_issues_query_search_nodes_on_issue_comments(
                        IssuesQuerySearchNodesOnIssueCommentsParam { comments },
                    )),
                    ..Default::default()
                },
            ),
        };

        let want = 1;
        let got = issue.comments_count_by(&by_name);
        assert_eq!(want, got);
    }

    #[test]
    fn test_closed_at() {
        let want: DateTime = (&chrono::Utc::now()).into();
        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    closed_at: Some(want.clone()),
                    ..Default::default()
                },
            ),
        };

        let got = issue.closed_at();
        assert_eq!(want, got.unwrap());
    }

    #[test]
    fn test_closed_at_by() {
        let by_name = "by".to_string();
        let want: DateTime = (&chrono::Utc::now()).into();
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
                    created_at: Some(want.clone()),
                },
            ),
        ];

        let issue = Issue {
            inner: get_dummy_issues_query_search_nodes_on_issue(
                IssuesQuerySearchNodesOnIssueParam {
                    timeline_items: Some(
                        get_dummy_issues_query_search_nodes_on_issue_timeline_items(
                            IssuesQuerySearchNodesOnIssueTimelineItemsParam { timeline_items },
                        ),
                    ),
                    ..Default::default()
                },
            ),
        };

        let got = issue.closed_at_by(&by_name);
        assert_eq!(want, got.unwrap());
    }
}
