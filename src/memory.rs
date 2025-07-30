use core::panic;

pub fn memory_read(){

}

pub trait MemoryRead {
    fn load8(&self, address: u32) -> u8;
    fn load16(&self, address: u32) -> u16;
    fn load32(&self, address: u32) -> u32;
}

pub struct Memory {
    data: Vec<u8>
    // Add other mapped regions as necessary
}

impl MemoryRead for Memory {

    fn load32(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        if addr + 3 >= self.data.len(){
            panic!("Memory access out of bounds: address {:#X} exceeds memory size {}", addr, self.data.len());
        }
        
        let b0 = self.data[addr as usize] as u32;
        let b1 = self.data[addr as usize + 1] as u32;
        let b2 = self.data[addr as usize + 2] as u32;
        let b3 = self.data[addr as usize + 3] as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }
    fn load16(&self, addr: u32) -> u16 {

        let addr = addr as usize;
        if addr + 1 >= self.data.len() {
            panic!("Memory access out of bounds: address {:#X} exceeds memory size {}", addr, self.data.len());
        }
        let b0 = self.data[addr as usize] as u16;
        let b1 = self.data[addr as usize + 1] as u16;
        b0 | (b1 << 8)
    }
    fn load8(&self, addr: u32) -> u8 {
        let addr = addr as usize;
        if addr >= self.data.len() {
            panic!("Memory access out of bounds: address {:#X} exceeds memory size {}", addr, self.data.len());
        }
        self.data[addr as usize]
    }

}