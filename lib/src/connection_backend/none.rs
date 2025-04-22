mod rfcomm;

use crate::api::connection;

use super::ConnectionBackends;

pub struct NoneConnectionBackends {}
impl ConnectionBackends for NoneConnectionBackends {
    type Rfcomm = rfcomm::NoneRfcommBackend;
    async fn rfcomm(&self) -> connection::Result<Self::Rfcomm> {
        unimplemented!()
    }
}
