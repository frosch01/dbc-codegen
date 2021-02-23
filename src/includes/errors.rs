#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// Minimum value defined in DBC for signal
        min: f64,
        /// Maximum value defined in DBC for signal
        max: f64,
    },
    InvalidPayloadSize,
}
