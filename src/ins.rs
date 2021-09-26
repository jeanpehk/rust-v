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
 * Conditional Branches
 */

pub fn beq(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return b_type(imm as u32, rs2, rs1, funct3::BEQ);
}

pub fn bne(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return b_type(imm as u32, rs2, rs1, funct3::BNE);
}

pub fn blt(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return b_type(imm as u32, rs2, rs1, funct3::BLT);
}

pub fn bltu(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return b_type(imm as u32, rs2, rs1, funct3::BLTU);
}

pub fn bge(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return b_type(imm as u32, rs2, rs1, funct3::BGE);
}

pub fn bgeu(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return b_type(imm as u32, rs2, rs1, funct3::BGEU);
}

/*
 * Load and Store
 */

pub fn lb(rd: u32, imm: i32, rs1: u32) -> u32 {
    return i_type(imm as u32, rs1, funct3::LB, rd, opcodes::LOAD);
}

pub fn lbu(rd: u32, imm: i32, rs1: u32) -> u32 {
    return i_type(imm as u32, rs1, funct3::LBU, rd, opcodes::LOAD);
}

pub fn lh(rd: u32, imm: i32, rs1: u32) -> u32 {
    return i_type(imm as u32, rs1, funct3::LH, rd, opcodes::LOAD);
}

pub fn lhu(rd: u32, imm: i32, rs1: u32) -> u32 {
    return i_type(imm as u32, rs1, funct3::LHU, rd, opcodes::LOAD);
}

pub fn lw(rd: u32, imm: i32, rs1: u32) -> u32 {
    return i_type(imm as u32, rs1, funct3::LW, rd, opcodes::LOAD);
}

/*
 * STORES: rs2 = read from, rs1 = base, imm = offset
 */

pub fn sb(rs2: u32, imm: i32, rs1: u32) -> u32 {
    return s_type(imm as u32, rs2, rs1, funct3::SB);
}

pub fn sh(rs2: u32, imm: i32, rs1: u32) -> u32 {
    return s_type(imm as u32, rs2, rs1, funct3::SH);
}

pub fn sw(rs2: u32, imm: i32, rs1: u32) -> u32 {
    return s_type(imm as u32, rs2, rs1, funct3::SW);
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

fn b_type(imm: u32, rs2: u32, rs1: u32, funct3: u32) -> u32 {
    let imm12 = (imm >> 12) & 0x1;
    let imm10_5 = (imm >> 5) & 0x3f;
    let imm4_1 = (imm >> 1) & 0xf;
    let imm11 = (imm >> 11) & 0x1;
    return (imm12 << 31)
        | (imm10_5 << 25)
        | ((rs2 & 0x1f) << 20)
        | ((rs1 & 0x1f) << 15)
        | ((funct3 & 0x7) << 12)
        | (imm4_1 << 8)
        | (imm11 << 7)
        | opcodes::BRANCH;
}

fn s_type(imm: u32, rs2: u32, rs1: u32, funct3: u32) -> u32 {
    let imm11_5 = (imm >> 5) & 0x7f;
    let imm4_0 = imm & 0x1f;
    return (imm11_5 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (imm4_0 << 7)
        | opcodes::STORE;
}
