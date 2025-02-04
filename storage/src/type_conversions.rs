use std::str::FromStr;

use macaddr::MacAddr6;
use openscq30_lib::soundcore_device::device_model::DeviceModel;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
    ToSql,
};

pub struct SqliteMacAddr6(pub MacAddr6);
impl FromSql for SqliteMacAddr6 {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(SqliteMacAddr6(
            MacAddr6::from_str(value.as_str()?)
                .map_err(|err| FromSqlError::Other(Box::new(err)))?,
        ))
    }
}
impl ToSql for SqliteMacAddr6 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.0.to_string())))
    }
}

pub struct SqliteDeviceModel(pub DeviceModel);
impl FromSql for SqliteDeviceModel {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(SqliteDeviceModel(
            DeviceModel::from_str(value.as_str()?)
                .map_err(|err| FromSqlError::Other(Box::new(err)))?,
        ))
    }
}
impl ToSql for SqliteDeviceModel {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let text: &'static str = self.0.into();
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(text.as_bytes())))
    }
}
