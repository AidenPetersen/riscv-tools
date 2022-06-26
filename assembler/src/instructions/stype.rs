use super::types::{Imm, Reg};
use super::Translate;

enum STypeMne {
    SB,
    SH,
    SW,
}

pub struct SType {
    mne: STypeMne,
    rs2: Reg,
    imm: Imm,
    rs1: Reg,
}

impl Translate for SType {
    fn translate(&self) -> Vec<u8> {
        let opcode = 0b0100011;
        let funct3 = match self.mne {
            STypeMne::SB => 0b000,
            STypeMne::SH => 0b001,
            STypeMne::SW => 0b010,
        };
        let imm11_5 = (self.imm >> 5) & 0x7F;
        let imm4_0 = self.imm & 0x1F;
        let result = 0
            | (imm11_5 << 25)
            | (self.rs2 << 20)
            | (self.rs1 << 15)
            | (funct3 << 12)
            | (imm4_0 << 7)
            | opcode;

        result.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sw_test() {
        let instruction = SType {
            mne: STypeMne::SW,
            rs2: 10,
            imm: 0b100101101010,
            rs1: 3,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x96, 0xA1, 0xA5, 0x23];
        assert_eq!(actual, expected);
    }

    #[test]
    fn sb_test() {
        let instruction = SType {
            mne: STypeMne::SB,
            rs2: 2,
            imm: 0b001010011100,
            rs1: 24,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x28, 0x2C, 0x0E, 0x23];
        assert_eq!(actual, expected);
    }
}
