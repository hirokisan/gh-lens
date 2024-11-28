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
