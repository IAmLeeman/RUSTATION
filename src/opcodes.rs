use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, instr: u32){
    let opcode = (instr >> 26) & 0x3f;
    match opcode {
        0x00 => {
            let funct = instr & 0x3f;
            match funct{
                0x20 => {ADD(cpu, instr)}
                0x22 => {/* SUB */}
                0x24 => {/* AND */}
                0x25 => {/* OR */}
                0x2a => {/* SLT */}
                _=> {panic!("Unknown code: 0x{:02x}", funct);
                }
            }
        }
    }
}
fn ADD(cpu: &mut CPU, instr: u32) {
    
}