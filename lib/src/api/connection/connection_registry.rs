use std::{collections::HashSet, fmt::Debug, future::Future, sync::Arc};

use macaddr::MacAddr6;

use crate::futures::{MaybeSend, MaybeSync};

use super::{connection::Connection, ConnectionDescriptor};

pub trait ConnectionRegistry {
    type ConnectionType: Connection + Send + Sync;
    type DescriptorType: ConnectionDescriptor + Debug + Send + Sync;

    fn connection_descriptors(
        &self,
    ) -> impl Future<Output = crate::Result<HashSet<Self::DescriptorType>>> + MaybeSend + MaybeSync;

    fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> impl Future<Output = crate::Result<Option<Arc<Self::ConnectionType>>>> + MaybeSend + MaybeSync;
}
