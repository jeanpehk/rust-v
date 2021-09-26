/*
 * TODO
 * cleanup when done
 */
#![ allow( dead_code ) ]
#![ allow( unused_imports ) ]
#![ allow( unused_variables ) ]
#![ allow( unused_mut ) ]

mod test;
mod constants;
mod ins;

use constants::MEMSIZE;
use constants::REG_NAMES;
use constants::funct3;
use constants::opcodes;
use ins::*;

/*
 * Main structure for core state
 */
struct Core {
    memory: [u8; MEMSIZE],
    regs: [i32;33]
}

fn run(core: &mut Core) {
    let mut ins_cnt = 0;
    loop {
        let pc = core.regs[32] as usize;
        let ins = ((core.memory[pc+3] as u32) << 24)
            | ((core.memory[pc+2] as u32) << 16)
            | ((core.memory[pc+1] as u32) << 8)
            | core.memory[pc] as u32;
        if ins == 0 { break; };
        eval(ins, core);
        ins_cnt += 1;
    }
    println!("Ran {} instructions.", ins_cnt);
}

fn store_mem_32(mut core: &mut Core, addr: u32, value: u32) {
    let addr = addr as usize;
    core.memory[addr] = (value & 0xff) as u8;
    core.memory[addr+1] = ((value>>8) & 0xff) as u8;
    core.memory[addr+2] = ((value>>16) & 0xff) as u8;
    core.memory[addr+3] = ((value>>24) & 0xff) as u8;
}

struct IType {
    imm: u32,
    rs1: usize,
    rd: usize,
    funct3: u32
}

struct UType {
    rd: usize,
    imm: u32
}

struct RType {
    funct7: u32,
    rs2: usize,
    rs1: usize,
    funct3: u32,
    rd: usize
}

struct JType {
    imm: u32,
    rd: usize
}

struct BType {
    imm: u32,
    rs2: usize,
    rs1: usize,
    funct3: u32
}

struct SType {
    imm: u32,
    rs2: usize,
    rs1: usize,
    funct3: u32
}

fn get_i_type(ins: u32) -> IType {
    let imm = take_range(31,20,ins);
    let rs1 = take_range(19,15,ins) as usize;
    let rd = take_range(11,7,ins) as usize;
    let funct3 = take_range(14,12,ins);
    return IType { imm, rs1, rd, funct3 };
}

fn get_u_type(ins: u32) -> UType {
    let rd = take_range(11,7,ins) as usize;
    let imm = take_range(31,12,ins);
    return UType { rd, imm };
}

fn get_r_type(ins: u32) -> RType {
    let funct7 = take_range(31,25,ins);
    let rs2 = take_range(24,20,ins) as usize;
    let rs1 = take_range(19,15,ins) as usize;
    let funct3 = take_range(14,12,ins);
    let rd = take_range(11,7,ins) as usize;
    return RType { funct7, rs2, rs1, funct3, rd };
}

fn get_j_type(ins: u32) -> JType {
    let imm20 = take_range(31,31,ins);
    let imm10_1 = take_range(30,21,ins);
    let imm11 = take_range(20,20,ins);
    let imm19_12 = take_range(19,12,ins);
    let imm = (imm20<<20) | (imm19_12<<12) | (imm11<<11) | (imm10_1<<1);
    let rd = take_range(11,7,ins) as usize;
    return JType { imm, rd };
}

fn get_b_type(ins: u32) -> BType {
    let imm12 = take_range(31,31,ins);
    let imm10_5 = take_range(30,25,ins);
    let rs2 = take_range(24,20,ins) as usize;
    let rs1 = take_range(19,15,ins) as usize;
    let funct3 = take_range(14,12,ins);
    let imm4_1 = take_range(11,8,ins);
    let imm11 = take_range(7,7,ins);
    let imm = (imm12<<12)|(imm11<<11)|(imm10_5<<5)|(imm4_1<<1);
    return BType { imm, rs2, rs1, funct3 };
}

fn get_s_type(ins: u32) -> SType {
    let imm11_5 = take_range(31,25,ins);
    let rs2 = take_range(24,20,ins) as usize;
    let rs1 = take_range(19,15,ins) as usize;
    let funct3 = take_range(14,12,ins);
    let imm4_0 = take_range(11,7,ins);
    let imm = (imm11_5<<5) | imm4_0;
    return SType { imm, rs2, rs1, funct3 };
}

pub fn take_range(start: u32, end: u32, ins: u32) -> u32 {
    return (ins >> end) & ((1 << (start-end+1))-1);
}

/*
 * dump 10 bytes starting from index
 */
fn dump_mem(core: &Core, addr: usize) {
    let range = 10;
    let mut i = 0;
    println!("{:11} {:5} {:3}", "Memory", "Dec",  "Hex");
    println!("{:11} {:5} {:3}", "------", "---", "---");
    while i < range {
        println!("{:#010x}: {:<#5} {:<#02x}", addr+i, core.memory[addr+i], core.memory[addr+i]);
        i += 1;
    }
}

fn dump_regs(core: &Core) {
    let regs = core.regs;
    println!("{:6} {:<10} {}", "Name", "Dec", "Hex");
    println!("{:6} {:<10} {}", "----", "---", "---");
    for i in 0..=32 {
        println!("{:6} {:<10} {:#010x}", REG_NAMES[i], regs[i], regs[i]);
    }
}

fn sign_extend(ins: u32, bits: u32) -> i32 {
    let sign_bit = take_range(bits-1, bits-1, ins);
    return if sign_bit == 0 {
        ins as i32
    }
    else {
        (u32::pow(2, bits)-ins) as i32 * -1
    };
}

fn eval(ins: u32, core: &mut Core) {
    let opcode = take_range(6, 0, ins);

    match opcode {
        opcodes::OP_IMM => {
            let IType { imm, rs1, rd, funct3 } = get_i_type(ins);
            let signed_imm = sign_extend(imm, 12);

            match funct3 {
                funct3::ADDI => {
                    core.regs[rd] = core.regs[rs1] + signed_imm
                },
                funct3::SLTI => {
                    core.regs[rd] = if core.regs[rs1] < signed_imm {1} else {0};
                },
                funct3::SLTIU => {
                    core.regs[rd] =
                        if (core.regs[rs1] as u32) < signed_imm as u32 {1} else {0};
                },
                funct3::ANDI => {
                    core.regs[rd] = core.regs[rs1] & signed_imm;
                },
                funct3::ORI => {
                    core.regs[rd] = core.regs[rs1] | signed_imm;
                },
                funct3::XORI => {
                    core.regs[rd] = core.regs[rs1] ^ signed_imm;
                },
                funct3::SLLI => {
                    let shamt = imm & 0b11111;
                    core.regs[rd] = core.regs[rs1] << shamt;
                },
                funct3::SRXI => {
                    let arithmetic = take_range(30,30, ins);
                    let shamt = imm & 0b11111;
                    if arithmetic == 1 { // SRAI
                        core.regs[rd] = core.regs[rs1] >> shamt;
                    }
                    else { // SRLI
                        let ans = core.regs[rs1] as u32 >> shamt;
                        core.regs[rd] = ans as i32;
                    }
                },
                _ => {
                    println!("Unknown funct3 in op_imm: {}", funct3);
                }
            }
        },
        opcodes::OP => {
            let RType { funct7, rs2, rs1, funct3, rd } = get_r_type(ins);
            match funct3 {
                funct3::ADD_SUB => {
                    if funct7 == 0 { // add
                        core.regs[rd] = core.regs[rs1] + core.regs[rs2];
                    }
                    else { // sub
                        core.regs[rd] = core.regs[rs1] - core.regs[rs2];
                    }
                },
                funct3::SLT => {
                    core.regs[rd] = if core.regs[rs1] < core.regs[rs2] {1} else {0};
                },
                funct3::SLTU => {
                    let (lhs, rhs) = (core.regs[rs1] as u32, core.regs[rs2] as u32);
                    core.regs[rd] = if lhs < rhs {1} else {0};
                },
                funct3::XOR => {
                    core.regs[rd] = core.regs[rs1] ^ core.regs[rs2];
                },
                funct3::SLL => {
                    let shamt = core.regs[rs2] & 0b11111;
                    core.regs[rd] = core.regs[rs1] << shamt;
                },
                funct3::SRX => {
                    let shamt = core.regs[rs2] & 0b11111;
                    core.regs[rd] = if funct7 == 0 { // SRL
                        (core.regs[rs1] as u32 >> shamt) as i32
                    }
                    else { // SRA
                        core.regs[rs1] >> shamt
                    };
                },
                funct3::OR => {
                    core.regs[rd] = core.regs[rs1] | core.regs[rs2];
                },
                funct3::AND => {
                    core.regs[rd] = core.regs[rs1] & core.regs[rs2];
                },
                _ => {
                    println!("Unknown Funct3 in Opcode Op: {}", funct3);
                }
            }
        },
        opcodes::LUI => {
            let UType { rd, imm } = get_u_type(ins);
            core.regs[rd] = (imm << 12) as i32;
        },
        opcodes::AUIPC => {
            let UType { rd, imm } = get_u_type(ins);
            core.regs[rd] = core.regs[32] + ((imm<<12) as i32);
        },
        opcodes::JAL => {
            let JType { imm, rd } = get_j_type(ins);
            let signed = sign_extend(imm, 21);
            core.regs[rd] = core.regs[32]+4;
            core.regs[32] = core.regs[32]+signed;
            return;
        },
        opcodes::JALR => {
            let IType { imm, rs1, rd, funct3: _ } = get_i_type(ins);

            let imm = sign_extend(imm, 12);
            let val = imm+core.regs[rs1];

            core.regs[rd] = core.regs[32]+4;
            core.regs[32] = if val%2 == 0 {val} else {val-1};
            return;
        },
        opcodes::BRANCH => {
            let BType { imm, rs2, rs1, funct3 } = get_b_type(ins);
            let imm = sign_extend(imm,13);
            let target_addr = (((core.regs[32] + imm) as usize)%MEMSIZE) as i32;
            match funct3 {
                funct3::BEQ => {
                    if core.regs[rs1] == core.regs[rs2] {
                        core.regs[32] = target_addr;
                        return;
                    }
                },
                funct3::BNE => {
                    if core.regs[rs1] != core.regs[rs2] {
                        core.regs[32] = target_addr;
                        return;
                    }
                },
                funct3::BLT => {
                    if core.regs[rs1] < core.regs[rs2] {
                        core.regs[32] = target_addr;
                        return;
                    }
                },
                funct3::BLTU => {
                    if (core.regs[rs1] as u32) < (core.regs[rs2] as u32) {
                        core.regs[32] = target_addr;
                        return;
                    }
                },
                funct3::BGE => {
                    if core.regs[rs1] > core.regs[rs2] {
                        core.regs[32] = target_addr;
                        return;
                    }
                },
                funct3::BGEU => {
                    if (core.regs[rs1] as u32) > (core.regs[rs2] as u32) {
                        core.regs[32] = target_addr;
                        return;
                    }
                },
                _ => {
                    println!("Unknown Funct3 in Opcode Branch: {}", funct3);
                }
            }
        },
        opcodes::LOAD => {
            let IType { imm, rs1, rd, funct3 } = get_i_type(ins);
            let target_addr = ((sign_extend(imm,12)+core.regs[rs1]) as usize)%MEMSIZE;
            match funct3 {
                funct3::LB => {
                    core.regs[rd] = (core.memory[target_addr] as i8) as i32;
                },
                funct3::LBU => {
                    core.regs[rd] = core.memory[target_addr] as i32;
                },
                funct3::LH => {
                    let b1 = core.memory[target_addr] as u16;
                    let b2 = core.memory[target_addr+1] as u16;
                    core.regs[rd] = (((b2<<8) | b1) as i16) as i32;
                },
                funct3::LHU => {
                    let b1 = core.memory[target_addr] as u16;
                    let b2 = core.memory[target_addr+1] as u16;
                    core.regs[rd] = ((b2<<8) | b1) as i32;
                },
                funct3::LW => {
                    let b1 = core.memory[target_addr] as u32;
                    let b2 = core.memory[target_addr+1] as u32;
                    let b3 = core.memory[target_addr+2] as u32;
                    let b4 = core.memory[target_addr+3] as u32;
                    core.regs[rd] = ((b4<<24) | (b3<<16) | (b2<<8) | b1) as i32;
                },
                _ => {
                    println!("Unknown funct3 in Opcode Load: {}", funct3);
                }
            }
        },
        opcodes::STORE => {
            let SType { imm, rs2, rs1, funct3 } = get_s_type(ins);
            let target_addr = ((sign_extend(imm,12)+core.regs[rs1]) as usize)%MEMSIZE;
            match funct3 {
                funct3::SB => {
                    core.memory[target_addr] = core.regs[rs2] as u8;
                },
                funct3::SH => {
                    core.memory[target_addr] = core.regs[rs2] as u8;
                    core.memory[target_addr+1] = (core.regs[rs2]>>8) as u8;
                },
                funct3::SW => {
                    core.memory[target_addr] = core.regs[rs2] as u8;
                    core.memory[target_addr+1] = (core.regs[rs2]>>8) as u8;
                    core.memory[target_addr+2] = (core.regs[rs2]>>16) as u8;
                    core.memory[target_addr+3] = (core.regs[rs2]>>24) as u8;
                },
                _ => {
                    println!("Unknown funct3 in Opcode Store: {}", funct3);
                }
            }
        },
        opcodes::MISCMEM => {
            let funct3 = take_range(14,12,ins);
            match funct3 {
                funct3::FENCE => {
                    /*
                    * We do single core no cache so nothing to see here.
                    */
                },
                _ => {
                    println!("Unknown funct3 in Opcode MiscMem: {} ", funct3);
                }
            }
        },
        _ => {
            println!("Unknown opcode: {}", opcode);
        }
    }
    core.regs[32] = core.regs[32]+4;
}

fn load_test_program(core: &mut Core) {
    // Instructions
    core.memory[1] = 5;
    store_mem_32(core, 0, addi(14,1,8));
    store_mem_32(core, 4, add(15,14,0));
}

fn main() {
    let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };

    load_test_program(&mut core);
    run(&mut core);

    dump_regs(&core);
    dump_mem(&core, 0x0);
}
