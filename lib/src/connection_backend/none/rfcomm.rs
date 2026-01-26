use std::collections::HashSet;

use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};

use crate::api::connection::{
    self, RfcommBackend, RfcommConnection, RfcommServiceSelectionStrategy,
};

pub struct NoneRfcommBackend {}
pub struct NoneRfcommConnection {}
impl RfcommBackend for NoneRfcommBackend {
    type ConnectionType = NoneRfcommConnection;

    async fn devices(&self) -> connection::Result<HashSet<connection::ConnectionDescriptor>> {
        unimplemented!()
    }

    async fn connect(
        &self,
        _mac_address: MacAddr6,
        _select_uuid: RfcommServiceSelectionStrategy,
    ) -> connection::Result<Self::ConnectionType> {
        unimplemented!()
    }
}
impl RfcommConnection for NoneRfcommConnection {
    async fn write(&self, _data: &[u8]) -> connection::Result<()> {
        unimplemented!()
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        unimplemented!()
    }

    fn connection_status(&self) -> watch::Receiver<connection::ConnectionStatus> {
        unimplemented!()
    }
}
