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
    OppImm = 0b0010011,
    Lui = 0b0110111,
    Auipc = 0b0010111
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
    if opcode == Opcode::OppImm as u32 {
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
    else {
        println!("Unknown opcode: {}", opcode);
    }
    return core;
}


fn main() {
    let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
    let test = 0x00002517; // auipc a0 2
    core.regs[1] = 0;
    core = eval(test, core);
    dump_regs(core);
}
