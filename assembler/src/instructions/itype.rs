use super::types::{Imm, Reg};
use super::instruction::Instruction;
use std::str::FromStr;

#[derive(PartialEq, Debug)] 
pub enum ITypeMne {
    JALR,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
}


impl FromStr for ITypeMne {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_ref() {
            "jalr" => Ok(ITypeMne::JALR),
            "lb" => Ok(ITypeMne::LB),
            "lh" => Ok(ITypeMne::LH),
            "lw" => Ok(ITypeMne::LW),
            "lbu" => Ok(ITypeMne::LBU),
            "lhu" => Ok(ITypeMne::LHU),
            "addi" => Ok(ITypeMne::ADDI),
            "slti" => Ok(ITypeMne::SLTI),
            "sltiu" => Ok(ITypeMne::SLTIU),
            "xori" => Ok(ITypeMne::XORI),
            "ori" => Ok(ITypeMne::ORI),
            "andi" => Ok(ITypeMne::ANDI),
            "slli" => Ok(ITypeMne::SLLI),
            "srli" => Ok(ITypeMne::SRLI),
            "srai" => Ok(ITypeMne::SRAI),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)] 
pub struct IType {
    pub mne: ITypeMne,
    pub rd: Reg,
    pub rs1: Reg,
    pub imm: Imm,
}

impl Instruction for IType {
    fn translate(&self) -> Vec<u8> {
        let opcode: u32 = match self.mne {
            // Jumps
            ITypeMne::JALR => 0x67,
            // Loads
            ITypeMne::LB | ITypeMne::LH | ITypeMne::LW | ITypeMne::LBU | ITypeMne::LHU => 0x03,
            // Normal ALU
            _ => 0x13,
        };

        let funct3: u32 = match self.mne {
            ITypeMne::JALR => 0b000,
            ITypeMne::LB => 0b000,
            ITypeMne::LH => 0b001,
            ITypeMne::LW => 0b010,
            ITypeMne::LBU => 0b100,
            ITypeMne::LHU => 0b100,
            ITypeMne::ADDI => 0b000,
            ITypeMne::SLTI => 0b010,
            ITypeMne::SLTIU => 0b011,
            ITypeMne::XORI => 0b100,
            ITypeMne::ORI => 0b110,
            ITypeMne::ANDI => 0b111,
            ITypeMne::SLLI => 0b001,
            ITypeMne::SRLI => 0b101,
            ITypeMne::SRAI => 0b101,
        };

        let imm = match self.mne {
            ITypeMne::SRAI => self.imm | 0x400,
            _ => self.imm,
        };

        let result: u32 =
            0 | (imm << 20) | (self.rs1 << 15) | (funct3 << 12) | (self.rd << 7) | opcode;
        result.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lb_test() {
        let instruction = IType {
            mne: ITypeMne::LB,
            rd: 1,
            rs1: 1,
            imm: 1,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x00, 0x10, 0x80, 0x83];
        assert_eq!(actual, expected);
    }

    #[test]
    fn addi_test() {
        let instruction = IType {
            mne: ITypeMne::ADDI,
            rd: 14,
            rs1: 21,
            imm: 123,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x07, 0xBA, 0x87, 0x13];
        assert_eq!(actual, expected);
    }

    #[test]
    fn srai_test() {
        let instruction = IType {
            mne: ITypeMne::SRAI,
            rd: 30,
            rs1: 5,
            imm: 12,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x40, 0xC2, 0xDF, 0x13];
        assert_eq!(actual, expected);
    }

    #[test]
    fn jalr_test() {
        let instruction = IType {
            mne: ITypeMne::JALR,
            rd: 23,
            rs1: 3,
            imm: 564,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x23, 0x41, 0x8B, 0xE7];
        assert_eq!(actual, expected);
    }
}
