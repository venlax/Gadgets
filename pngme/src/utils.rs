pub fn u8_array_to_u32(arr: [u8; 4]) -> u32 {
    (arr[0] as u32) << 24 |
    (arr[1] as u32) << 16 |
    (arr[2] as u32) << 8  |
    (arr[3] as u32)
}

pub fn u8_array_to_u64(arr: [u8; 8]) -> u64 {
    (arr[0] as u64) << 56 |
    (arr[1] as u64) << 48 |
    (arr[2] as u64) << 40 |
    (arr[3] as u64) << 32 |
    (arr[4] as u64) << 24 |
    (arr[5] as u64) << 16 |
    (arr[6] as u64) << 8  |
    (arr[7] as u64)
}