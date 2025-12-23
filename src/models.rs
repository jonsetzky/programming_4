use chrono::{DateTime, Utc};
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    prelude::*,
    sql_types::Binary,
    sqlite::Sqlite,
};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    id: UuidWrapper,
    channel: UuidWrapper,
    sender: UuidWrapper,
    // reply_to: Uuid,
    time: DateTime<Utc>,
    message: String,
}

#[derive(FromSqlRow)]
pub struct UuidWrapper(pub Uuid);

impl FromSql<Binary, Sqlite> for UuidWrapper {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        unsafe {
            let res: [u8; 16] = match <*const [u8]>::from_sql(bytes) {
                Ok(r) => *r
                    .as_array::<16>()
                    .expect("invalid array size when deserializing uuid"),
                Err(err) => return Err(err),
            };
            Ok(UuidWrapper(Uuid::from_bytes(res)))
        }
    }
}

impl Message {
    // pub fn time(&self) -> DateTime<Utc> {
    //     DateTime::from_timestamp(self.time, 0).unwrap()
    // }
}
