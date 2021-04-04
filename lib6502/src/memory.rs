const MAXMEM: usize = 1024 * 64;

pub struct Memory {
    data: [u8; MAXMEM],
}

impl Memory {
    pub fn new() -> Memory {
        let d = [0 as u8; MAXMEM];
        Memory { data: d }
    }
}
