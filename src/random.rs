pub fn get_source() -> random::Default {
    let mut seed = [0u8; 16];
    getrandom::getrandom(&mut seed).expect("Failed to get random seed");
    let seed: [u64; 2] = [
        u64::from_ne_bytes([
            seed[0], seed[1], seed[2], seed[3], seed[4], seed[5], seed[6], seed[7],
        ]),
        u64::from_ne_bytes([
            seed[8], seed[9], seed[10], seed[11], seed[12], seed[13], seed[14], seed[15],
        ]),
    ];
    random::default().seed(seed)
}
