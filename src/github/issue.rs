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
        let result: Vec<String> = self
            .inner
            .assignees
            .nodes
            .as_ref()
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
        let result: Vec<String> = self
            .inner
            .participants
            .nodes
            .as_ref()
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
        let mut timeline_items = vec![];
        for item in self
            .inner
            .timeline_items
            .nodes
            .as_ref()
            .unwrap()
            .iter()
            .flatten()
        {
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
