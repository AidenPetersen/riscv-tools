use crate::instructions::types::{Imm, Reg};

use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag, tag_no_case};
use nom::character::complete::{
    alphanumeric0, alphanumeric1, char, digit0, multispace0, one_of, space0, space1, u32,
};
use nom::combinator::{cut, map, opt};
use nom::error::{context, VerboseError};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

use crate::instructions::InstructionData;

#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    pub instruction: InstructionData,
    pub label: Option<String>,
    pub label_dst: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataSize {
    Byte,
    Half,
    Word,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Data {
    pub label: Option<String>,
    pub data: Vec<u8>,
    pub size: DataSize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FullFile {
    pub text: Vec<Text>,
    pub data: Vec<Data>,
}

fn num_to_bytes(data: Vec<u32>, datasize: DataSize) -> Vec<u8>{
    match datasize {
        DataSize::Byte => {
            data.iter().map(|b| {
                b.to_be_bytes()[0] 
            }).collect()
        }
        DataSize::Half => {
            data.iter().flat_map(|h| {
                h.to_be_bytes()[0..1].to_vec()
            }).collect()
        }
        DataSize::Word => {
            data.iter().flat_map(|h| {
                h.to_be_bytes().to_vec()
            }).collect()
        }
    }
}

fn str_to_reg(s: &str) -> Option<Reg> {
    match s {
        "0" | "zero" => Some(0),
        "1" | "ra" => Some(1),
        "2" | "sp" => Some(2),
        "3" | "gp" => Some(3),
        "4" | "tp" => Some(4),
        "5" | "t0" => Some(5),
        "6" | "t1" => Some(6),
        "7" | "t2" => Some(7),
        "8" | "s0" | "fp" => Some(8),
        "9" | "s1" => Some(9),
        "10" | "a0" => Some(10),
        "11" | "a1" => Some(11),
        "12" | "a2" => Some(12),
        "13" | "a3" => Some(13),
        "14" | "a4" => Some(14),
        "15" | "a5" => Some(15),
        "16" | "a6" => Some(16),
        "17" | "a7" => Some(17),
        "18" | "s2" => Some(18),
        "19" | "s3" => Some(19),
        "20" | "s4" => Some(20),
        "21" | "s5" => Some(21),
        "22" | "s6" => Some(22),
        "23" | "s7" => Some(23),
        "24" | "s8" => Some(24),
        "25" | "s9" => Some(25),
        "26" | "s10" => Some(26),
        "27" | "s11" => Some(27),
        "28" | "t3" => Some(28),
        "29" | "t4" => Some(29),
        "30" | "t5" => Some(30),
        "31" | "t6" => Some(31),
        _ => None,
    }
}

fn parse_label(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    preceded(
        multispace0,
        terminated(is_not(" \t\r\n:"), terminated(tag(":"), multispace0)),
    )(i)
}

fn inside_par(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    preceded(
        space0,
        preceded(
            tag("("),
            preceded(
                space0,
                terminated(alphanumeric1, terminated(space0, tag(")"))),
            ),
        ),
    )(i)
}

fn reg(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    terminated(preceded(space0, alphanumeric1), space0)(i)
}

fn parse_imm(i: &str) -> IResult<&str, (Option<&str>, &str), VerboseError<&str>> {
    pair(opt(alt((tag("0x"), tag("0b")))), alphanumeric1)(i)
}

fn parse_load_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((
                tag_no_case("lbu"),
                tag_no_case("lhu"),
                tag_no_case("lb"),
                tag_no_case("lw"),
                tag_no_case("lh"),
            )),
        ),
        space1,
    );
    let rd_p = reg;
    let imm_p = preceded(
        // Remove beginning whitespace and comma
        preceded(preceded(space0, tag(",")), space0),
        // Gets numbers
        parse_imm,
    );
    let rs1_p = terminated(inside_par, multispace0);
    map(
        tuple((label_p, mne_p, rd_p, imm_p, rs1_p)),
        |(label, mne, rd, (imm1, imm2), rs1)| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: None,
                rd: str_to_reg(rd),
                imm: match imm1 {
                    Some("0x") => Some(u32::from_str_radix(imm2, 16).unwrap()),
                    Some("0b") => Some(u32::from_str_radix(imm2, 2).unwrap()),
                    None => Some(imm2.parse::<Imm>().unwrap()),
                    _ => unreachable!(),
                },
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

fn parse_store_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((tag_no_case("sw"), tag_no_case("sb"), tag_no_case("sh"))),
        ),
        space1,
    );
    let rs2_p = reg;
    let imm_p = preceded(
        // Remove beginning whitespace
        preceded(preceded(space0, tag(",")), space0),
        parse_imm,
    );
    let rs1_p = terminated(inside_par, multispace0);
    map(
        tuple((label_p, mne_p, rs2_p, imm_p, rs1_p)),
        |(label, mne, rs2, (imm1, imm2), rs1)| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: str_to_reg(rs2),
                rd: None,
                imm: match imm1 {
                    Some("0x") => Some(u32::from_str_radix(imm2, 16).unwrap()),
                    Some("0b") => Some(u32::from_str_radix(imm2, 2).unwrap()),
                    None => Some(imm2.parse::<Imm>().unwrap()),
                    _ => unreachable!(),
                },
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

fn parse_branch_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((
                tag_no_case("bltu"),
                tag_no_case("bgeu"),
                tag_no_case("beq"),
                tag_no_case("bne"),
                tag_no_case("blt"),
                tag_no_case("bge"),
            )),
        ),
        space1,
    );
    let rs1_p = reg;
    let rs2_p = preceded(tag(","), reg);
    let imm_p = terminated(
        preceded(
            // Remove beginning whitespace
            preceded(preceded(space0, tag(",")), space0),
            parse_imm,
        ),
        multispace0,
    );

    map(
        tuple((label_p, mne_p, rs1_p, rs2_p, imm_p)),
        |(label, mne, rs1, rs2, (imm1, imm2))| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: str_to_reg(rs2),
                rd: None,
                imm: match imm1 {
                    Some("0x") => Some(u32::from_str_radix(imm2, 16).unwrap()),
                    Some("0b") => Some(u32::from_str_radix(imm2, 2).unwrap()),
                    None => Some(imm2.parse::<Imm>().unwrap()),
                    _ => unreachable!(),
                },
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

fn parse_branch_pseudo_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((
                tag_no_case("bltu"),
                tag_no_case("bgeu"),
                tag_no_case("beq"),
                tag_no_case("bne"),
                tag_no_case("blt"),
                tag_no_case("bge"),
            )),
        ),
        space1,
    );
    let rs1_p = reg;
    let rs2_p = preceded(tag(","), reg);
    let label_dst_p = terminated(
        preceded(
            // Remove beginning whitespace
            preceded(preceded(space0, tag(",")), space0),
            is_not(" \t\r\n:"),
        ),
        multispace0,
    );

    map(
        tuple((label_p, mne_p, rs1_p, rs2_p, label_dst_p)),
        |(label, mne, rs1, rs2, label_dst)| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: str_to_reg(rs2),
                rd: None,
                imm: None,
            },

            label: label.map(|s| String::from(s)),
            label_dst: Some(label_dst.to_string()),
        },
    )(i)
}

fn parse_imm_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((
                tag_no_case("addi"),
                tag_no_case("xori"),
                tag_no_case("ori"),
                tag_no_case("andi"),
                tag_no_case("slli"),
                tag_no_case("srli"),
                tag_no_case("srai"),
                tag_no_case("slti"),
                tag_no_case("sltiu"),
            )),
        ),
        space1,
    );
    let rd_p = reg;
    let rs1_p = preceded(tag(","), reg);
    let imm_p = terminated(
        preceded(
            // Remove beginning whitespace
            preceded(preceded(space0, tag(",")), space0),
            parse_imm,
        ),
        multispace0,
    );

    map(
        tuple((label_p, mne_p, rd_p, rs1_p, imm_p)),
        |(label, mne, rd, rs1, (imm1, imm2))| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: None,
                rd: str_to_reg(rd),
                imm: match imm1 {
                    Some("0x") => Some(u32::from_str_radix(imm2, 16).unwrap()),
                    Some("0b") => Some(u32::from_str_radix(imm2, 2).unwrap()),
                    None => Some(imm2.parse::<Imm>().unwrap()),
                    _ => unreachable!(),
                },
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

fn parse_reg_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((
                tag_no_case("add"),
                tag_no_case("sub"),
                tag_no_case("xor"),
                tag_no_case("or"),
                tag_no_case("and"),
                tag_no_case("sll"),
                tag_no_case("srl"),
                tag_no_case("sra"),
                tag_no_case("slt"),
                tag_no_case("sltu"),
            )),
        ),
        space1,
    );
    let rd_p = reg;
    let rs1_p = preceded(tag(","), reg);
    let rs2_p = terminated(preceded(tag(","), reg), multispace0);

    map(
        tuple((label_p, mne_p, rd_p, rs1_p, rs2_p)),
        |(label, mne, rd, rs1, rs2)| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: str_to_reg(rs2),
                rd: str_to_reg(rd),
                imm: None,
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

fn parse_uj_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(
        preceded(
            space0,
            alt((tag_no_case("lui"), tag_no_case("auipc"), tag_no_case("jal"))),
        ),
        space1,
    );
    let rd_p = reg;
    let imm_p = terminated(preceded(space0, parse_imm), multispace0);
    map(
        tuple((label_p, mne_p, rd_p, imm_p)),
        |(label, mne, rd, (imm1, imm2))| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: None,
                rs2: None,
                rd: str_to_reg(rd),
                imm: match imm1 {
                    Some("0x") => Some(u32::from_str_radix(imm2, 16).unwrap()),
                    Some("0b") => Some(u32::from_str_radix(imm2, 2).unwrap()),
                    None => Some(imm2.parse::<Imm>().unwrap()),
                    _ => unreachable!(),
                },
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

fn parse_jal_pseudo_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(preceded(space0, tag_no_case("jal")), space1);
    let rd_p = reg;
    let label_dst_p = terminated(preceded(space0, is_not(" \n\t\r:")), multispace0);
    map(
        tuple((label_p, mne_p, rd_p, label_dst_p)),
        |(label, mne, rd, label_dst)| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: None,
                rs2: None,
                rd: str_to_reg(rd),
                imm: None,
            },
            label: label.map(|s| String::from(s)),
            label_dst: Some(label_dst.to_string()),
        },
    )(i)
}

fn parse_jalr_instr(i: &str) -> IResult<&str, Text, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let mne_p = terminated(preceded(space0, tag_no_case("jalr")), space1);
    let rd_p = reg;
    let imm_p = preceded(
        // Remove beginning whitespace and comma
        preceded(preceded(space0, tag(",")), space0),
        // Gets numbers
        parse_imm,
    );
    let rs1_p = terminated(inside_par, multispace0);
    map(
        tuple((label_p, mne_p, rd_p, imm_p, rs1_p)),
        |(label, mne, rd, (imm1, imm2), rs1)| Text {
            instruction: InstructionData {
                mne: mne.to_string(),
                rs1: str_to_reg(rs1),
                rs2: None,
                rd: str_to_reg(rd),
                imm: match imm1 {
                    Some("0x") => Some(u32::from_str_radix(imm2, 16).unwrap()),
                    Some("0b") => Some(u32::from_str_radix(imm2, 2).unwrap()),
                    None => Some(imm2.parse::<Imm>().unwrap()),
                    _ => unreachable!(),
                },
            },
            label: label.map(|s| String::from(s)),
            label_dst: None,
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_load_instr_test1() {
        let (leftover, result) = parse_load_instr("label: lw s1, 123(s2)").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "lw".to_string(),
                    rs1: Some(18),
                    rs2: None,
                    rd: Some(9),
                    imm: Some(123),
                },
                label: Some("label".to_string()),
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_load_instr_test2() {
        let (leftover, result) = parse_load_instr("lb s1, 0x1b3(s2)\n").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "lb".to_string(),
                    rs1: Some(18),
                    rs2: None,
                    rd: Some(9),
                    imm: Some(0x1b3),
                },
                label: None,

                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_load_instr_test3() {
        let (leftover, result) =
            parse_load_instr("\t label: \n lhu s1, 0b101 (  s2 )  \n").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "lhu".to_string(),
                    rs1: Some(18),
                    rs2: None,
                    rd: Some(9),
                    imm: Some(0b101),
                },
                label: Some("label".to_string()),

                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_store_instr_test1() {
        let (leftover, result) = parse_store_instr("label: sw s1, 123(s2)").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "sw".to_string(),
                    rs1: Some(18),
                    rs2: Some(9),
                    rd: None,
                    imm: Some(123),
                },
                label: Some("label".to_string()),

                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_branch_instr_test1() {
        let (leftover, result) = parse_branch_instr("label: beq s1, s2, 123").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "beq".to_string(),
                    rs1: Some(9),
                    rs2: Some(18),
                    rd: None,
                    imm: Some(123),
                },
                label: Some("label".to_string()),
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_branch_instr_test2() {
        let (leftover, result) = parse_branch_instr("\tblt s1, s2, 0xa23\n").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "blt".to_string(),
                    rs1: Some(9),
                    rs2: Some(18),
                    rd: None,
                    imm: Some(0xA23),
                },
                label: None,
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_branch_pseudo_instr_test1() {
        let (leftover, result) =
            parse_branch_pseudo_instr("label:\n blt s1, s2, label2\n").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "blt".to_string(),
                    rs1: Some(9),
                    rs2: Some(18),
                    rd: None,
                    imm: None,
                },
                label: Some("label".to_string()),
                label_dst: Some("label2".to_string()),
            }
        );
    }

    #[test]
    fn parse_branch_pseudo_instr_test2() {
        let (leftover, result) = parse_branch_pseudo_instr("bne zero, s2, label2").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "bne".to_string(),
                    rs1: Some(0),
                    rs2: Some(18),
                    rd: None,
                    imm: None,
                },
                label: None,
                label_dst: Some("label2".to_string()),
            }
        );
    }

    #[test]
    fn parse_imm_instr_test1() {
        let (leftover, result) = parse_imm_instr("hello: addi zero, ra, 0b101010").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "addi".to_string(),
                    rs1: Some(1),
                    rs2: None,
                    rd: Some(0),
                    imm: Some(0b101010),
                },
                label: Some("hello".to_string()),
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_reg_instr_test1() {
        let (leftover, result) = parse_reg_instr("hello: add zero, ra, sp").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "add".to_string(),
                    rs1: Some(1),
                    rs2: Some(2),
                    rd: Some(0),
                    imm: None,
                },
                label: Some("hello".to_string()),
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_reg_instr_test2() {
        let (leftover, result) = parse_reg_instr("hello:\n sll zero, ra, sp").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "sll".to_string(),
                    rs1: Some(1),
                    rs2: Some(2),
                    rd: Some(0),
                    imm: None,
                },
                label: Some("hello".to_string()),
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_reg_uj_test1() {
        let (leftover, result) = parse_uj_instr("hello:\njal zero 0x12312A").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "jal".to_string(),
                    rs1: None,
                    rs2: None,
                    rd: Some(0),
                    imm: Some(0x12312A),
                },
                label: Some("hello".to_string()),
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_reg_uj_test2() {
        let (leftover, result) = parse_uj_instr("lui s1 0x12312A").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "lui".to_string(),
                    rs1: None,
                    rs2: None,
                    rd: Some(9),
                    imm: Some(0x12312A),
                },
                label: None,
                label_dst: None,
            }
        );
    }

    #[test]
    fn parse_reg_jal_pseudo_test1() {
        let (leftover, result) = parse_jal_pseudo_instr("jal s1 cool_label").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "jal".to_string(),
                    rs1: None,
                    rs2: None,
                    rd: Some(9),
                    imm: None,
                },
                label: None,
                label_dst: Some("cool_label".to_string()),
            }
        );
    }

    #[test]
    fn parse_reg_jal_pseudo_test2() {
        let (leftover, result) = parse_jal_pseudo_instr("label: jal zero anotherLabel").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "jal".to_string(),
                    rs1: None,
                    rs2: None,
                    rd: Some(0),
                    imm: None,
                },
                label: Some("label".to_string()),
                label_dst: Some("anotherLabel".to_string()),
            }
        );
    }

    #[test]
    fn parse_reg_jalr_test1() {
        let (leftover, result) = parse_jalr_instr("label: jalr zero, 0xabc(ra)").unwrap();
        assert_eq!(
            result,
            Text {
                instruction: InstructionData {
                    mne: "jalr".to_string(),
                    rs1: Some(1),
                    rs2: None,
                    rd: Some(0),
                    imm: Some(0xabc),
                },
                label: Some("label".to_string()),
                label_dst: None,
            }
        );
    }
}

fn parse_text(i: &str) -> IResult<&str, Vec<Text>, VerboseError<&str>> {
    preceded(
        multispace0,
        preceded(
            opt(tag(".text")),
            preceded(
                multispace0,
                many0(alt((
                    parse_load_instr,
                    parse_store_instr,
                    parse_branch_instr,
                    parse_branch_pseudo_instr,
                    parse_imm_instr,
                    parse_reg_instr,
                    parse_uj_instr,
                    parse_jal_pseudo_instr,
                    parse_jalr_instr,
                ))),
            ),
        ),
    )(i)
}

fn parse_string(i: &str) -> IResult<&str, Data, VerboseError<&str>> {
    let label_p = opt(parse_label);
    let dir = terminated(
        preceded(space0, alt((tag_no_case(".string"), tag_no_case(".asciz")))),
        multispace0,
    );
    let string = terminated(
        preceded(
            char('\"'),
            cut(terminated(
                escaped(alphanumeric1, '\\', one_of("\"n\\")),
                char('\"'),
            )),
        ),
        multispace0,
    );

   map(
       tuple((label_p, dir, string)),
       |(label, _, string)| Data {
           label: label.map(|s| String::from(s)),
           size: DataSize::Byte,
           data: string.to_string().into_bytes()
       },
   )(i)
}

fn parse_datasize(i: &str) -> IResult<&str, DataSize, VerboseError<&str>> {
    alt((
        map(tag(".word"), |_| DataSize::Word),
        map(tag(".half"), |_| DataSize::Half),
        map(tag(".byte"), |_| DataSize::Byte),
    ))(i)
}

fn parse_datalist(i: &str) -> IResult<&str, Vec<u8>, VerboseError<&str>> {
    map(
    terminated(preceded(multispace0, separated_list1(preceded(multispace0, terminated(char(','), multispace0)), parse_imm)), multispace0)
    |(pre, num)| {

    }
    )
}

fn parse_dataline (i: &str) -> IResult<&str, Data, VerboseError<&str>> {

}
//fn parse_data(i: &str) -> IResult<&str, Vec<Text>, VerboseError<&str>> {
//    preceded(multispace0, preceded(tag(".data"), ))

//fn parse(i: &str) -> IResult<&str, Vec<Text>, VerboseError<&str>> {
//    many0(alt((parse_text, parse_data)))(i)
//}
