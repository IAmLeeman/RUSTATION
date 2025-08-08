use crate::memory::MemoryRead;
use crate::opcodes::execute;

pub struct CPU {
    pub pc: u32,
    pub registers: [u32; 32],
    pub hi: u32,
    pub lo: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self { pc: 0xBFC0_0000,
             registers: [0; 32],
            hi: 0, lo: 0}
    }

    pub fn step(&mut self, memory: &impl MemoryRead) {
        let instr = memory.load32(self.pc);
        self.pc = self.pc.wrapping_add(4);
        self.execute(instr);
    }
    fn execute(&mut self, instr: u32) {
        
        let opcode  = (instr >> 26) & 0x3f;
        match opcode {
           0x00 => self.execute_special(instr),
           _=> {
           println("unknown opcode: {:02X}", opcode);
        }
    }
    fn execute_special(&mut self, instr: u32){
        let funct = instr & 0x3f;
        match funct {

        }
    } 

}