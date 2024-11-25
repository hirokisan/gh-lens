use graphql_client::GraphQLQuery;

use super::scaler::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/gql/schema.json",
    query_path = "src/github/gql/pull_requests_query.graphql",
    variables_derives = "Clone, Debug, PartialEq",
    response_derives = "Debug, Clone"
)]
pub struct PullRequestsQuery;

#[cfg(test)]
#[allow(dead_code)]
pub mod tests {
    use super::*;
    use crate::github::gql::scaler::tests::get_dummy_date_time;

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestParam {
        pub created_at: Option<DateTime>,
        pub merged_at: Option<DateTime>,
        pub url: Option<String>,
        pub total_comments_count: Option<i64>,
        pub author: Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthor>,
        pub merged_by:
            Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestMergedBy>,
        pub commits: Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommits>,
        pub changed_files: Option<i64>,
        pub comments:
            Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestComments>,
        pub reviews: Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviews>,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request(
        param: PullRequestsQuerySearchNodesOnPullRequestParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequest {
            created_at: param.created_at.unwrap_or_else(|| get_dummy_date_time()),
            merged_at: param.merged_at,
            url: param.url.unwrap_or_else(|| "".to_string()),
            total_comments_count: param.total_comments_count,
            author: param.author,
            merged_by: param.merged_by,
            commits: param.commits.unwrap_or_else(|| {
                get_dummy_pull_requests_query_search_nodes_on_pull_request_commits(
                    PullRequestsQuerySearchNodesOnPullRequestCommitsParam::default(),
                )
            }),
            changed_files: param.changed_files.unwrap_or_else(|| 0),
            comments: param.comments.unwrap_or_else(|| {
                get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
                    PullRequestsQuerySearchNodesOnPullRequestCommentsParam::default(),
                )
            }),
            reviews: param.reviews,
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestCommitsParam {
        pub total_count: Option<i64>,
        pub commits:
            Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommitsNodesCommit>,
    }

    impl PullRequestsQuerySearchNodesOnPullRequestCommitsParam {
        fn nodes(
            &self,
        ) -> Option<
            Vec<Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommitsNodes>>,
        > {
            if self.commits.len() == 0 {
                return None;
            };
            let nodes = self
                .commits
                .iter()
                .map(|commit| {
                    Some(pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommitsNodes{
                        commit: commit.clone(),
                    })
                })
                .collect();
            Some(nodes)
        }
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_commits(
        param: PullRequestsQuerySearchNodesOnPullRequestCommitsParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommits {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommits {
            total_count: param.total_count.unwrap_or_else(|| 0),
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestCommentsParam {
        pub comments:
            Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes>,
    }

    impl PullRequestsQuerySearchNodesOnPullRequestCommentsParam {
        fn nodes(
            &self,
        ) -> Option<
            Vec<
                Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes>,
            >,
        > {
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

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam {
        pub created_at: Option<DateTime>,
        pub author: Option<
            pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthor,
        >,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes(
        param: PullRequestsQuerySearchNodesOnPullRequestCommentsNodesParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodes {
            created_at: param.created_at.unwrap_or_else(|| get_dummy_date_time()),
            author: param.author,
        }
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_comments(
        param: PullRequestsQuerySearchNodesOnPullRequestCommentsParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestComments {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestComments {
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_comments_nodes_author(
        param: PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthor {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthor {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestCommentsNodesAuthorOn::User,
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
        pub reviews:
            Vec<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>,
    }

    impl PullRequestsQuerySearchNodesOnPullRequestReviewsParam {
        fn nodes(
            &self,
        ) -> Option<
            Vec<Option<pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes>>,
        > {
            if self.reviews.len() == 0 {
                return None;
            };
            let nodes = self
                .reviews
                .iter()
                .map(|review| Some(review.clone()))
                .collect();
            Some(nodes)
        }
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews(
        param: PullRequestsQuerySearchNodesOnPullRequestReviewsParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviews {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviews {
            nodes: param.nodes(),
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam {
        pub body: Option<String>,
        pub created_at: Option<DateTime>,
        pub author: Option<
            pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthor,
        >,
        pub state: Option<pull_requests_query::PullRequestReviewState>,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes(
        param: PullRequestsQuerySearchNodesOnPullRequestReviewsNodesParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodes {
            body: param.body.unwrap_or_else(|| "".to_string()),
            author: param.author,
            created_at: param.created_at.unwrap_or_else(|| get_dummy_date_time()),
            state: param
                .state
                .unwrap_or_else(|| pull_requests_query::PullRequestReviewState::COMMENTED),
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_reviews_nodes_author(
        param: PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthor {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthor {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestReviewsNodesAuthorOn::User,
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestAuthorParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_author(
        param: PullRequestsQuerySearchNodesOnPullRequestAuthorParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthor {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthor {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestAuthorOn::User,
        }
    }

    #[derive(Default)]
    pub struct PullRequestsQuerySearchNodesOnPullRequestMergedByParam {
        pub login: Option<String>,
    }

    pub fn get_dummy_pull_requests_query_search_nodes_on_pull_request_merged_by(
        param: PullRequestsQuerySearchNodesOnPullRequestMergedByParam,
    ) -> pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestMergedBy {
        pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestMergedBy {
            login: param.login.unwrap_or_else(|| "".to_string()),
            on: pull_requests_query::PullRequestsQuerySearchNodesOnPullRequestMergedByOn::User,
        }
    }
}
