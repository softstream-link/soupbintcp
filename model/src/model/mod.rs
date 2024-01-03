pub mod debug;
pub mod payload;
pub mod sequenced_data;
pub mod soup_bin;
pub mod types;
pub mod unsequenced_data;

pub mod clt;
pub mod svc;

pub mod sample_payload;

use byteserde::prelude::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use std::fmt::Debug;

pub trait SoupBinTcpPayload<P>: Debug + Clone + Send + Sync + 'static + ByteSerializeStack + ByteSerializedLenOf + ByteDeserializeSlice<P> + PartialEq {}
