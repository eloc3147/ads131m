//! Types for configuring an ADS131M registers

use enum_iterator::{self, Sequence};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use ux::{u10, u24, u4};

macro_rules! is_bit_set {
    ($word:ident, $bit:literal) => {
        ($word & (1 << $bit)) != 0
    };
}

/// SPI Word size configuration
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum WordLength {
    /// 16-bit words
    Bits16 = 0,

    /// 24-bit words
    ///
    /// This is the default word length
    #[default]
    Bits24 = 1,

    /// 32-bit words, with zero padding
    Bits32Zero = 2,

    /// 32-bit words with the sign extension for 24-bit ADC data
    Bits32Signed = 3,
}

/// CRC implementation used for device communication
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum CrcType {
    /// 16 bit CCITT.
    ///
    /// This is the default CRC mode
    #[default]
    Ccitt = 0,

    /// 16 bit ANSI
    Ansi = 1,
}

/// DRDY pin source selection
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum DrdySource {
    /// Most lagging enabled channel
    ///
    /// This is the default DRDY source
    #[default]
    MostLagging = 0,

    /// Logic OR of all the enabled channels
    LogicOr = 1,

    /// Most leading enabled channel
    #[num_enum(alternatives = [3])]
    MostLeading = 2,
}

/// DRDY state when conversion data is not available
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum DrdyNotReadyState {
    /// Logic high
    ///
    /// This is the default DRDY not ready state
    #[default]
    LogicHigh = 0,

    /// High impedance
    HighImpedance = 1,
}

/// DRDY state when conversion data is available
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum DrdyReadyState {
    /// Logic low
    ///
    /// This is the default DRDY not state
    #[default]
    LogicLow = 0,

    /// Low pulse with a fixed duration
    LowPulse = 1,
}

/// Oversampling mode configuration
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum OversamplingRatio {
    /// Oversampling ratio of 64
    #[num_enum(alternatives = [9..16])]
    Osr64 = 8,

    /// Oversampling ratio of 128
    Osr128 = 0,

    /// Oversampling ratio of 256
    Osr256 = 1,

    /// Oversampling ratio of 512
    Osr512 = 2,

    /// Oversampling ratio of 1024
    ///
    /// This is the default oversampling ratio
    #[default]
    Osr1024 = 3,

    /// Oversampling ratio of 2048
    Osr2048 = 4,

    /// Oversampling ratio of 4096
    Osr4096 = 5,

    /// Oversampling ratio of 8192
    Osr8192 = 6,

    /// Oversampling ratio of 16256
    Osr16256 = 7,
}

/// Power mode setting
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum PowerMode {
    /// Very low power mode
    VeryLowPower = 0,

    /// Low power mode
    LowPower = 1,

    /// High resolution mode
    ///
    /// This is the default power mode
    #[default]
    #[num_enum(alternatives = [3])]
    HighResolution = 2,
}

/// PGA gain setting
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum PgaGain {
    /// 1x gain
    ///
    /// This is the default gain setting
    #[default]
    Gain1 = 0,

    /// 2x gain
    Gain2 = 1,

    /// 4x gain
    Gain4 = 2,

    /// 8x gain
    Gain8 = 3,

    /// 16x gain
    Gain16 = 4,

    /// 32x gain
    Gain32 = 5,

    /// 64x gain
    Gain64 = 6,

    /// 128x gain
    Gain128 = 7,
}

/// Global chop delay selection
///
/// Delay in modulator clock periods before measurement begins
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum GlobalChopDelay {
    /// 2 modulator clock delay
    Delay2 = 0,

    /// 4 modulator clock delay
    Delay4 = 1,

    /// 8 modulator clock delay
    Delay8 = 2,

    /// 16 modulator clock delay
    ///
    /// This is the default global-chop delay
    #[default]
    Delay16 = 3,

    /// 32 modulator clock delay
    Delay32 = 4,

    /// 64 modulator clock delay
    Delay64 = 5,

    /// 128 modulator clock delay
    Delay128 = 6,

    /// 256 modulator clock delay
    Delay256 = 7,

    /// 512 modulator clock delay
    Delay512 = 8,

    /// 1024 modulator clock delay
    Delay1024 = 9,

    /// 2048 modulator clock delay
    Delay2048 = 10,

    /// 4096 modulator clock delay
    Delay4096 = 11,

    /// 8192 modulator clock delay
    Delay8192 = 12,

    /// 16384 modulator clock delay
    Delay16384 = 13,

    /// 32768 modulator clock delay
    Delay32768 = 14,

    /// 65536 modulator clock delay
    Delay65536 = 15,
}

/// Current-detect channel selection
/// Channels required to trigger current-detect
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum CurrentDetectChannels {
    /// Any channel
    ///
    /// This is the default channel selection
    #[default]
    AnyChannel = 0,

    /// All channels
    AllChannels = 1,
}

/// Number of current-detect exceeded thresholds to trigger a detection
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum CurrentDetectCount {
    /// 1 detection
    ///
    /// This is the default current-detect length
    #[default]
    Count1 = 0,

    /// 2 detections
    Count2 = 1,

    /// 4 detections
    Count4 = 2,

    /// 8 detections
    Count8 = 3,

    /// 16 detections
    Count16 = 4,

    /// 32 detections
    Count32 = 5,

    /// 64 detections
    Count64 = 6,

    /// 128 detections
    Count128 = 7,
}

/// Current-detect measurement length in conversion periods
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum CurrentDetectLength {
    /// 128 conversion periods
    ///
    /// This is the default current-detect length
    #[default]
    Len128 = 0,

    /// 256 conversion periods
    Len256 = 1,

    /// 512 conversion periods
    Len512 = 2,

    /// 768 conversion periods
    Len768 = 3,

    /// 1280 conversion periods
    Len1280 = 4,

    /// 1792 conversion periods
    Len1792 = 5,

    /// 2560 conversion periods
    Len2560 = 6,

    /// 3584 conversion periods
    Len3584 = 7,
}

/// DC block filter setting
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum DcBlock {
    /// DC block filter disabled
    #[default]
    Disabled = 0,

    /// 1/4
    OneOver4 = 1,

    /// 1/8
    OneOver8 = 2,

    /// 1/16
    OneOver16 = 3,

    /// 1/32
    OneOver32 = 4,

    /// 1/64
    OneOver64 = 5,

    /// 1/128
    OneOver128 = 6,

    /// 1/256
    OneOver256 = 7,

    /// 1/512
    OneOver512 = 8,

    /// 1/1024
    OneOver1024 = 9,

    /// 1/2048
    OneOver2048 = 10,

    /// 1/4096
    OneOver4096 = 11,

    /// 1/8192
    OneOver8192 = 12,

    /// 1/16384
    OneOver16384 = 13,

    /// 1/32768
    OneOver32768 = 14,

    /// 1/65536
    OneOver65536 = 15,
}

/// Channel input selection
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Sequence)]
#[repr(u16)]
pub enum ChannelMux {
    /// AINxP and AINxN
    ///
    /// This is the default channel mux setting
    #[default]
    AnalogIn = 0,

    /// ADC inputs shorted
    Shorted = 1,

    /// Positive DC test signal
    PositiveTest = 2,

    /// Negative DC test signal
    NegativeTest = 3,
}

/// Device ID register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id {
    channel_count: u4,
}

impl Id {
    /// Decode a Status from it's register word
    pub fn from_word(word: u16) -> Id {
        Id {
            channel_count: u4::new(((word >> 8) & 0b1111) as u8),
        }
    }
}

/// Device STATUS register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// Whether the SPI interface is locked
    pub lock: bool,

    /// ADC resynchronization indicator.
    /// This bit is set each time the ADC resynchronizes.
    pub resync: bool,

    /// Whether the register map had a CRC error
    pub reg_map_crc_err: bool,

    /// Whether a SPI input had a CRC error
    pub spi_crc_err: bool,

    /// CRC type
    pub crc_type: CrcType,

    /// Whether a reset occurred
    pub reset: bool,

    /// Data word length
    pub word_length: WordLength,

    /// Channel 0 ADC data available indicator
    pub drdy0: bool,

    /// Channel 1 ADC data available indicator
    pub drdy1: bool,

    /// Channel 2 ADC data available indicator
    pub drdy2: bool,

    /// Channel 3 ADC data available indicator
    pub drdy3: bool,
}

impl Status {
    /// Decode a Status from it's register word
    pub fn from_word(word: u16) -> Status {
        Status {
            lock: is_bit_set!(word, 15),
            resync: is_bit_set!(word, 14),
            reg_map_crc_err: is_bit_set!(word, 13),
            spi_crc_err: is_bit_set!(word, 12),
            crc_type: CrcType::try_from((word >> 11) & 0b1).unwrap(),
            reset: is_bit_set!(word, 10),
            word_length: WordLength::try_from((word >> 8) & 0b11).unwrap(),
            drdy0: is_bit_set!(word, 0),
            drdy1: is_bit_set!(word, 1),
            drdy2: is_bit_set!(word, 2),
            drdy3: is_bit_set!(word, 3),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status {
            lock: false,
            resync: false,
            reg_map_crc_err: false,
            spi_crc_err: false,
            crc_type: CrcType::Ccitt,
            reset: true,
            word_length: WordLength::Bits24,
            drdy0: false,
            drdy1: false,
            drdy2: false,
            drdy3: false,
        }
    }
}

/// Device MODE register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mode {
    /// Whether register map CRC checking is enabled
    pub reg_crc_enable: bool,

    /// Whether SPI CRC checking is enabled
    pub spi_crc_enable: bool,

    /// SPI and register map CRC type
    pub crc_type: CrcType,

    /// Reset
    /// Write false to clear this bit in the status register
    pub reset: bool,

    /// Data word length
    pub word_length: WordLength,

    /// SPI timeout enable
    pub spi_timeout: bool,

    /// DRDY pin signal source selection
    pub drdy_source: DrdySource,

    /// DRDY state when conversion data is not available
    pub drdy_not_ready_state: DrdyNotReadyState,

    /// DRDY state when conversion data is available
    pub drdy_ready_state: DrdyReadyState,
}

impl Mode {
    /// Decode a Mode from it's register word
    pub fn from_word(word: u16) -> Mode {
        Mode {
            reg_crc_enable: is_bit_set!(word, 13),
            spi_crc_enable: is_bit_set!(word, 12),
            crc_type: CrcType::try_from((word >> 11) & 0b1).unwrap(),
            reset: is_bit_set!(word, 10),
            word_length: WordLength::try_from((word >> 8) & 0b11).unwrap(),
            spi_timeout: is_bit_set!(word, 4),
            drdy_source: DrdySource::try_from((word >> 2) & 0b11).unwrap(),
            drdy_not_ready_state: DrdyNotReadyState::try_from((word >> 1) & 0b1).unwrap(),
            drdy_ready_state: DrdyReadyState::try_from(word & 0b1).unwrap(),
        }
    }

    /// Returns the register word for this Mode configuration
    pub fn to_word(&self) -> u16 {
        u16::from(self.reg_crc_enable) << 13
            | u16::from(self.spi_crc_enable) << 12
            | u16::from(self.crc_type) << 11
            | u16::from(self.reset) << 10
            | u16::from(self.word_length) << 8
            | u16::from(self.spi_timeout) << 4
            | u16::from(self.drdy_source) << 2
            | u16::from(self.drdy_not_ready_state) << 1
            | u16::from(self.drdy_ready_state)
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            reg_crc_enable: false,
            spi_crc_enable: false,
            crc_type: CrcType::default(),
            reset: true,
            word_length: WordLength::default(),
            spi_timeout: true,
            drdy_source: DrdySource::default(),
            drdy_not_ready_state: DrdyNotReadyState::default(),
            drdy_ready_state: DrdyReadyState::default(),
        }
    }
}

/// Device CLOCK register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    /// Channel 0 ADC enable
    pub channel0_en: bool,

    /// Channel 1 ADC enable
    pub channel1_en: bool,

    /// Channel 2 ADC enable
    pub channel2_en: bool,

    /// Channel 3 ADC enable
    pub channel3_en: bool,

    /// Modulator oversampling ratio
    pub oversampling_ratio: OversamplingRatio,

    /// Power mode
    pub power_mode: PowerMode,
}

impl Clock {
    /// Decode a Clock from it's register word
    pub fn from_word(word: u16) -> Clock {
        Clock {
            channel0_en: is_bit_set!(word, 8),
            channel1_en: is_bit_set!(word, 9),
            channel2_en: is_bit_set!(word, 10),
            channel3_en: is_bit_set!(word, 11),
            oversampling_ratio: OversamplingRatio::try_from((word >> 2) & 0b1111).unwrap(),
            power_mode: PowerMode::try_from(word & 0b11).unwrap(),
        }
    }

    /// Returns the register word for this Clock configuration
    pub fn to_word(&self) -> u16 {
        u16::from(self.channel3_en) << 11
            | u16::from(self.channel2_en) << 10
            | u16::from(self.channel1_en) << 9
            | u16::from(self.channel0_en) << 8
            | u16::from(self.channel0_en) << 8
            | u16::from(self.oversampling_ratio) << 2
            | u16::from(self.power_mode)
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            channel0_en: true,
            channel1_en: true,
            channel2_en: true,
            channel3_en: true,
            oversampling_ratio: OversamplingRatio::default(),
            power_mode: PowerMode::default(),
        }
    }
}

/// Device GAIN1 register
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gain {
    /// PGA gain selection for channel 0
    pub pga_gain0: PgaGain,

    /// PGA gain selection for channel 1
    pub pga_gain1: PgaGain,

    /// PGA gain selection for channel 2
    pub pga_gain2: PgaGain,

    /// PGA gain selection for channel 3
    pub pga_gain3: PgaGain,
}

impl Gain {
    /// Decode a Gain from it's register word
    pub fn from_word(word: u16) -> Gain {
        Gain {
            pga_gain0: PgaGain::try_from(word & 0b111).unwrap(),
            pga_gain1: PgaGain::try_from((word >> 4) & 0b111).unwrap(),
            pga_gain2: PgaGain::try_from((word >> 8) & 0b111).unwrap(),
            pga_gain3: PgaGain::try_from((word >> 12) & 0b111).unwrap(),
        }
    }

    /// Returns the register word for this Gain configuration
    pub fn to_word(&self) -> u16 {
        u16::from(self.pga_gain0)
            | u16::from(self.pga_gain1) << 4
            | u16::from(self.pga_gain2) << 8
            | u16::from(self.pga_gain3) << 12
    }
}

/// Device CFG register
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Global-chop delay
    pub global_chop_delay: GlobalChopDelay,

    /// Global-chop mode enable
    pub global_chop_enable: bool,

    /// Current-detect channel selection
    /// Channels required to trigger current-detect
    pub current_detect_channels: CurrentDetectChannels,

    /// Number of current-detect exceeded thresholds to trigger a detection
    pub current_detect_count: CurrentDetectCount,

    /// Current-detect measurement length
    pub current_detect_length: CurrentDetectLength,

    /// Current-detect mode enable
    pub current_detect_enable: bool,
}

impl Config {
    /// Decode a Config from it's register word
    pub fn from_word(word: u16) -> Config {
        Config {
            global_chop_delay: GlobalChopDelay::try_from((word >> 9) & 0b1111).unwrap(),
            global_chop_enable: is_bit_set!(word, 8),
            current_detect_channels: CurrentDetectChannels::try_from((word >> 7) & 0b1).unwrap(),
            current_detect_count: CurrentDetectCount::try_from((word >> 4) & 0b111).unwrap(),
            current_detect_length: CurrentDetectLength::try_from((word >> 1) & 0b111).unwrap(),
            current_detect_enable: is_bit_set!(word, 0),
        }
    }

    /// Returns the register word for this Config configuration
    pub fn to_word(&self) -> u16 {
        u16::from(self.global_chop_delay) << 9
            | u16::from(self.global_chop_enable) << 8
            | u16::from(self.current_detect_channels) << 7
            | u16::from(self.current_detect_count) << 4
            | u16::from(self.current_detect_length) << 1
            | u16::from(self.current_detect_enable)
    }
}

/// Device THRSHLD_MSB and THRSHLD_LSB registers
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Threshold {
    /// Current-detect mode threshold
    pub current_detect_threshold: u24,

    /// DC block filter setting
    pub dc_block: DcBlock,
}

impl Threshold {
    /// Decode a Threshold from it's register words
    ///
    /// Words must be in the order [THRSHLD_MSB, THRSHLD_LSB]
    pub fn from_words(words: [u16; 2]) -> Threshold {
        Threshold {
            current_detect_threshold: u24::new((words[0] as u32) << 8 | (words[1] as u32) >> 8),
            dc_block: DcBlock::try_from(words[1] & 0b1111).unwrap(),
        }
    }

    /// Returns the register words for this Threshold configuration
    ///
    /// Words are in the order [THRSHLD_MSB, THRSHLD_LSB]
    pub fn to_words(&self) -> [u16; 2] {
        let threshold = u32::from(self.current_detect_threshold);

        [
            (threshold >> 8) as u16,
            ((threshold as u16) << 8) | u16::from(self.dc_block),
        ]
    }
}

/// Device CHx_CFG register
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelConfig {
    /// Channel 0 phase delay in modulator clock cycles
    phase: u10,

    /// DC block filter for channel 0 disable
    dc_block_disable: bool,

    /// Channel 0 input selection
    mux: ChannelMux,
}

impl ChannelConfig {
    /// Decode a ChannelConfig from it's register word
    pub fn from_word(word: u16) -> ChannelConfig {
        ChannelConfig {
            phase: u10::new(word >> 6),
            dc_block_disable: is_bit_set!(word, 2),
            mux: ChannelMux::try_from(word & 0b11).unwrap(),
        }
    }

    /// Returns the register word for this ChannelConfig
    pub fn to_word(&self) -> u16 {
        u16::from(self.phase) << 6 | u16::from(self.dc_block_disable) << 2 | u16::from(self.mux)
    }
}

/// Device CHx_OCAL_MSB and CHx_OCAL_LSB registers
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetCal {
    /// Channel offset calibration
    pub offset: u24,
}

impl OffsetCal {
    /// Decode an OffsetCal from it's register words
    ///
    /// Words must be in the order [CHx_OCAL_MSB, CHx_OCAL_LSB]
    pub fn from_words(words: [u16; 2]) -> OffsetCal {
        OffsetCal {
            offset: u24::new((words[0] as u32) << 8 | (words[1] as u32) >> 8),
        }
    }

    /// Returns the register words for this OffsetCal configuration
    ///
    /// Words are in the order [CHx_OCAL_MSB, CHx_OCAL_LSB]
    pub fn to_words(&self) -> [u16; 2] {
        let offset = u32::from(self.offset);

        [(offset >> 8) as u16, ((offset as u16) << 8)]
    }
}

/// Device CHx_GCAL_MSB and CHx_GCAL_LSB registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GainCal {
    /// Channel gain calibration
    pub gain: u24,
}

impl GainCal {
    /// Decode an GainCal from it's register words
    ///
    /// Words must be in the order [CHx_GCAL_MSB, CHx_GCAL_LSB]
    pub fn from_words(words: [u16; 2]) -> GainCal {
        GainCal {
            gain: u24::new((words[0] as u32) << 8 | (words[1] as u32) >> 8),
        }
    }

    /// Returns the register words for this GainCal configuration
    ///
    /// Words are in the order [CHx_GCAL_MSB, CHx_GCAL_LSB]
    pub fn to_words(&self) -> [u16; 2] {
        let gain = u32::from(self.gain);

        [(gain >> 8) as u16, ((gain as u16) << 8)]
    }
}

impl Default for GainCal {
    fn default() -> Self {
        GainCal {
            gain: u24::new(0x800000),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_decode() {
        for (word, count) in [
            (0x2200, 2),
            (0x2300, 3),
            (0x2400, 4),
            (0x2600, 6),
            (0x2800, 8),
        ] {
            assert_eq!(
                Id::from_word(word),
                Id {
                    channel_count: u4::new(count)
                }
            )
        }
    }

    #[test]
    fn status_default() {
        assert_eq!(Status::from_word(0x500), Status::default())
    }

    #[test]
    fn mode_default() {
        assert_eq!(Mode::default().to_word(), 0x510);
        assert_eq!(Mode::from_word(0x510), Mode::default())
    }

    #[test]
    fn mode_round_trip() {
        for bools in [false, true] {
            for crc_type in enum_iterator::all::<CrcType>() {
                for word_length in enum_iterator::all::<WordLength>() {
                    for drdy_source in enum_iterator::all::<DrdySource>() {
                        for drdy_not_ready_state in enum_iterator::all::<DrdyNotReadyState>() {
                            for drdy_ready_state in enum_iterator::all::<DrdyReadyState>() {
                                let mode = Mode {
                                    reg_crc_enable: bools,
                                    spi_crc_enable: bools,
                                    crc_type,
                                    reset: bools,
                                    word_length,
                                    spi_timeout: bools,
                                    drdy_source,
                                    drdy_not_ready_state,
                                    drdy_ready_state,
                                };

                                assert_eq!(mode, Mode::from_word(mode.to_word()))
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn clock_default() {
        assert_eq!(Clock::default().to_word(), 0xF0E);
        assert_eq!(Clock::from_word(0xF0E), Clock::default())
    }

    #[test]
    fn clock_round_trip() {
        for bools in [false, true] {
            for oversampling_ratio in enum_iterator::all::<OversamplingRatio>() {
                for power_mode in enum_iterator::all::<PowerMode>() {
                    let clock = Clock {
                        channel0_en: bools,
                        channel1_en: bools,
                        channel2_en: bools,
                        channel3_en: bools,
                        oversampling_ratio,
                        power_mode,
                    };

                    assert_eq!(clock, Clock::from_word(clock.to_word()))
                }
            }
        }
    }

    #[test]
    fn gain_default() {
        assert_eq!(Gain::default().to_word(), 0);
        assert_eq!(Gain::from_word(0), Gain::default())
    }

    #[test]
    fn gain_round_trip() {
        for gains in enum_iterator::all::<PgaGain>() {
            let gain = Gain {
                pga_gain0: gains,
                pga_gain1: gains,
                pga_gain2: gains,
                pga_gain3: gains,
            };

            assert_eq!(gain, Gain::from_word(gain.to_word()))
        }
    }

    #[test]
    fn config_default() {
        assert_eq!(Config::default().to_word(), 0x600);
        assert_eq!(Config::from_word(0x600), Config::default())
    }

    #[test]
    fn config_round_trip() {
        for bools in [false, true] {
            for global_chop_delay in enum_iterator::all::<GlobalChopDelay>() {
                for current_detect_channels in enum_iterator::all::<CurrentDetectChannels>() {
                    for current_detect_count in enum_iterator::all::<CurrentDetectCount>() {
                        for current_detect_length in enum_iterator::all::<CurrentDetectLength>() {
                            let config = Config {
                                global_chop_delay,
                                global_chop_enable: bools,
                                current_detect_channels,
                                current_detect_count,
                                current_detect_length,
                                current_detect_enable: bools,
                            };

                            assert_eq!(config, Config::from_word(config.to_word()))
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn threshold_default() {
        assert_eq!(Threshold::default().to_words(), [0, 0]);
        assert_eq!(Threshold::from_words([0, 0]), Threshold::default())
    }

    #[test]
    fn threshold_round_trip() {
        for thresholds in [
            0, 2097152, 4194304, 6291456, 8388608, 10485760, 12582912, 14680064,
        ] {
            for dc_block in enum_iterator::all::<DcBlock>() {
                let threshold = Threshold {
                    current_detect_threshold: u24::new(thresholds),
                    dc_block,
                };

                assert_eq!(threshold, Threshold::from_words(threshold.to_words()))
            }
        }
    }

    #[test]
    fn channel_config_default() {
        assert_eq!(ChannelConfig::default().to_word(), 0);
        assert_eq!(ChannelConfig::from_word(0), ChannelConfig::default())
    }

    #[test]
    fn channel_config_round_trip() {
        for phase in [0, 128, 256, 384, 512, 640, 768, 896] {
            for dc_block_disable in [false, true] {
                for mux in enum_iterator::all::<ChannelMux>() {
                    let config = ChannelConfig {
                        phase: u10::new(phase),
                        dc_block_disable,
                        mux,
                    };

                    assert_eq!(config, ChannelConfig::from_word(config.to_word()))
                }
            }
        }
    }

    #[test]
    fn channel_offset_cal_default() {
        assert_eq!(OffsetCal::default().to_words(), [0, 0]);
        assert_eq!(OffsetCal::from_words([0, 0]), OffsetCal::default())
    }

    #[test]
    fn channel_offset_cal_round_trip() {
        for offset in [
            0, 2097152, 4194304, 6291456, 8388608, 10485760, 12582912, 14680064,
        ] {
            let offset_cal = OffsetCal {
                offset: u24::new(offset),
            };

            assert_eq!(offset_cal, OffsetCal::from_words(offset_cal.to_words()));
        }
    }

    #[test]
    fn channel_gain_cal_default() {
        assert_eq!(GainCal::default().to_words(), [0x8000, 0]);
        assert_eq!(GainCal::from_words([0x8000, 0]), GainCal::default())
    }

    #[test]
    fn channel_gain_cal_round_trip() {
        for gain in [
            0, 2097152, 4194304, 6291456, 8388608, 10485760, 12582912, 14680064,
        ] {
            let gain_cal = GainCal {
                gain: u24::new(gain),
            };

            assert_eq!(gain_cal, GainCal::from_words(gain_cal.to_words()));
        }
    }
}
