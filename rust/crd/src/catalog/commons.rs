use serde::{Deserialize, Serialize};
use stackable_operator::schemars::{self, JsonSchema};

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetastoreConnection {
    /// Name of the discovery-configmap providing information about the Hive metastore
    pub config_map: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HdfsConnection {
    /// Name of the discovery-configmap providing information about the HDFS cluster
    pub config_map: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HbaseConnection {
    /// Name of the discovery-configmap providing information about the Hbase cluster
    pub config_map: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZookeeperConnection {
    /// Name of the discovery-configmap providing information about the Zookeeper Znode
    pub config_map: String,
}
