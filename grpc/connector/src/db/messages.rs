use diesel::{update, delete, insert_into, QueryDsl, RunQueryDsl, QueryResult};

use crate::db::models::{InputInsertMessage, Message, NewMessage};
use crate::db::Pool;
use crate::db::schema::messages::dsl::messages;
use crate::util::datetime;


/// ユーザーの存在確認はしません。
pub fn get_single_message(pool_clone: Pool, message_id: String) -> Result<Message, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    messages.find(message_id).get_result(&conn)
}


pub fn insert_single_message(pool_clone: Pool, input_message: InputInsertMessage) -> Result<Message, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();

    let new_message = NewMessage {
        id: input_message.id,
        from: input_message.from,
        to: input_message.to,
        content_type: input_message.content_type,
        metadata: input_message.metadata,
        text: input_message.text,
        created_at: &datetime::now(),
        updated_at: &datetime::now()
    };

    let res = insert_into(messages).values(&new_message).get_result(&conn)?;
    Ok(res)
}