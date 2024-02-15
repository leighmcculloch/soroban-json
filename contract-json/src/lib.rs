#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Bytes, Env, IntoVal, String};

#[contract]
pub struct Contract;

#[contracterror]
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Error {
    JsonDecodeError = 1,
}

#[derive(serde::Deserialize)]
struct Data<'a> {
    pub challenge: &'a str,
}

#[contractimpl]
impl Contract {
    /// Extract the 'field' out of the JSON.
    pub fn extract(e: Env, json: Bytes) -> Result<String, Error> {
        let json = json.to_buffer::<1024>();
        let json = json.as_slice();
        let (data, _): (Data, _) =
            serde_json_core::de::from_slice(json).map_err(|_| Error::JsonDecodeError)?;
        Ok(data.challenge.into_val(&e))
    }
}
