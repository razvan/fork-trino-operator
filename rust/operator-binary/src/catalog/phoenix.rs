use super::{config::CatalogConfig, ExtendCatalogConfig, FromTrinoCatalogError, ToCatalogConfig};
use async_trait::async_trait;
use stackable_operator::client::Client;
use stackable_trino_crd::catalog::phoenix::PhoenixConnector;

pub const CONNECTOR_NAME: &str = "phoenix5";

#[async_trait]
impl ToCatalogConfig for PhoenixConnector {
    async fn to_catalog_config(
        &self,
        catalog_name: &str,
        catalog_namespace: Option<String>,
        client: &Client,
    ) -> Result<CatalogConfig, FromTrinoCatalogError> {
        let mut config = CatalogConfig::new(catalog_name.to_string(), CONNECTOR_NAME);

        self.hbase
            .extend_catalog_config(&mut config, catalog_name, catalog_namespace.clone(), client)
            .await?;

        config.add_property("phoenix.connection-url", "jdbc:phoenix:zookeeper-server-default-0.zookeeper-server-default.gdpr.svc.cluster.local:2282:/znode-d6933fb5-a660-4fe7-802e-7770877663b2/hbase");

        Ok(config)
    }
}
