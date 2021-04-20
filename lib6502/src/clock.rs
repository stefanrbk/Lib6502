use super::*;
use std::{thread, thread::JoinHandle, time};

impl super::Clock {
    pub fn new(count: usize) -> Clock {
        Clock {
            phase_1_positive_edge: new_barrier!(count),
            phase_1_negative_edge: new_barrier!(count),
            phase_2_positive_edge: new_barrier!(count),
            phase_2_negative_edge: new_barrier!(count),
            p1: new_pin_unset!(),
            p2: new_pin_unset!(),
            kill: new_bus!(false),
        }
    }
    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            let rise_fall = time::Duration::from_nanos(25);
            let cycle = time::Duration::from_nanos(475);
            loop {
                thread::sleep(rise_fall);
                if read_bus!(self.kill) {
                    break;
                }
                set_pin!(self.p1);
                println!("phase 1 started");
                self.phase_1_positive_edge.wait();

                thread::sleep(cycle);
                if read_bus!(self.kill) {
                    break;
                }
                clear_pin!(self.p1);
                println!("phase 1 ended");
                self.phase_1_negative_edge.wait();

                thread::sleep(rise_fall);
                if read_bus!(self.kill) {
                    break;
                }
                set_pin!(self.p2);
                println!("phase 2 started");
                self.phase_2_positive_edge.wait();

                thread::sleep(cycle);
                if read_bus!(self.kill) {
                    break;
                }
                clear_pin!(self.p2);
                println!("phase 2 ended");
                self.phase_2_negative_edge.wait();
            }
        })
    }
}
