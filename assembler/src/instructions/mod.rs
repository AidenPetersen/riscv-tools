mod btype;
pub mod instruction;
mod itype;
mod jtype;
mod rtype;
mod stype;
pub mod types;
mod utype;

use std::str::FromStr;

use self::btype::*;
use self::instruction::Instruction;
use self::itype::*;
use self::jtype::*;
use self::rtype::*;
use self::stype::*;
use self::types::{Imm, Reg};
use self::utype::*;

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionData {
    pub mne: String,
    pub rd: Option<Reg>,
    pub rs1: Option<Reg>,
    pub rs2: Option<Reg>,
    pub imm: Option<Imm>,
}

/// Could crash easily. Returns the corresponding instruction object.
/// If this crashes though, the parser is either wrong, or the assembly
/// syntax is incorrect.
pub fn generate_instruction(data: InstructionData) -> Box<dyn Instruction> {
    match data.mne.to_lowercase().as_ref() {
        // B type
        "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => {
            let rs1 = data.rs1.unwrap();
            let rs2 = data.rs2.unwrap();
            let imm = data.imm.unwrap();

            Box::new(BType {
                mne: BTypeMne::from_str(data.mne.as_ref()).unwrap(),
                rs1,
                rs2,
                imm,
            })
        }
        // I type
        "jalr" | "lb" | "lh" | "lw" | "lbu" | "lhu" | "addi" | "slti" | "sltiu" | "xori"
        | "ori" | "andi" | "slli" | "srli" | "srai" => {
            let rd = data.rd.unwrap();
            let rs1 = data.rs1.unwrap();
            let imm = data.imm.unwrap();

            Box::new(IType {
                mne: ITypeMne::from_str(data.mne.as_ref()).unwrap(),
                rd,
                rs1,
                imm,
            })
        }
        // J type
        "jal" => {
            let rd = data.rd.unwrap();
            let imm = data.imm.unwrap();

            Box::new(JType {
                mne: JTypeMne::from_str(data.mne.as_ref()).unwrap(),
                rd,
                imm,
            })
        }
        // R type
        "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "srl" | "sra" | "or" | "and" => {
            let rd = data.rd.unwrap();
            let rs1 = data.rs1.unwrap();
            let rs2 = data.rs2.unwrap();

            Box::new(RType {
                mne: RTypeMne::from_str(data.mne.as_ref()).unwrap(),
                rd,
                rs1,
                rs2,
            })
        }
        // S type
        "sb" | "sh" | "sw" => {
            let rs1 = data.rs1.unwrap();
            let rs2 = data.rs2.unwrap();
            let imm = data.imm.unwrap();

            Box::new(SType {
                mne: STypeMne::from_str(data.mne.as_ref()).unwrap(),
                rs1,
                rs2,
                imm,
            })
        }
        // U type
        "lui" | "auipc" => {
            let rd = data.rd.unwrap();
            let imm = data.imm.unwrap();
            Box::new(UType {
                mne: UTypeMne::from_str(data.mne.as_ref()).unwrap(),
                rd,
                imm,
            })
        }
        _ => {
            panic!("Invalid mnemonic")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btype_test() {
        let actual = generate_instruction(InstructionData {
            mne: String::from("beq"),
            rd: None,
            rs1: Some(21),
            rs2: Some(12),
            imm: Some(1234),
        });

        let expected = BType {
            mne: BTypeMne::BEQ,
            rs1: 21,
            rs2: 12,
            imm: 1234,
        };
        assert_eq!(expected.translate(), actual.translate())
    }

    #[test]
    fn itype_test() {
        let actual = generate_instruction(InstructionData {
            mne: String::from("lb"),
            rd: Some(12),
            rs1: Some(23),
            rs2: None,
            imm: Some(1234),
        });

        let expected = IType {
            mne: ITypeMne::LB,
            rd: 12,
            rs1: 23,
            imm: 1234,
        };
        assert_eq!(expected.translate(), actual.translate())
    }

    #[test]
    fn jtype_test() {
        let actual = generate_instruction(InstructionData {
            mne: String::from("jal"),
            rd: Some(12),
            rs1: None,
            rs2: None,
            imm: Some(1234),
        });

        let expected = JType {
            mne: JTypeMne::JAL,
            rd: 12,
            imm: 1234,
        };
        assert_eq!(expected.translate(), actual.translate())
    }

    #[test]
    fn rtype_test() {
        let actual = generate_instruction(InstructionData {
            mne: String::from("add"),
            rd: Some(12),
            rs1: Some(13),
            rs2: Some(14),
            imm: None,
        });

        let expected = RType {
            mne: RTypeMne::ADD,
            rd: 12,
            rs1: 13,
            rs2: 14,
        };
        assert_eq!(expected.translate(), actual.translate())
    }

    #[test]
    fn stype_test() {
        let actual = generate_instruction(InstructionData {
            mne: String::from("sw"),
            rd: None,
            rs1: Some(13),
            rs2: Some(14),
            imm: Some(1234),
        });

        let expected = SType {
            mne: STypeMne::SW,
            rs1: 13,
            rs2: 14,
            imm: 1234,
        };
        assert_eq!(expected.translate(), actual.translate())
    }

    #[test]
    fn utype_test() {
        let actual = generate_instruction(InstructionData {
            mne: String::from("lui"),
            rd: Some(12),
            rs1: None,
            rs2: None,
            imm: Some(1234),
        });

        let expected = UType {
            mne: UTypeMne::LUI,
            rd: 12,
            imm: 1234,
        };
        assert_eq!(expected.translate(), actual.translate())
    }
}
