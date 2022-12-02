use crate::instruction::types::RVType;

pub enum RVOpcode {
    ADDI = 1
}

impl RVOpcode {
    /// Returns the Risc-V instruction type of the instruction.
    pub fn default_type(&self) -> RVType {
        todo!()
    }
}

impl From<u8> for RVOpcode {
    fn from(code: u8) -> Self {
        todo!()
    }
}