/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align down: align is not a power of 2.");
    }
    addr & !(align - 1)
    // (addr / align) * align
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align up: align is not a power of 2.");
    }
    addr.saturating_add(align - 1) & !(align - 1)
}
