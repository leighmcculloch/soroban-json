#![no_std]
use core::ops::Range;

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
    pub field: &'a str,
}

#[contractimpl]
impl Contract {
    /// Extract the 'field' out of the JSON.
    pub fn extract(e: Env, data: Bytes) -> Result<String, Error> {
        let (buf, range) = to_buffered_slice::<1024>(&data);
        let json = &buf[range];
        let (data, _): (Data, _) =
            serde_json_core::de::from_slice(json).map_err(|_| Error::JsonDecodeError)?;
        Ok(data.field.into_val(&e))
    }
}

fn to_buffered_slice<const B: usize>(b: &Bytes) -> ([u8; B], Range<usize>) {
    let mut buf = [0u8; B];
    let len = b.len() as usize;
    {
        let slice = &mut buf[0..len];
        b.copy_into_slice(slice);
    }
    (buf, 0..len)
}
