#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use num::FromPrimitive;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl FromPrimitive for ResultCode {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(ResultCode::ANEURALNETWORKS_NO_ERROR),
            1 => Some(ResultCode::ANEURALNETWORKS_OUT_OF_MEMORY),
            2 => Some(ResultCode::ANEURALNETWORKS_INCOMPLETE),
            3 => Some(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL),
            4 => Some(ResultCode::ANEURALNETWORKS_BAD_DATA),
            5 => Some(ResultCode::ANEURALNETWORKS_OP_FAILED),
            6 => Some(ResultCode::ANEURALNETWORKS_BAD_STATE),
            7 => Some(ResultCode::ANEURALNETWORKS_UNMAPPABLE),
            8 => Some(ResultCode::ANEURALNETWORKS_OUTPUT_INSUFFICIENT_SIZE),
            9 => Some(ResultCode::ANEURALNETWORKS_UNAVAILABLE_DEVICE),
            10 => Some(ResultCode::ANEURALNETWORKS_MISSED_DEADLINE_TRANSIENT),
            11 => Some(ResultCode::ANEURALNETWORKS_MISSED_DEADLINE_PERSISTENT),
            12 => Some(ResultCode::ANEURALNETWORKS_RESOURCE_EXHAUSTED_TRANSIENT),
            13 => Some(ResultCode::ANEURALNETWORKS_RESOURCE_EXHAUSTED_PERSISTENT),
            14 => Some(ResultCode::ANEURALNETWORKS_DEAD_OBJECT),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}
