#![ allow( dead_code ) ]

mod test;
mod constants;

use constants::MEMSIZE;
use constants::REG_NAMES;

/*
 * Main structure for core state
 */
struct Core {
    memory: [u8; MEMSIZE],
    regs: [i32;33]
}

enum Opcode {
    Op = 0b0110011,
    OpImm = 0b0010011,
    Lui = 0b0110111,
    Auipc = 0b0010111,
    Jal = 0b1101111,
    Jalr = 0b1100111,
    Branch = 0b1100011
}

enum Funct3 {
    Addi = 0b000,
    Slti = 0b010,
    Sltiu = 0b011,
    Andi = 0b111,
    Ori = 0b110,
    Xori = 0b100,
    Slli = 0b001,
    Srxi = 0b101, // SRLI or SRAI, set in bit 30
}

impl Funct3 {
    const ADDI: Funct3 = Funct3::Addi;
    const SLTI: Funct3 = Funct3::Slti;
    const SLTIU: Funct3 = Funct3::Sltiu;
    const ANDI: Funct3 = Funct3::Andi;
    const ORI: Funct3 = Funct3::Ori;
    const XORI: Funct3 = Funct3::Xori;
    const SLLI: Funct3 = Funct3::Slli;
    const SRLI: Funct3 = Funct3::Srxi;
    const SRAI: Funct3 = Funct3::Srxi;
    const ADD: Funct3 = Funct3::Addi;
    const SUB: Funct3 = Funct3::Addi;
    const SLT: Funct3 = Funct3::Slti;
    const SLTU: Funct3 = Funct3::Sltiu;
    const XOR: Funct3 = Funct3::Xori;
    const SLL: Funct3 = Funct3::Slli;
    const SRL: Funct3 = Funct3::Srxi;
    const SRA: Funct3 = Funct3::Srxi;
    const OR: Funct3 = Funct3::Ori;
    const AND: Funct3 = Funct3::Andi;

    // Branch instructions
    const BEQ: Funct3 = Funct3::Addi;
    const BNE: Funct3 = Funct3::Slli;
    const BLT: Funct3 = Funct3::Xori;
    const BLTU: Funct3 = Funct3::Ori;
    const BGE: Funct3 = Funct3::Srxi;
    const BGEU: Funct3 = Funct3::Andi;
}


/*
 * dump 10 bytes starting from index
 */
fn _dump_mem(mem: [u8;MEMSIZE], addr: usize) {
    let range = 10;
    let mut i = 0;
    while i < range {
        println!("{:#010x}: {:#02x}", addr+i, mem[addr+i]);
        i += 1;
    }
}

fn dump_regs(core: Core) -> Core {
    let regs = core.regs;
    println!("{:6} {:<10} {}", "Name", "Dec", "Hex");
    println!("{:6} {:<10} {}", "----", "---", "---");
    for i in 0..=32 {
        println!("{:6} {:<10} {:#010x}", REG_NAMES[i], regs[i], regs[i]);
    }
    return core;
}

fn take_range(start: u32, end: u32, ins: u32) -> u32 {
    return (ins >> end) & ((1 << (start-end+1))-1);
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

fn eval(ins: u32, mut core: Core) -> Core {
    let opcode = take_range(6, 0, ins);

    if opcode == Opcode::OpImm as u32 {
        let funct3 = take_range(14,12,ins);
        let rd = take_range(11,7,ins);
        let rs1 = take_range(19,15,ins);
        let i_imm = take_range(31,20,ins);
        let signed_i_imm = sign_extend(i_imm,12);

        if funct3 == Funct3::Addi as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] + signed_i_imm;
        }
        else if funct3 == Funct3::Slti as u32 {
            core.regs[rd as usize] = if core.regs[rs1 as usize] < signed_i_imm {
                1
            } else {
                0
            };
        }
        else if funct3 == Funct3::Sltiu as u32 {
            core.regs[rd as usize] =
                if (core.regs[rs1 as usize] as u32) < signed_i_imm as u32 {
                    1
                } else {
                    0
                };
        }
        else if funct3 == Funct3::Andi as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] & signed_i_imm;
        }
        else if funct3 == Funct3::Ori as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] | signed_i_imm;
        }
        else if funct3 == Funct3::Xori as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] ^ signed_i_imm;
        }
        else if funct3 == Funct3::Slli as u32 {
            let shamt = i_imm & 0b11111;
            core.regs[rd as usize] = core.regs[rs1 as usize] << shamt;
        }
        else if funct3 == Funct3::Srxi as u32 {
            let arithmetic = take_range(30,30, ins);
            let shamt = i_imm & 0b11111;
            if arithmetic == 1 {
                core.regs[rd as usize] = core.regs[rs1 as usize] >> shamt;
            }
            else {
                let ans = core.regs[rs1 as usize] as u32 >> shamt;
                core.regs[rd as usize] = ans as i32;
            }
        }

        else {
            println!("Unknown funct3 in op_imm: {}", funct3);
        }
    }
    else if opcode == Opcode::Op as u32 {
        let funct7 = take_range(31,25,ins);
        let rs2 = take_range(24,20,ins) as usize;
        let rs1 = take_range(19,15,ins) as usize;
        let funct3 = take_range(14,12,ins);
        let rd = take_range(11,7,ins) as usize;

        // add or sub
        if funct3 == Funct3::ADD as u32 {
            // add
            if funct7 == 0 {
                core.regs[rd] = core.regs[rs1] + core.regs[rs2];
            }
            // sub
            else {
                core.regs[rd] = core.regs[rs1] - core.regs[rs2];
            }
        }
        else if funct3 == Funct3::SLT as u32 {
            core.regs[rd] = if core.regs[rs1] < core.regs[rs2] {
                1
            }
            else {
                0
            };
        }
        else if funct3 == Funct3::SLTU as u32 {
            core.regs[rd] = if (core.regs[rs1] as u32) < (core.regs[rs2] as u32) {
                1
            }
            else {
                0
            };
        }
        else if funct3 == Funct3::XOR as u32 {
            core.regs[rd] = core.regs[rs1] ^ core.regs[rs2];
        }
        else if funct3 == Funct3::SLL as u32 {
            let shamt = core.regs[rs2] & 0b11111;
            core.regs[rd] = core.regs[rs1] << shamt;
        }
        // srl or sra
        else if funct3 == Funct3::SRL as u32 {
            let shamt = core.regs[rs2] & 0b11111;
            if funct7 == 0 {
                core.regs[rd] = ((core.regs[rs1] as u32) >> shamt) as i32;
            }
            else {
                core.regs[rd] = core.regs[rs1] >> shamt;
            }
        }
        else if funct3 == Funct3::OR as u32 {
            core.regs[rd] = core.regs[rs1] | core.regs[rs2];
        }
        else if funct3 == Funct3::AND as u32 {
            core.regs[rd] = core.regs[rs1] & core.regs[rs2];
        }
    }
    else if opcode == Opcode::Lui as u32 {
        let rd = take_range(11,7,ins);
        let u_imm = take_range(31,12,ins);
        core.regs[rd as usize] = (u_imm << 12) as i32;
    }
    else if opcode == Opcode::Auipc as u32 {
        let rd = take_range(11,7,ins);
        let u_imm = take_range(31,12,ins);
        let offset = (u_imm << 12) as i32;
        core.regs[rd as usize] = core.regs[32] + offset;
    }
    else if opcode == Opcode::Jal as u32 {
        let imm20 = take_range(31,31,ins);
        let imm10_1 = take_range(30,21,ins);
        let imm11 = take_range(20,20,ins);
        let imm19_12 = take_range(19,12,ins);
        let rd = take_range(11,7,ins) as usize;
        let imm = (imm20<<20) | (imm19_12<<12) | (imm11<<11) | (imm10_1<<1);
        let signed = sign_extend(imm, 21);
        core.regs[rd] = core.regs[32]+4;
        core.regs[32] = core.regs[32]+signed;
    }
    else if opcode == Opcode::Jalr as u32 {
        let imm = sign_extend(take_range(31,20,ins),12);
        let rs1 = take_range(19,15,ins) as usize;
        let _funct3 = take_range(14,12,ins); // when do we need this?
        let rd = take_range(11,7,ins) as usize;
        let val = imm+core.regs[rs1];
        core.regs[rd] = core.regs[32]+4;
        core.regs[32] = if val%2 == 0 {val} else {val-1};
    }
    else if opcode == Opcode::Branch as u32 {
        let imm12 = take_range(31,31,ins);
        let imm10_5 = take_range(30,25,ins);
        let rs2 = take_range(24,20,ins) as usize;
        let rs1 = take_range(19,15,ins) as usize;
        let funct3 = take_range(14,12,ins);
        let imm4_1 = take_range(11,8,ins);
        let imm11 = take_range(7,7,ins);
        let imm = sign_extend((imm12<<12)|(imm11<<11)|(imm10_5<<5)|(imm4_1<<1),13);
        let target_addr = core.regs[32] + imm;
        println!("imm: {}", imm);
        println!("rs1: {}", rs1);
        println!("rs2: {}", rs2);

        if funct3 == Funct3::BEQ as u32 {
            println!("beq");
            if core.regs[rs1] == core.regs[rs2] {
                core.regs[32] = target_addr;
            }
        }
        else if funct3 == Funct3::BNE as u32 {
            println!("bne");
            if core.regs[rs1] != core.regs[rs2] {
                core.regs[32] = target_addr;
            }
        }
        else if funct3 == Funct3::BLT as u32 {
            println!("blt");
            if core.regs[rs1] < core.regs[rs2] {
                core.regs[32] = target_addr;
            }

        }
        else if funct3 == Funct3::BLTU as u32 {
            if (core.regs[rs1] as u32) < (core.regs[rs2] as u32) {
                core.regs[32] = target_addr;
            }

        }
        else if funct3 == Funct3::BGE as u32 {
            if core.regs[rs1] > core.regs[rs2] {
                core.regs[32] = target_addr;
            }

        }
        else if funct3 == Funct3::BGEU as u32 {
            if (core.regs[rs1] as u32) > (core.regs[rs2] as u32) {
                core.regs[32] = target_addr;
            }
        }
    }
    else {
        println!("Unknown opcode: {}", opcode);
    }
    return core;
}

fn main() {
    let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
    // 0020d663            bgeu ra,sp,12
    let test = 0x0020f663;
    core.regs[1] = -1;
    core.regs[2] = 2;
    core = eval(test, core);
    dump_regs(core);
}
