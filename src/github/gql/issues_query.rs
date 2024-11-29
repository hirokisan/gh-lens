use graphql_client::GraphQLQuery;

use super::scaler::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/gql/schema.json",
    query_path = "src/github/gql/issues_query.graphql",
    variables_derives = "Clone, Debug, PartialEq",
    response_derives = "Debug, Clone"
)]
pub struct IssuesQuery;

#[cfg(test)]
#[allow(dead_code)]
pub mod tests {
    use super::*;
    use crate::github::gql::scaler::tests::get_dummy_date_time;

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueParam {
        pub url: Option<String>,
        pub created_at: Option<DateTime>,
        pub closed_at: Option<DateTime>,
        pub author: Option<issues_query::IssuesQuerySearchNodesOnIssueAuthor>,
        pub comments: Option<issues_query::IssuesQuerySearchNodesOnIssueComments>,
        pub timeline_items: Option<issues_query::IssuesQuerySearchNodesOnIssueTimelineItems>,
        pub assignees: Option<issues_query::IssuesQuerySearchNodesOnIssueAssignees>,
        pub participants: Option<issues_query::IssuesQuerySearchNodesOnIssueParticipants>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue(
        param: IssuesQuerySearchNodesOnIssueParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssue {
        issues_query::IssuesQuerySearchNodesOnIssue {
            url: param.url.unwrap_or_else(|| "".to_string()),
            created_at: param.created_at.unwrap_or_else(|| get_dummy_date_time()),
            closed_at: param.closed_at,
            author: param.author,
            comments: param.comments.unwrap_or_else(|| {
                get_dummy_issues_query_search_nodes_on_issue_comments(
                    IssuesQuerySearchNodesOnIssueCommentsParam::default(),
                )
            }),
            timeline_items: param.timeline_items.unwrap_or_else(|| {
                get_dummy_issues_query_search_nodes_on_issue_timeline_items(
                    IssuesQuerySearchNodesOnIssueTimelineItemsParam::default(),
                )
            }),
            assignees: param.assignees.unwrap_or_else(|| {
                get_dummy_issues_query_search_nodes_on_issue_assignees(
                    IssuesQuerySearchNodesOnIssueAssigneesParam::default(),
                )
            }),
            participants: param.participants.unwrap_or_else(|| {
                get_dummy_issues_query_search_nodes_on_issue_participants(
                    IssuesQuerySearchNodesOnIssueParticipantsParam::default(),
                )
            }),
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueAuthorParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_author(
        param: IssuesQuerySearchNodesOnIssueAuthorParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueAuthor {
        issues_query::IssuesQuerySearchNodesOnIssueAuthor {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: issues_query::IssuesQuerySearchNodesOnIssueAuthorOn::User,
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueCommentsParam {
        pub comments: Vec<issues_query::IssuesQuerySearchNodesOnIssueCommentsNodes>,
    }

    impl IssuesQuerySearchNodesOnIssueCommentsParam {
        fn nodes(
            &self,
        ) -> Option<Vec<Option<issues_query::IssuesQuerySearchNodesOnIssueCommentsNodes>>> {
            if self.comments.len() == 0 {
                return None;
            };
            let nodes = self
                .comments
                .iter()
                .map(|comment| Some(comment.clone()))
                .collect();
            Some(nodes)
        }
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_comments(
        param: IssuesQuerySearchNodesOnIssueCommentsParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueComments {
        issues_query::IssuesQuerySearchNodesOnIssueComments {
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueCommentsNodesParam {
        pub author: Option<issues_query::IssuesQuerySearchNodesOnIssueCommentsNodesAuthor>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_comments_nodes(
        param: IssuesQuerySearchNodesOnIssueCommentsNodesParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueCommentsNodes {
        issues_query::IssuesQuerySearchNodesOnIssueCommentsNodes {
            author: param.author,
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_comments_nodes_author(
        param: IssuesQuerySearchNodesOnIssueCommentsNodesAuthorParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueCommentsNodesAuthor {
        issues_query::IssuesQuerySearchNodesOnIssueCommentsNodesAuthor {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: issues_query::IssuesQuerySearchNodesOnIssueCommentsNodesAuthorOn::User,
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueTimelineItemsParam {
        pub timeline_items: Vec<issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodes>,
    }

    impl IssuesQuerySearchNodesOnIssueTimelineItemsParam {
        fn nodes(
            &self,
        ) -> Option<Vec<Option<issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodes>>>
        {
            if self.timeline_items.len() == 0 {
                return None;
            };
            let nodes = self
                .timeline_items
                .iter()
                .map(|item| Some(item.clone()))
                .collect();
            Some(nodes)
        }
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_timeline_items(
        param: IssuesQuerySearchNodesOnIssueTimelineItemsParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueTimelineItems {
        issues_query::IssuesQuerySearchNodesOnIssueTimelineItems {
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueTimelineItemsNodesClosedEventParam {
        pub actor:
            Option<issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActor>,
        pub created_at: Option<DateTime>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_timeline_items_nodes_closed_event(
        param: IssuesQuerySearchNodesOnIssueTimelineItemsNodesClosedEventParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodes {
        let event = issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEvent {
            actor: param.actor,
            created_at: param.created_at.unwrap_or_else(|| get_dummy_date_time()),
        };
        issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodes::ClosedEvent(event)
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActorParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_timeline_items_nodes_closed_event_actor(
        param: IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActorParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActor {
        issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActor {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: issues_query::IssuesQuerySearchNodesOnIssueTimelineItemsNodesOnClosedEventActorOn::User,
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueAssigneesParam {
        pub assignees: Vec<issues_query::IssuesQuerySearchNodesOnIssueAssigneesNodes>,
    }

    impl IssuesQuerySearchNodesOnIssueAssigneesParam {
        fn nodes(
            &self,
        ) -> Option<Vec<Option<issues_query::IssuesQuerySearchNodesOnIssueAssigneesNodes>>>
        {
            if self.assignees.len() == 0 {
                return None;
            };
            let nodes = self
                .assignees
                .iter()
                .map(|item| Some(item.clone()))
                .collect();
            Some(nodes)
        }
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_assignees(
        param: IssuesQuerySearchNodesOnIssueAssigneesParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueAssignees {
        issues_query::IssuesQuerySearchNodesOnIssueAssignees {
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueAssigneesNodesParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_assignees_nodes(
        param: IssuesQuerySearchNodesOnIssueAssigneesNodesParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueAssigneesNodes {
        issues_query::IssuesQuerySearchNodesOnIssueAssigneesNodes {
            login: param.login.unwrap_or_else(|| "".to_string()),
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueParticipantsParam {
        pub participants: Vec<issues_query::IssuesQuerySearchNodesOnIssueParticipantsNodes>,
    }

    impl IssuesQuerySearchNodesOnIssueParticipantsParam {
        fn nodes(
            &self,
        ) -> Option<Vec<Option<issues_query::IssuesQuerySearchNodesOnIssueParticipantsNodes>>>
        {
            if self.participants.len() == 0 {
                return None;
            };
            let nodes = self
                .participants
                .iter()
                .map(|item| Some(item.clone()))
                .collect();
            Some(nodes)
        }
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_participants(
        param: IssuesQuerySearchNodesOnIssueParticipantsParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueParticipants {
        issues_query::IssuesQuerySearchNodesOnIssueParticipants {
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct IssuesQuerySearchNodesOnIssueParticipantsNodesParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_issues_query_search_nodes_on_issue_participants_nodes(
        param: IssuesQuerySearchNodesOnIssueParticipantsNodesParam,
    ) -> issues_query::IssuesQuerySearchNodesOnIssueParticipantsNodes {
        issues_query::IssuesQuerySearchNodesOnIssueParticipantsNodes {
            login: param.login.unwrap_or_else(|| "".to_string()),
        }
    }
}
