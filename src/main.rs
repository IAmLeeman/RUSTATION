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
    
    Ok(())
}        
