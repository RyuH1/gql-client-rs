use serde::{Deserialize, de::Error};

#[derive(Deserialize, Debug)]
pub struct NodeList<T> {
  pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
  pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct SinglePost {
  pub post: Post,
}

#[derive(Deserialize, Debug)]
pub struct AllPosts {
  pub posts: NodeList<Post>,
}

pub mod inputs {
  use serde::Serialize;

  #[derive(Serialize, Debug)]
  pub struct SinglePostVariables {
    pub id: u32,
  }

  #[derive(Serialize, Debug)]
  pub struct SingleProposalVariables {
    pub current: String,
  }
}

//////////////////////////////////
/// AnyDAO

#[derive(Deserialize, Debug)]
pub enum Protocol {
  Substrate,
  Solidity
}

#[derive(Deserialize, Debug)]
pub enum Privacy {
  Opaque,
  Private,
  Public,
  Mixed
}

#[derive(Deserialize, Debug)]
pub enum Status {
  Pending,
  Ongoing,
  Closed
}

#[derive(Deserialize, Debug)]
pub struct CrossChainAccount {
  pub id: String,
  pub protocol: Protocol
}

#[derive(Deserialize, Debug)]
pub struct Project {
  pub id: String,
  pub owner: CrossChainAccount,
  pub data: String,
  pub updated: u32
}

#[derive(Deserialize, Debug)]
pub struct Proposal {
  pub id: String,
  pub project: Project,
  pub proposal: u32,
  #[serde(deserialize_with = "deserialize_u64")]
  pub start: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub end: u64,
  pub author: CrossChainAccount,
  pub privacy: Privacy,
  pub frequency: Option<String>,
  pub status: Status,
  #[serde(deserialize_with = "deserialize_vec")]
  pub votes: Vec<u64>,
  pub pubvote: Option<String>,
  pub created: u32
}

#[derive(Deserialize, Debug)]
pub struct MyNodeList<T> {
  pub nodes: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct AllProposals {
  pub proposals: MyNodeList<Proposal>,
}

pub fn deserialize_u64<'a, D>(d: D) -> Result<u64, D::Error>
where
	D: serde::Deserializer<'a>,
{
  let s: String = Deserialize::deserialize(d)?;
  Ok(s.parse::<u64>().map_err(Error::custom)?)
}

pub fn deserialize_vec<'a, D>(d: D) -> Result<Vec<u64>, D::Error>
where
	D: serde::Deserializer<'a>,
{
  let s: String = Deserialize::deserialize(d)?;

  Ok(s.split(',').map(|s| s.parse::<u64>().unwrap_or_default()).collect())
}