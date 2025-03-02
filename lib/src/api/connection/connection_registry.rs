use std::{collections::HashSet, fmt::Debug, sync::Arc};

use macaddr::MacAddr6;

use super::{ConnectionDescriptor, connection::Connection};

pub trait ConnectionRegistry {
    type ConnectionType: Connection + Send + Sync;
    type DescriptorType: ConnectionDescriptor + Debug + Send + Sync;

    fn connection_descriptors(
        &self,
    ) -> impl Future<Output = crate::Result<HashSet<Self::DescriptorType>>> + Send + Sync;

    fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> impl Future<Output = crate::Result<Option<Arc<Self::ConnectionType>>>> + Send + Sync;
}
