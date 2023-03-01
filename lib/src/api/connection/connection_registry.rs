use std::{collections::HashSet, fmt::Debug, sync::Arc};

use async_trait::async_trait;

use super::{connection::Connection, ConnectionDescriptor};

#[async_trait]
pub trait ConnectionRegistry {
    type ConnectionType: Connection + Send + Sync;
    type DescriptorType: ConnectionDescriptor + Debug + Send + Sync;

    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>>;

    async fn connection(
        &self,
        mac_address: &str,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>>;
}
