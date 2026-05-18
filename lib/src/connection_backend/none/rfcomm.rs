use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;

use crate::api::connection::{
    self, RfcommBackend, RfcommConnection, RfcommServiceSelectionStrategy,
};

pub struct NoneRfcommBackend;

#[async_trait]
impl RfcommBackend for NoneRfcommBackend {
    async fn devices(&self) -> connection::Result<HashSet<connection::ConnectionDescriptor>> {
        unimplemented!()
    }

    async fn connect(
        &self,
        _mac_address: MacAddr6,
        _select_uuid: RfcommServiceSelectionStrategy,
    ) -> connection::Result<Arc<dyn RfcommConnection + Send + Sync>> {
        unimplemented!()
    }
}
