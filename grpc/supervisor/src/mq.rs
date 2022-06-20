use serde_derive::{Serialize, Deserialize};
use pulsar::{
    message::proto, producer, Error as PulsarError, Pulsar, SerializeMessage, TokioExecutor,
};
use std::default::Default;


#[derive(Serialize, Deserialize)]
pub struct Notice {
    pub(crate) data: String
}

impl <'a> SerializeMessage for &'a Notice {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}
