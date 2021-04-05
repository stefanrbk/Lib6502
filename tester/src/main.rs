extern crate lib6502;

use lib6502::*;

use std::thread;

fn main() {
    let mem = new_bus!(Memory::new());
    write_bus!(mem, 0x0001, 0x69);

    let mem_arc1 = mem.clone();

    let kill_signal = new_pin!(false);
    let rw_signal = new_pin!(false);

    let addr_bus = new_bus!(0 as u16);
    let data_bus = new_bus!(0 as u8);

    let phase_1_positive_edge = new_barrier!(2);
    let addr_stable = new_barrier!(2);
    let data_stable = new_barrier!(2);

    let mem_addr_bus = addr_bus.clone();
    let mem_data_bus = data_bus.clone();

    let mem_kill_signal = kill_signal.clone();
    let mem_rw_signal = rw_signal.clone();

    let mem_start = phase_1_positive_edge.clone();
    let mem_addr_stable = addr_stable.clone();
    let mem_data_stable = data_stable.clone();

    let mem_handle = thread::spawn(move || {
        while !*mem_kill_signal.read().unwrap() {
            // wait until the start of a cpu cycle
            mem_start.wait();

            if read_bus!(mem_rw_signal) {
                // if cpu signals a memory read
                mem_addr_stable.wait(); // wait until cpu has pushed an address
                let addr = read_bus!(mem_addr_bus);
                // push the data at that addr into data bus
                write_bus!(mem_data_bus, read_bus!(mem_arc1)[addr]);
                mem_data_stable.wait(); // signal the cpu that the requested data is available
            } else {
                // if cpu signals a memory write
                mem_addr_stable.wait(); // wait until cpu has pushed an address
                let addr = read_bus!(mem_addr_bus);
                mem_data_stable.wait(); // wait until cpu has pushed the data
                let data = read_bus!(mem_data_bus);
                write_bus!(mem_arc1, addr, data);
            }
        }
        println!("Memory[0000] = {:x}", read_bus!(mem_arc1)[0x0000]);
    });

    let mut io = CpuIO::new();
    io.rdy = kill_signal;
    io.rw = rw_signal;
    io.addr_bus = addr_bus;
    io.data_bus = data_bus;
    io.phase_1_positive_edge = phase_1_positive_edge;
    io.addr_stable = addr_stable;
    io.data_stable = data_stable;

    let cpu = Cpu::new(io);
    let cpu_handle = cpu.start();
    mem_handle.join().unwrap();
    cpu_handle.join().unwrap();
}
