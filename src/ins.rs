use crate::constants::opcodes;
use crate::constants::funct3;

/*
 * Instruction encoding for creating basic test programs.
 *
 *
 * Integer Register-Immediate Instructions:
 */

pub fn addi(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, funct3::ADDI, rd, opcodes::OP_IMM);
}

pub fn slti(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, funct3::SLTI, rd, opcodes::OP_IMM);
}

pub fn sltiu(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, funct3::SLTIU, rd, opcodes::OP_IMM);
}

pub fn andi(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, funct3::ANDI, rd, opcodes::OP_IMM);
}

pub fn ori(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, funct3::ORI, rd, opcodes::OP_IMM);
}

pub fn xori(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, funct3::XORI, rd, opcodes::OP_IMM);
}

pub fn slli(rd: u32, rs1: u32, shamt: u32) -> u32 {
    return i_type(shamt, rs1, funct3::SLLI, rd, opcodes::OP_IMM);
}

pub fn srli(rd: u32, rs1: u32, shamt: u32) -> u32 {
    return i_type(shamt, rs1, funct3::SRXI, rd, opcodes::OP_IMM);
}

pub fn srai(rd: u32, rs1: u32, shamt: u32) -> u32 {
    let imm = (1 << 10) | shamt;
    return i_type(imm, rs1, funct3::SRXI, rd, opcodes::OP_IMM);
}

pub fn lui(rd: u32, imm: i32) -> u32 {
    return u_type(imm as u32, rd, opcodes::LUI);
}

pub fn auipc(rd: u32, imm: i32) -> u32 {
    return u_type(imm as u32, rd, opcodes::AUIPC);
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
 * Control Transfer Instructions
 */

pub fn jal(rd: u32, imm: i32) -> u32 {
    return j_type(imm as u32, rd);
}

pub fn jalr(rd: u32, rs1: u32, imm: i32) -> u32 {
    return i_type(imm as u32, rs1, 0, rd, opcodes::JALR);
}

/*
 * Instruction types
 */

fn r_type(funct7: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32) -> u32 {
    return ((funct7 & 0x7f) << 25)
        | ((rs2 & 0x1f) << 20)
        | ((rs1 & 0x1f) << 15)
        | ((funct3 & 0x7) << 12)
        | ((rd & 0x1f) << 7)
        | (opcodes::OP);
}

fn j_type(imm: u32, rd: u32) -> u32 {
    let imm = imm >> 1;
    let imm20 = (imm >> 19) & 0x1;
    let imm10_1 = imm & 0x3ff;
    let imm11 = (imm >> 10) & 0x1;
    let imm19_12 = (imm >> 11) & 0xff;
    let offset20_1 = (imm20 << 31)
        | (imm10_1 << 21)
        | (imm11 << 20)
        | (imm19_12 << 12);
    return offset20_1 | ((rd & 0x1f) << 7) | opcodes::JAL;
}

fn u_type(imm: u32, rd: u32, opcode: u32) -> u32 {
    return (((imm as u32 & 0xfffff) << 12))
        | ((rd & 0x1f) << 7)
        | (opcode & 0x7f);
}

fn i_type(imm: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    return ((imm & 0xfff) << 20)
        | ((rs1 & 0x1f) << 15)
        | ((funct3 & 0x7) << 12)
        | ((rd & 0x1f) << 7)
        | (opcode & 0x7f);
}

