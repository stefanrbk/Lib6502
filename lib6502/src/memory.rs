use super::*;

use std::fs;
use std::ops::{Index, IndexMut};

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.data[usize::from(index)]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[usize::from(index)]
    }
}

impl Memory {
    pub fn new() -> Memory {
        let d = [0 as u8; MAXMEM];
        Memory { data: d }
    }

    pub fn load_prg(&mut self, filename: String) -> &mut Memory {
        let prg = fs::read(&filename).expect("Loading program");
        if prg.len() <= 2 {
            panic!("File {} is too short to contain a program!", filename);
        }
        let (addr, data) = prg.split_at(2);
        let address = (addr[0] as u16) | ((addr[1] as u16) << 8);
        let (_, rest) = self.data.split_at_mut(address.into());
        let (mid, _) = rest.split_at_mut(data.len());
        mid.copy_from_slice(data);

        self
    }
    pub fn set_byte(&mut self, addr: u16, value: u8) -> &mut Memory {
        self[addr] = value;

        return self;
    }
    pub fn set_word(&mut self, addr: u16, value: u16) -> &mut Memory {
        self[addr] = (value & 0x00FF) as u8;
        self[u16::wrapping_add(addr, 1)] = (value >> 8) as u8;

        return self;
    }
}
