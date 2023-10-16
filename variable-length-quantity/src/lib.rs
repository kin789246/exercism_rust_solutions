#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}
/*
    00000080             81 00          1|000 0000 -> 1000 0001 0000 0000 -> 81 00
    00002000             C0 00          00|10 0000 0|000 0000 -> 0000 0000 | 1100 0000 | 0000 0000 -> 0 C0 00
    00003FFF             FF 7F          00|11 1111 1|111 1111 -> 0000 0000 | 1111 1111 | 0111 1111 -> 0 FF 7F
    00004000           81 80 00         01|00 0000 0|000 0000 -> 1000 0001 | 1000 0000 | 0000 0000 -> 81 80 00 
 */
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut ans: Vec<u8> = Vec::new();
    values.iter().rev().for_each(|&n| {
        ans.push(n as u8 & 0x7f);
        let mut rest = n >> 7;
        while rest != 0 {
            ans.push(rest as u8 & 0x7f | 0x80);
            rest >>= 7;
        }
    });
    ans.reverse();
    ans
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut f: Vec<u32> = Vec::new();
    let mut iter = bytes.iter().peekable();
    let mut n = 0_u32;
    while let Some(byte) = iter.next() {
        n = if n.leading_zeros() >= 7 {
            (n << 7) | (byte & 0x7f) as u32
        }
        else {
            return Err(Error::Overflow)
        };
        if byte & 0x80 == 0 {
            f.push(n);
            n = 0;
            continue;
        }
        else if iter.peek().is_none() {
            return Err(Error::IncompleteNumber)
        }
    }
    Ok(f)
}
