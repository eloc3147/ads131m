use crate::int::i24;

#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeTuple;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

/// A single ADC sample grab for all channels
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleGrab<const CHANNELS: usize> {
    pub(crate) data: [[u8; 3]; CHANNELS],
}

impl<const CHANNELS: usize> SampleGrab<CHANNELS> {
    /// Get a reference to the underlying sample bytes
    #[must_use]
    pub const fn as_bytes(&self) -> &[[u8; 3]; CHANNELS] {
        &self.data
    }

    /// Extract the underlying sample bytes
    #[must_use]
    pub const fn to_bytes(self) -> [[u8; 3]; CHANNELS] {
        self.data
    }

    /// Convert the sample data into an array of signed 24-bit integers
    #[must_use]
    pub fn into_i24_array(self) -> [i24; CHANNELS] {
        let mut values = [i24::new_masked(0); CHANNELS];
        for (idx, value) in values.iter_mut().enumerate() {
            *value = i24::from_be_bytes(self.data[idx]);
        }

        values
    }

    /// Convert the sample data into an array of signed 32-bit integers
    #[must_use]
    pub fn into_i32_array(self) -> [i32; CHANNELS] {
        let mut values = [0; CHANNELS];
        for (idx, value) in values.iter_mut().enumerate() {
            *value = i24::from_be_bytes(self.data[idx]).get();
        }

        values
    }

    /// Convert the sample data into floating point numbers between -1 and 1
    #[must_use]
    pub fn into_floats(self) -> [f64; CHANNELS] {
        let mut values = [0.0; CHANNELS];
        for (idx, value) in values.iter_mut().enumerate() {
            let int = i24::from_be_bytes(self.data[idx]).get();
            if int < 0 {
                *value = f64::from(int) / f64::from(-i24::MIN);
            } else {
                *value = f64::from(int) / f64::from(i24::MAX);
            }
        }

        values
    }
}

#[cfg(feature = "serde")]
struct SampleGrabVisitor<const CHANNELS: usize>;

#[cfg(feature = "serde")]
impl<'de, const CHANNELS: usize> Visitor<'de> for SampleGrabVisitor<CHANNELS> {
    type Value = SampleGrab<CHANNELS>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("an array of CHANNELS arrays of 3 ints")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut data = [[0; 3]; CHANNELS];
        let mut count = 0;

        for sample in &mut data {
            for byte in &mut sample[..] {
                *byte = seq.next_element()?.ok_or_else(|| {
                    A::Error::invalid_length(count, &"an array of CHANNELS arrays of 3 ints")
                })?;
                count += 1;
            }
        }

        Ok(SampleGrab { data })
    }
}

#[cfg(feature = "serde")]
impl<const CHANNELS: usize> Serialize for SampleGrab<CHANNELS> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(3 * CHANNELS)?;
        for sample in self.data {
            for byte in sample {
                seq.serialize_element(&byte)?;
            }
        }

        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, const CHANNELS: usize> Deserialize<'de> for SampleGrab<CHANNELS> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(3 * CHANNELS, SampleGrabVisitor::<CHANNELS>)
    }
}

#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn sample_to_ints() {
        assert_eq!(
            SampleGrab {
                data: [[0x7F, 0xFF, 0xFF]]
            }
            .into_i32_array(),
            [8_388_607]
        );
        assert_eq!(
            SampleGrab {
                data: [[0x00, 0x00, 0x01]]
            }
            .into_i32_array(),
            [1]
        );
        assert_eq!(
            SampleGrab {
                data: [[0x00, 0x00, 0x00]]
            }
            .into_i32_array(),
            [0]
        );
        assert_eq!(
            SampleGrab {
                data: [[0xFF, 0xFF, 0xFF]]
            }
            .into_i32_array(),
            [-1]
        );
        assert_eq!(
            SampleGrab {
                data: [[0x80, 0x00, 0x00]]
            }
            .into_i32_array(),
            [-8_388_608]
        );
    }

    #[test]
    fn sample_to_floats() {
        assert_approx_eq!(
            f64,
            SampleGrab {
                data: [[0x7F, 0xFF, 0xFF]]
            }
            .into_floats()[0],
            1.0,
            epsilon = 1e-8
        );
        assert_approx_eq!(
            f64,
            SampleGrab {
                data: [[0x00, 0x00, 0x01]]
            }
            .into_floats()[0],
            0.000_000_119_209,
            epsilon = 1e-8
        );
        assert_approx_eq!(
            f64,
            SampleGrab {
                data: [[0x00, 0x00, 0x00]]
            }
            .into_floats()[0],
            0.0,
            epsilon = 1e-8
        );
        assert_approx_eq!(
            f64,
            SampleGrab {
                data: [[0xFF, 0xFF, 0xFF]]
            }
            .into_floats()[0],
            -0.000_000_119_209,
            epsilon = 1e-8
        );
        assert_approx_eq!(
            f64,
            SampleGrab {
                data: [[0x80, 0x00, 0x00]]
            }
            .into_floats()[0],
            -1.0,
            epsilon = 1e-8
        );
    }
}
