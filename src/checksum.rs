use adler32::RollingAdler32;

use crate::unpack::Endian;

use super::unpack;

pub fn adler32_checksum(contents: &Vec<u8>, adler32_bytes: &[u8], byteorder: Endian) -> bool {
    let adler32 = match byteorder {
        Endian::BE => unpack::unpack_u32(adler32_bytes, Endian::BE),
        Endian::LE => unpack::unpack_u32(adler32_bytes, Endian::LE),
    };

    let mut rolling_adler32 = RollingAdler32::new();
    rolling_adler32.update_buffer(contents);
    let hash = rolling_adler32.hash();
    if hash & 0xffffffff == adler32 {
        return true;
    }
    false
}

#[test]
fn test() {
    let contents = "abcdefghi";
    let mut rolling_adler32 = RollingAdler32::new();
    rolling_adler32.update_buffer(&contents.as_bytes());
    let hash = rolling_adler32.hash();
    assert_eq!(0x118e038e, hash);
}
