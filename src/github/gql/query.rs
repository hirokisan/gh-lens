use graphql_client::GraphQLQuery;

use crate::github::gql::scaler::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/gql/schema.json",
    query_path = "src/github/gql/pull_requests_query.graphql",
    variables_derives = "Clone, Debug, PartialEq",
    response_derives = "Debug"
)]
pub struct PullRequestsQuery;
