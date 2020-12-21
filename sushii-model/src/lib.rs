pub mod error;
pub mod keys;
pub mod model;
pub mod prelude;

pub use crate::error::{Error, Result};

use juniper::{Value, ParseScalarResult, ParseScalarValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct BigInt(i64);

#[juniper::graphql_scalar(description = "BigInt")]
impl<S> GraphQLScalar for BigInt 
where
    S: ScalarValue
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue) -> Option<BigInt> {
        v.as_scalar_value()
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u64>().ok())
        .map(|i| BigInt(i as i64))
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}

impl From<BigInt> for u64 {
    fn from(id: BigInt) -> u64 {
        id.0 as u64
    }
}

impl From<BigInt> for i64 {
    fn from(id: BigInt) -> i64 {
        id.0 as i64
    }
}
