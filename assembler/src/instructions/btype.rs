use super::types::{Imm, Reg};
use super::instruction::Instruction;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum BTypeMne {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

impl FromStr for BTypeMne {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_ref() {
            "beq" => Ok(BTypeMne::BEQ),
            "bne" => Ok(BTypeMne::BNE),
            "blt" => Ok(BTypeMne::BLT),
            "bge" => Ok(BTypeMne::BGE),
            "bltu" => Ok(BTypeMne::BLTU),
            "bgeu" => Ok(BTypeMne::BGEU),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BType {
    pub mne: BTypeMne,
    pub rs1: Reg,
    pub rs2: Reg,
    pub imm: Imm,
}


impl Instruction for BType {
    fn translate(&self) -> Vec<u8> {
        let funct3 = match self.mne {
            BTypeMne::BEQ => 0b000,
            BTypeMne::BNE => 0b001,
            BTypeMne::BLT => 0b100,
            BTypeMne::BGE => 0b101,
            BTypeMne::BLTU => 0b110,
            BTypeMne::BGEU => 0b111,
        };
        let opcode = 0b1100011;
        let imm12 = (self.imm >> 12) & 0x1;
        let imm10_5 = (self.imm >> 5) & 0x3F;
        let imm4_1 = (self.imm >> 1) & 0xF;
        let imm11 = (self.imm >> 11) & 0x1;
        let result: u32 = 0
            | (imm12 << 31)
            | (imm10_5 << 25)
            | (self.rs2 << 20)
            | (self.rs1 << 15)
            | (funct3 << 12)
            | (imm4_1 << 8)
            | (imm11 << 7)
            | opcode;
        result.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beq_test(){
        let instruction = BType {
            mne: BTypeMne::BEQ,
            rs1: 10,
            rs2: 23,
            imm: 0b1010101010101,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0xD5, 0x75, 0x0A, 0x63];
        assert_eq!(actual, expected);
    }

    #[test]
    fn bne_test(){
        let instruction = BType {
            mne: BTypeMne::BNE,
            rs1: 11,
            rs2: 3,
            imm: 0b0110111010101,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x5C, 0x35, 0x9A, 0xE3];
        assert_eq!(actual, expected);
    }
}
