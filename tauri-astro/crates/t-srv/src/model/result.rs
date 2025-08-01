use std::{error::Error, mem};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Status {
    Ok = 0,
    Fail,
}

impl Status {
    pub const MAX: u8 = (mem::variant_count::<Self>() - 1) as u8;
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Safe: std::mem::discriminant(_)
        let code = unsafe { *<*const Self>::from(self).cast::<u8>() };
        Serialize::serialize(&code, serializer)
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            code @ 0..=Self::MAX => {
                // Safe: Status in (0..=MAX)
                let status = unsafe { mem::transmute(code) };
                Ok(status)
            },
            rest => Err(de::Error::custom(format!("Invalid Status: {rest} > {}", Self::MAX))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T> {
    pub status: Status,
    pub data: Option<T>,
    pub cause: Option<String>,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Self {
        Self { status: Status::Ok, data: Some(data), cause: None }
    }

    pub fn fail<E>(err: E) -> Self
    where
        E: Error,
    {
        Self { status: Status::Fail, data: None, cause: Some(err.to_string()) }
    }
}

impl<T, E> From<Result<T, E>> for Data<T>
where
    E: Error,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(data) => Self::new(data),
            Err(err) => Self::fail(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use nill::{Nil, nil};
    use t_lib::error::Result;

    use super::*;

    #[test]
    fn test_status() -> Result<Nil> {
        use serde_json::{from_str, to_string};

        let code_ok = to_string(&Status::Ok)?;
        assert_eq!(code_ok, "0");

        let stat_ok: Status = from_str(&code_ok)?;
        assert_eq!(stat_ok, Status::Ok);

        let code_fail = to_string(&Status::Fail)?;
        assert_eq!(code_fail, "1");

        let stat_fail: Status = from_str(&code_fail)?;
        assert_eq!(stat_fail, Status::Fail);

        let code = format!("{}", Status::MAX + 1);
        let invalid = from_str::<Status>(&code);
        assert!(invalid.is_err());

        Ok(nil)
    }
}
