#![cfg_attr(not(feature = "std"), no_std)]

use postcard::take_from_bytes;
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use tokio_util::bytes::{Buf, BytesMut};
#[cfg(feature = "std")]
use tokio_util::codec::Decoder;

pub use postcard;

/// Root telemetry type.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Telemetry {
    pub rpm: u32,
}

#[cfg(feature = "std")]
pub struct PostCardCodec;

#[cfg(feature = "std")]
impl Decoder for PostCardCodec {
    type Item = Telemetry;
    type Error = std::io::Error;

    /// Munch some bytes in order to build a T
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match take_from_bytes::<Telemetry>(src) {
            Ok((telemetry, remainder)) => {
                src.advance(src.len() - remainder.len());
                Ok(Some(telemetry))
            }
            Err(_) => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use postcard::to_slice;
    use rstest::rstest;

    #[rstest]
    #[case::deserialize_nothing(0, None)]
    #[case::deserialize_default(1, Some(Telemetry::default()))]
    #[case::deserialize_default_with_remainder(2, Some(Telemetry::default()))]
    #[case::deserialize_default_with_remainder(4, Some(Telemetry::default()))]
    fn test_simple(#[case] index: usize, #[case] expect: Option<Telemetry>) {
        let mut buffer = [0; 10];
        // Serialize to buffer
        to_slice(&Telemetry::default(), &mut buffer).unwrap();

        // Deserialize through Decoder
        let res = PostCardCodec
            .decode(&mut BytesMut::from(&buffer[..index]))
            .unwrap();

        assert_eq!(res, expect);
    }
}
