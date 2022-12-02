use crate::instruction::opcodes::RVOpcode;

pub enum RVType {
    /// R-Type instructions
    R(
        RVOpcode, // opcode       bits 0 - 7  -> 7 bit
        u8, // rd           bits 7 -12  -> 5 bit
        u8, // func3        bits 12-15  -> 3 bit
        u8, // rs1          bits 15-20  -> 5 bit
        u8, // rs2          bits 20-25  -> 5 bit
        u8, // func7        bits 25-32  -> 7 bit
    ),
    /// I-Type instructions
    I(
        RVOpcode, // opcode       bits 0 - 7  -> 7 bits
        u8, // rd           bits 7 -12  -> 5 bits
        u8, // func3        bits 12-15  -> 3 bits
        u8, // rs1          bits 15-20  -> 5 bits
        u32, // imm[0:12]   bits 20-32  -> 12 bits
    ),
    /// S-Type instructions
    S(
        RVOpcode, // opcode       bits 0 - 7  -> 7 bits
            // imm[0:5]     bits 7 -12  -> 5 bits
        u8, // func3        bits 12-15  -> 3 bits
        u8, // rs1          bits 15-20  -> 5 bits
        u8, // rs2          bits 20-25  -> 5 bits
        u32, // imm[5:12]   bits 25-32  -> 7 bits
    ),
    /// B-Subtype instructions
    B(
        RVOpcode, // code         bits 0 - 7  -> 7 bits
            // imm[11]      bits 7 - 8  -> 1 bit
            // imm[1:5]     bits 8 - 12 -> 4 bits
        u8, // func3        bits 12-15  -> 3 bits
        u8, // rs1          bits 15-20  -> 5 bits
        u8, // rs2          bits 20-25  -> 5 bits
            // imm[5:11]    bits 25-31  -> 11 bits
        u32, // imm[12]     bits 31-32  -> 1 bit
    ),
    /// U-Type instructions
    U(
        RVOpcode, // opcode       bits 0 - 7  -> 7 bits
        u8, // rd           bits 7 -12  -> 5 bits
        u32, // imm[12:32]  bits 12-32  -> 20 bits
    ),
    /// J-Subtype instructions
    J(
        RVOpcode, // opcode       bits 0 - 7  -> 7 bits
        u8, // rd           bits 7 -12  -> 5 bits
            // imm[12:20]   bits 12-20  -> 8 bits
            // imm[11]      bits 20-21  -> 1 bit
            // imm[1:11]    bits 21-31  -> 10 bits
        u32, // imm[20]     bits 31-32  -> 1 bit
    ),
    Unknown,
}

impl RVType {
    pub fn decode(instr: u32) -> Self {
        let opcode: RVOpcode = ((instr & 0b1111111) as u8).into();
        let mut out = opcode.default_type();
        match &mut out {
            RVType::R(code, rd, func3, rs1, rs2, func7) => {
                *code = opcode;
                *rd = ((instr >> 7) & 0b11111) as u8;
                *func3 = ((instr >> 12) & 0b111) as u8;
                *rs1 = ((instr >> 15) & 0xb11111) as u8;
                *rs2 = ((instr >> 20) & 0xb11111) as u8;
                *func7 = (instr >> 25) as u8;
            }
            RVType::I(code, rd, func3, rs1, imm) => {
                *code = opcode;
                *rd = ((instr >> 7) & 0b11111) as u8;
                *func3 = ((instr >> 12) & 0b111) as u8;
                *rs1 = ((instr >> 15) & 0b11111) as u8;
                // flip all upper bits from 12:32 if sign = 1
                *imm = (instr >> 20) | (((1 << 20) - (instr >> 31)) << 12);
            }
            RVType::S(code, func3, rs1, rs2, imm) => {
                *code = opcode;
                *imm = (instr >> 7) & 0b11111;
                *func3 = ((instr >> 12) & 0b111) as u8;
                *rs1 = ((instr >> 15) & 0b11111) as u8;
                *rs2 = ((instr >> 20) & 0b11111) as u8;
                *imm |= (instr >> 25) << 5;
                // flip all upper bits from 12:32 if sign = 1
                *imm |= ((1 << 20) - (instr >> 31)) << 12;
            }
            RVType::B(code, func3, rs1, rs2, imm) => {
                *code = opcode;
                *imm = ((instr >> 8) & 0b1111) << 1;
                *imm |= ((instr >> 7) & 0b1) << 11;
                *func3 = ((instr >> 12) & 0b111) as u8;
                *rs1 = ((instr >> 15) & 0b11111) as u8;
                *rs2 = ((instr >> 20) & 0b11111) as u8;
                *imm |= ((instr >> 25) & 0b111111) << 5;
                // flip all upper bits from 12:32 if sign = 1
                *imm |= ((1 << 20) - (instr >> 31)) << 12;
            }
            RVType::U(code, rd, imm) => {
                *code = opcode;
            }
            RVType::J(code, rd, imm) => {
                *code = opcode;
            }
            RVType::Unknown => {
                panic!("Unknown opcode type");
            }
        }

        out
    }
}
