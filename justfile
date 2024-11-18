get-schema:
  graphql-client introspect-schema https://api.github.com/graphql --header "Authorization: bearer $GITHUB_TOKEN" --header "user-agent: rust-graphql-client" > ./src/github/gql/schema.json
