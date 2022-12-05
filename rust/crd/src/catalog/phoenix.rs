use super::commons::{HbaseConnection, ZookeeperConnection};
use serde::{Deserialize, Serialize};
use stackable_operator::schemars::{self, JsonSchema};

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoenixConnector {
    /// Connection to an HBase cluster
    pub hbase: HbaseConnection,
    /// Connection to an Zookeeper cluster
    pub zookeeper: ZookeeperConnection,
}
