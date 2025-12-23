use chrono::{DateTime, Utc};
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::ToSql,
    sql_types::Binary,
    sqlite::Sqlite,
};
use uuid::Uuid;

#[derive(FromSqlRow, Debug, AsExpression)]
#[diesel(sql_type = Binary)]
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

impl ToSql<Binary, Sqlite> for UuidWrapper {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        <[u8] as ToSql<Binary, Sqlite>>::to_sql(self.0.as_bytes(), out)
    }
}

// impl AsExpression<Binary> for UuidWrapper {
//     type Expression = [u8];
//     fn as_expression(self) -> Self::Expression {
//         self.0.as_bytes()
//     }
// }

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MessageModel {
    id: UuidWrapper,
    channel: UuidWrapper,
    sender: UuidWrapper,
    // reply_to: Uuid,
    time: DateTime<Utc>,
    message: String,
}
impl From<MessageModel> for crate::repository::Message {
    fn from(value: MessageModel) -> Self {
        crate::repository::Message {
            channel: value.channel.0,
            id: value.id.0,
            sender: value.sender.0,
            time: value.time,
            message: value.message,
        }
    }
}
impl From<crate::repository::Message> for MessageModel {
    fn from(value: crate::repository::Message) -> Self {
        Self {
            channel: UuidWrapper(value.channel),
            id: UuidWrapper(value.id),
            message: value.message,
            sender: UuidWrapper(value.sender),
            time: value.time,
        }
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChannelModel {
    id: UuidWrapper,
    name: String,
}
impl From<ChannelModel> for crate::repository::Channel {
    fn from(value: ChannelModel) -> Self {
        crate::repository::Channel {
            id: value.id.0,
            name: value.name,
        }
    }
}
impl From<crate::repository::Channel> for ChannelModel {
    fn from(value: crate::repository::Channel) -> Self {
        Self {
            id: UuidWrapper(value.id),
            name: value.name,
        }
    }
}

impl MessageModel {
    // pub fn time(&self) -> DateTime<Utc> {
    //     DateTime::from_timestamp(self.time, 0).unwrap()
    // }
}
