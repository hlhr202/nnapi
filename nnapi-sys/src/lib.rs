#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use num::FromPrimitive;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl FromPrimitive for ResultCode {
    fn from_i64(n: i64) -> Option<Self> {
        let n = n as i32;
        ResultCode::from_i32(n)
    }

    fn from_u64(n: u64) -> Option<Self> {
        let n = n as i32;
        ResultCode::from_i32(n)
    }
}
