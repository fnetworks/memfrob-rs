//! A rust implementation of the popular GNU LibC `memfrob` function.

use std::ops::BitXorAssign;

/// Encrypts the slice s by exclusive-ORing each element with the number 42, converted into T.
///
/// Note that this function is not a proper encryption routine as the XOR constant is fixed,
/// and is only suitable for hiding strings.
pub fn memfrob<T: BitXorAssign + From<u8>>(s: &mut [T]) {
    for c in s {
        *c ^= 42.into();
    }
}

#[cfg(test)]
mod tests {
    use super::memfrob;
    #[test]
    fn it_frobnicates() {
        let mut buf = [4, 2];
        memfrob(&mut buf);
        memfrob(&mut buf);
        assert_eq!(buf[0], 4);
        assert_eq!(buf[1], 2);
    }
}
