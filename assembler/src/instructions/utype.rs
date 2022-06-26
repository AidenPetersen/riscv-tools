use super::types::{Imm, Reg};
use super::Translate;

enum UTypeMne {
    LUI,
    AUIPC,
}

pub struct UType {
    mne: UTypeMne,
    rd: Reg,
    imm: Imm,
}

impl Translate for UType {
    fn translate(&self) -> Vec<u8> {
        let opcode = match self.mne {
            UTypeMne::LUI => 0b0110111,
            UTypeMne::AUIPC => 0b0010111,
        };

        let result: u32 = 0 | (self.imm << 12) | (self.rd << 7) | opcode;

        result.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lui_test() {
        let instruction = UType {
            mne: UTypeMne::LUI,
            rd: 12,
            imm: 0xDEAD,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x0D, 0xEA, 0xD6, 0x37];
        assert_eq!(actual, expected);
    }

    #[test]
    fn auipc_test() {
        let instruction = UType {
            mne: UTypeMne::AUIPC,
            rd: 1,
            imm: 0xD1DF2,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0xD1, 0xDF, 0x20, 0x97];
        assert_eq!(actual, expected);
    }
}
