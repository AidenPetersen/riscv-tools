use super::types::{Imm, Reg};
use super::Translate;

enum JTypeMne {
    JAL,
}

pub struct JType {
    mne: JTypeMne,
    rd: Reg,
    imm: Imm,
}

impl Translate for JType {
    fn translate(&self) -> Vec<u8> {
        let opcode: u32 = match self.mne {
            JTypeMne::JAL => 0x6F,
        };
        let imm20 = (self.imm >> 20) & 1;
        let imm10_1 = (self.imm >> 1) & 0x3FF;
        let imm11 = (self.imm >> 11) & 1;
        let imm19_12 = (self.imm >> 12) & 0xFF;
        let ordered_imm = (imm20 << 19) | (imm10_1 << 9) | (imm11 << 8) | imm19_12;
        let result: u32 = 0 | (ordered_imm << 12) | (self.rd << 7) | opcode;
        result.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jal1_test() {
        let instruction = JType {
            mne: JTypeMne::JAL,
            rd: 1,
            imm: 0b101010101010101010101,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0xD5, 0x45, 0x50, 0xEF];
        assert_eq!(actual, expected);
    }

    #[test]
    fn jal2_test() {
        let instruction = JType {
            mne: JTypeMne::JAL,
            rd: 21,
            imm: 0b100111010001010011011,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0xA9, 0xA3, 0xAA, 0xEF];
        assert_eq!(actual, expected);
    }
}
