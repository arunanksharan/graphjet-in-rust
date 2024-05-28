pub struct IdentityIDMask;

impl IDMask for IdentityIDMask {
    fn restore(&self, node: u64) -> u64 {
        node
    }
}
