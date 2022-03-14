mod structs;

use crate::structs::{inputs::{SinglePostVariables, SingleProposalVariables}, AllPosts, SinglePost, AllProposals};
use gql_client::Client;
use std::collections::HashMap;

// Initialize endpoint
const ENDPOINT: &'static str = "https://api.subquery.network/sq/RyuH1/fst_anydao__Unl1S";

#[tokio::test]
pub async fn fetches_one_post() {
  let client = Client::new(ENDPOINT);

  let query = r#"
    query SinglePostQuery($id: ID!) {
      post(id: $id) {
        id
      }
    }
  "#;

  let variables = SinglePostVariables { id: 2 };
  let data = client
    .query_with_vars_unwrap::<SinglePost, SinglePostVariables>(query, variables)
    .await
    .unwrap();

  assert_eq!(data.post.id, String::from("2"), "Post id retrieved 2");
}

#[tokio::test]
pub async fn fetches_all_posts() {
  let mut headers = HashMap::new();
  headers.insert("content-type", "application/json");

  let client = Client::new_with_headers(ENDPOINT, headers);

  let query = r#"
    query AllPostsQuery {
      posts {
        data {
          id
        }
      }
    }
  "#;

  let data: AllPosts = client.query_unwrap::<AllPosts>(query).await.unwrap();

  assert!(data.posts.data.len() > 0 as usize);
}

#[tokio::test]
pub async fn fetches_all_projects() {
  let mut headers = HashMap::new();
  headers.insert("content-type", "application/json");

  let client = Client::new_with_headers(ENDPOINT, headers);

  let query = r#"
    query AllProjectsQuery {
      proposals {
        nodes {
          id
          project {
            id
            owner {
              id
              protocol
            }
            data
            updated
          }
          proposal
          start
          end
          author {
            id
            protocol
          }
          privacy
          frequency
          status
          votes
          pubvote
          created
        }
      }
    }
  "#;

  let data: AllProposals = client.query_unwrap::<AllProposals>(query).await.unwrap();

  assert!(data.proposals.nodes.len() > 0 as usize);

  println!("data: {:?}", data);
}

#[tokio::test]
pub async fn fetches_all_projects_with_filter() {
  let mut headers = HashMap::new();
  headers.insert("content-type", "application/json");

  let client = Client::new_with_headers(ENDPOINT, headers);

  let query = r#"
    query AllProjectsQuery($current: BigFloat!) {
      proposals(filter: {end: {greaterThanOrEqualTo: $current}}) {
        nodes {
          id
          project {
            id
            owner {
              id
              protocol
            }
            data
            updated
          }
          proposal
          start
          end
          author {
            id
            protocol
          }
          privacy
          frequency
          status
          votes
          pubvote
          created
        }
      }
    }
  "#;

  let variables = SingleProposalVariables { current: "1646841600000".to_string() };
  let data: AllProposals = client.query_with_vars_unwrap::<AllProposals, SingleProposalVariables>(query, variables).await.unwrap();

  assert!(data.proposals.nodes.len() > 0 as usize);

  println!("data: {:?}", data);
}

#[tokio::test]
pub async fn fetches_all_projects_with_status_filter() {
  let mut headers = HashMap::new();
  headers.insert("content-type", "application/json");

  let client = Client::new_with_headers(ENDPOINT, headers);

  let query = r#"
    query AllProjectsQuery {
      proposals(filter: {status: {notEqualTo: Closed}}) {
        nodes {
          id
          project {
            id
            owner {
              id
              protocol
            }
            data
            updated
          }
          proposal
          start
          end
          author {
            id
            protocol
          }
          privacy
          frequency
          status
          votes
          pubvote
          created
        }
      }
    }
  "#;

  let data: AllProposals = client.query_unwrap::<AllProposals>(query).await.unwrap();

  assert!(data.proposals.nodes.len() > 0 as usize);

  println!("data: {:?}", data);
}