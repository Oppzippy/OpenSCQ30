use std::{collections::HashSet, fmt::Debug, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;

use super::{connection::Connection, ConnectionDescriptor};

#[async_trait]
pub trait ConnectionRegistry {
    type ConnectionType: Connection + Send + Sync;
    type DescriptorType: ConnectionDescriptor + Debug + Send + Sync;

    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>>;

    async fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>>;
}
