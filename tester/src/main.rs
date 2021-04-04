extern crate lib6502;

use lib6502::*;

fn main() {
    let barrier = no_barrier!();
    let mut io = CpuIO::new();
    io.phase_1_negative_edge = barrier;
    let cpu = Cpu::new(io);
    cpu.start();
}
