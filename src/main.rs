// RUSTATION //
// SUPAHAXOR // 14/07/2025 //
// main.rs //

use std::{thread, time::Duration};
// This program is horribly innacurate, but it is a start

mod cpu;
mod memory;
mod bus;
mod bios;

use anyhow::Result;

fn main() -> Result<()> {

    env_logger::init();
    let bios = bios::load_bios("SCPH1001.BIN")?;
    //let mut memory = memory::Memory::new(bios);
    //let mut cpu = cpu::CPU::new();
    
    let mut clock_cycle = 0;
    let max_clock_cycles = 100000;
    let clock_interval = 29.98e-9; // 29.98 nanoseconds

    while clock_cycle < max_clock_cycles{

        thread::sleep(Duration::from_secs_f64(clock_interval));

        if clock_cycle % 10000 == 0{
            println!("Clock cycle: {}", clock_cycle);
        }
        clock_cycle += 1;

    }
    println!("Clock cycles completed: {}", clock_cycle);
    // Please tell me this is a joke?
    // Yeah, of course it is. I'll sort an actual 60Hz loop later.
    Ok(())
}        
