use crate::constants::opcodes;
use crate::constants::funct3;

/*
 * Instruction encoding for creating basic test programs.
 */

/*
 * Integer Register-Immediate Instructions
 */

pub fn addi(rd: u32, rs1: u32, imm: u32) -> u32 {
    return i_imm(imm, rs1, funct3::ADDI, rd);
}

pub fn slti(rd: u32, rs1: u32, imm: u32) -> u32 {
    return i_imm(imm, rs1, funct3::SLTI, rd);
}

pub fn sltiu(rd: u32, rs1: u32, imm: u32) -> u32 {
    return i_imm(imm, rs1, funct3::SLTIU, rd);
}

pub fn andi(rd: u32, rs1: u32, imm: u32) -> u32 {
    return i_imm(imm, rs1, funct3::ANDI, rd);
}

pub fn ori(rd: u32, rs1: u32, imm: u32) -> u32 {
    return i_imm(imm, rs1, funct3::ORI, rd);
}

pub fn xori(rd: u32, rs1: u32, imm: u32) -> u32 {
    return i_imm(imm, rs1, funct3::XORI, rd);
}

pub fn slli(rd: u32, rs1: u32, shamt: u32) -> u32 {
    return i_imm(shamt, rs1, funct3::SLLI, rd);
}

pub fn srli(rd: u32, rs1: u32, shamt: u32) -> u32 {
    return i_imm(shamt, rs1, funct3::SRXI, rd);
}

pub fn srai(rd: u32, rs1: u32, shamt: u32) -> u32 {
    let imm = (1 << 10) | shamt;
    return i_imm(imm, rs1, funct3::SRXI, rd);
}

pub fn lui(rd: u32, imm: u32) -> u32 {
    return (imm >> 12) | (rd >> 7) | opcodes::LUI;
}

pub fn auipc(rd: u32, imm: u32) -> u32 {
    return (imm >> 12) | (rd >> 7) | opcodes::AUIPC;
}

/*
 * Integer Register-Register Operations
 */

pub fn add(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::ADD_SUB, rd);
}

pub fn slt(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::SLT, rd);
}

pub fn sltu(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::SLTU, rd);
}

pub fn and(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::AND, rd);
}

pub fn or(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::OR, rd);
}

pub fn xor(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::XOR, rd);
}

pub fn sll(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::SLL, rd);
}
pub fn srl(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0, rs2, rs1, funct3::SRX, rd);
}
pub fn sub(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0x20, rs2, rs1, funct3::ADD_SUB, rd);
}
pub fn sra(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return r_type(0x20, rs2, rs1, funct3::SRX, rd);
}

pub fn nop() -> u32 {
    return addi(0, 0, 0);
}

/*
 * Instruction types
 */

fn i_imm(imm: u32, rs1: u32, funct3: u32, rd: u32) -> u32 {
    return ((imm & 0xfff) << 20)
        | ((rs1 & 0x1f) << 15)
        | ((funct3 & 0x7) << 12)
        | ((rd & 0x1f) << 7)
        | opcodes::OP_IMM;
}

fn r_type(funct7: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32) -> u32 {
    return ((funct7 & 0x7f) << 25)
        | ((rs2 & 0x1f) << 20)
        | ((rs1 & 0x1f) << 15)
        | ((funct3 & 0x7) << 12)
        | ((rd & 0x1f) << 7)
        | (opcodes::OP);
}

