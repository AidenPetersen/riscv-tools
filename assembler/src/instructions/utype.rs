use super::types::{Imm, Reg};
use super::instruction::Instruction;
use std::str::FromStr;

#[derive(PartialEq, Debug)] 
pub enum UTypeMne {
    LUI,
    AUIPC,
}

impl FromStr for UTypeMne {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_ref() {
            "lui" => Ok(UTypeMne::LUI),
            "auipc" => Ok(UTypeMne::AUIPC),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)] 
pub struct UType {
    pub mne: UTypeMne,
    pub rd: Reg,
    pub imm: Imm,
}

impl Instruction for UType {
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
