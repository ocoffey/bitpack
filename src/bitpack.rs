/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let (neg, n) = match n.is_negative() {
        true => (1, (n * -1) as u64),
        false => (0, n as u64),
    };
    // works equivalently to fitsu
    fitsu(n, width - neg)
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    match (width != 0, n.highest_one()) {
        (true, Some(idx)) => (idx as u64) < width,
        (true, None) => true,
        (false, _) => false,
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
    match bitmask(width, lsb) {
        Some(mask) => {
            let leftpad = 64 - width - lsb;
            Some(((word & mask) << leftpad) as i64 >> (leftpad + lsb))
            /*
            match extract.highest_one() == Some((width + lsb - 1) as u32) {
                true => {
                    extract &= !extract.isolate_highest_one();
                    extract >>= lsb;
                    Some(extract as i64 * -1)
                },
                false => Some((extract >> lsb) as i64),
            }*/
        },
        None => None,
    }
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
    match bitmask(width, lsb) {
        Some(mask) => Some((word & mask) >> lsb),
        None => None,
    }
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
    let (neg, mut value) = match value.is_negative() {
        true => (true, (value * -1).cast_unsigned()),
        false => (false, value.cast_unsigned()),
    };
    if neg {
        // highest_one will never panic, since value had to be negative
        // to get here
        value |= 1 << (value.highest_one().unwrap() + 1);
    }
    match bitmask_val(width, lsb, value) {
        Some((mask, shiftval)) => Some(insert_val(word, mask, shiftval)),
        None => None
    }
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
    match bitmask_val(width, lsb, value) {
        Some((mask, shiftval)) => Some(insert_val(word, mask, shiftval)),
        None => None
    }
}

/// # `bitmask`
///
/// Given a 'width' and 'least significant bit' each of type u64,
/// if their sum fits within 64 (bits), create a full mask of 1's
/// (u64::MAX), `shift left` to match size `width` of 1's, then
/// `shift right` to place the 1's into their required place
///
/// ## Arguments
///
/// * `width`: the size (in bits) for our mask
/// * `lsb`: the placement (in bits) from the righthand side of our mask
///
/// ## Returns
///
/// * Some(mask) if the requested mask is of appropriate size
/// * None otherwise
#[inline]
fn bitmask(width: u64, lsb: u64) -> Option<u64> {
    match (width != 0, width + lsb <= 64) {
        (true, true) => Some((u64::MAX << 64 - width) >> 64 - width - lsb),
        _ => None
    }
}

/// # `bitmask_val`
///
/// Extension to the `bitmask` function. In addition to creating a mask, we
/// shift a passed `value` to align with the mask for insertion.
///
/// ## Arguments
///
/// * `width`: the size (in bits) for our mask
/// * `lsb`: the placement (in bits) from the righthand side of our mask
/// * `value`: a u64 to shift by `lsb` bits
///
/// ## Returns
///
/// * Some((mask, shifted_value))
///   * If the requested mask is of appropriate size
///   * And our `value` fits within `width`
/// * None otherwise
#[inline]
fn bitmask_val(width: u64, lsb: u64, value: u64) -> Option<(u64, u64)> {
    match (fitsu(value, width), bitmask(width, lsb)) {
        (true, Some(mask)) => Some((mask, value << lsb)),
        _ => None,
    }
}

/// # `insert_val`
///
/// Inserts an u64 `shifted value` into an existing packed `word`
/// via bitmask manipulations
///
/// ## Arguments
///
/// * `word`: a u64 value which may have other packed values inside
/// * `mask`: a u64 bitmask used to erase the relevant section of `word`
/// * `shiftval`: a `value` which has been shifted to its correct bit location
///
/// ## Returns
///
/// * The `word` with `shiftval` inserted into it's appropriate location
#[inline]
fn insert_val(word: u64, mask: u64, shiftval: u64) -> u64 {
    (word & !mask) | shiftval
}
