pub const MEMSIZE: usize = 4 * 1024;
pub const REG_NAMES: [&str; 33] = [
    "zero",
    "ra",
    "sp",
    "gp",
    "tp",
    "t0",
    "t1",
    "t2",
    "s0",
    "s1",
    "a0",
    "a1",
    "a2",
    "a3",
    "a4",
    "a5",
    "a6",
    "a7",
    "s2",
    "s3",
    "s4",
    "s5",
    "s6",
    "s7",
    "s8",
    "s9",
    "s10",
    "s11",
    "t3",
    "t4",
    "t5",
    "t6",
    "pc"
];

pub mod opcodes {
    pub const TEST: u32 = 0;
    pub const OP: u32 = 0b0110011;
    pub const OP_IMM: u32 = 0b0010011;
    pub const LUI: u32 = 0b0110111;
    pub const AUIPC: u32 = 0b0010111;
    pub const JAL: u32 = 0b1101111;
    pub const JALR: u32 = 0b1100111;
    pub const BRANCH: u32 = 0b1100011;
    pub const LOAD: u32 = 0b0000011;
    pub const STORE: u32 = 0b0100011;
    pub const MISCMEM: u32 = 0b0001111;
}

pub mod funct3 {
    // Integer Register Immediate
    pub const ADDI: u32 = 0b000;
    pub const SLTI: u32 = 0b010;
    pub const SLTIU: u32 = 0b011;
    pub const ANDI: u32 = 0b111;
    pub const ORI: u32 = 0b110;
    pub const XORI: u32 = 0b100;
    pub const SLLI: u32 = 0b001;
    pub const SRXI: u32 = 0b101;

    // Integer Register Register
    pub const ADD: u32 = 0b000;
    pub const SLT: u32 = 0b010;
    pub const SLTU: u32 = 0b011;
    pub const AND: u32 = 0b111;
    pub const OR: u32 = 0b110;
    pub const XOR: u32 = 0b100;
    pub const SLL: u32 = 0b001;
    // SRL or SRA
    pub const SRX: u32 = 0b101;
    pub const _SUB: u32 = 0b000;

    // Branch
    pub const BEQ: u32 = 0b000;
    pub const BNE: u32 = 0b001;
    pub const BLT: u32 = 0b100;
    pub const BLTU: u32 = 0b110;
    pub const BGE: u32 = 0b101;
    pub const BGEU: u32 = 0b111;

    // Load/Store
    pub const LB: u32 = 0b000;
    pub const LH: u32 = 0b001;
    pub const LW: u32 = 0b010;
    pub const LBU: u32 = 0b100;
    pub const LHU: u32 = 0b101;
    pub const SB: u32 = 0b000;
    pub const SH: u32 = 0b001;
    pub const SW: u32 = 0b010;

    pub const FENCE: u32 = 0b000;
}
