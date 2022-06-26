use super::types::Reg;
use super::Translate;

enum RTypeMne {
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
}

pub struct RType {
    mne: RTypeMne,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

impl Translate for RType {
    fn translate(&self) -> Vec<u8> {
        let opcode: u32 = 0b0110011;
        let funct7: u32 = match self.mne {
            RTypeMne::SRA | RTypeMne::SUB => 0x20,
            _ => 0,
        };
        let funct3: u32 = match self.mne {
            RTypeMne::ADD => 0b000,
            RTypeMne::SUB => 0b000,
            RTypeMne::SLL => 0b001,
            RTypeMne::SLT => 0b010,
            RTypeMne::SLTU => 0b011,
            RTypeMne::XOR => 0b100,
            RTypeMne::SRL => 0b101,
            RTypeMne::SRA => 0b101,
            RTypeMne::OR => 0b110,
            RTypeMne::AND => 0b111,
        };
        let result: u32 = 0
            | (funct7 << 25)
            | (self.rs2 << 20)
            | (self.rs1 << 15)
            | (funct3 << 12)
            | (self.rd << 7)
            | opcode;
        result.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let instruction = RType {
            mne: RTypeMne::ADD,
            rd: 1,
            rs1: 1,
            rs2: 1,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x00, 0x10, 0x80, 0xB3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn sub_test() {
        let instruction = RType {
            mne: RTypeMne::SUB,
            rd: 31,
            rs1: 4,
            rs2: 13,
        };
        let actual: Vec<u8> = instruction.translate();
        let expected: Vec<u8> = vec![0x40, 0xD2, 0x0F, 0xB3];
        assert_eq!(actual, expected);
    }
}
