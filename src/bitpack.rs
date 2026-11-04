/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 {
        return false;
    }
    let mut neg = false;
    if n < 0 {
        neg = true;
    }
    match n.highest_one() {
        Some(idx) => if neg {
            idx as u64 <= width
        } else {
            idx as u64 + 1 <= width
        },
        None => true,
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 {
        return false;
    }
    match n.highest_one() {
        Some(idx) => idx as u64 <= width,
        None => true,
    }
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bits.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
/// or `None` if the field is impossible
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    None
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bytes.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
///
/// or None
/// if `lsb + width > 64`
///
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    None
}

/// Given an unsigned 64-bit `word`, and an unsigned `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the unsigned value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(value, width) || width + lsb > 64 {
        return None
    }
    Some(0)
}

/// Given an unsigned 64-bit `word`, and a signed `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the signed value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
///
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width) {
        return None
    }
    Some(0)
}
