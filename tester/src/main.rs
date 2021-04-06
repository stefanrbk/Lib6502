extern crate lib6502;

use lib6502::*;

use std::thread;

fn main() {
    let clock = Clock::new(2);

    let p1s = clock.phase_1_positive_edge.clone();
    let p1e = clock.phase_1_negative_edge.clone();
    let p2s = clock.phase_2_positive_edge.clone();
    let p2e = clock.phase_2_negative_edge.clone();
    let kill = clock.kill.clone();

    let handle1 = thread::spawn(move || {
        for _ in 0..4 {
            p1s.wait();
            p1e.wait();
            p2s.wait();
            p2e.wait();
        }
        p1s.wait();
        write_bus!(kill, true);
    });

    let handle2 = clock.start();

    handle1.join().unwrap();
    handle2.join().unwrap();
}
