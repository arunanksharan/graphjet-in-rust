/// Bit mask used to encode further information in entity ids.

pub trait IDMask {
    /// Return original value of node/id
    /// before additional information was masked into it
    fn restore(&self, node: u64) -> u64;
}
