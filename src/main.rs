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
    Branch = 0b1100011,
    Load = 0b0000011,
    Store = 0b0100011
}

enum Funct3 {
    Zero = 0b000,
    One = 0b001,
    Two = 0b010,
    Three = 0b011,
    Four = 0b100,
    Five = 0b101,
    Six = 0b110,
    Seven = 0b111,
}

impl Funct3 {
    // Integer Register Immediate
    const ADDI: Funct3 = Funct3::Zero;
    const SLTI: Funct3 = Funct3::Two;
    const SLTIU: Funct3 = Funct3::Three;
    const ANDI: Funct3 = Funct3::Seven;
    const ORI: Funct3 = Funct3::Six;
    const XORI: Funct3 = Funct3::Four;
    const SLLI: Funct3 = Funct3::One;
    const SRXI: Funct3 = Funct3::Five;

    // Integer Register Register
    const ADD: Funct3 = Funct3::Zero;
    const SLT: Funct3 = Funct3::Two;
    const SLTU: Funct3 = Funct3::Three;
    const AND: Funct3 = Funct3::Seven;
    const OR: Funct3 = Funct3::Six;
    const XOR: Funct3 = Funct3::Four;
    const SLL: Funct3 = Funct3::One;
    const SRL: Funct3 = Funct3::Five;
    const _SUB: Funct3 = Funct3::Zero;
    const SRA: Funct3 = Funct3::Five;

    // Branch
    const BEQ: Funct3 = Funct3::Zero;
    const BNE: Funct3 = Funct3::One;
    const BLT: Funct3 = Funct3::Four;
    const BLTU: Funct3 = Funct3::Six;
    const BGE: Funct3 = Funct3::Five;
    const BGEU: Funct3 = Funct3::Seven;

    // Load/Store
    const LB: Funct3 = Funct3::Zero;
    const LH: Funct3 = Funct3::One;
    const LW: Funct3 = Funct3::Two;
    const LBU: Funct3 = Funct3::Four;
    const LHU: Funct3 = Funct3::Five;
    const SB: Funct3 = Funct3::Zero;
    const SH: Funct3 = Funct3::One;
    const SW: Funct3 = Funct3::Two;
}

/*
 * dump 10 bytes starting from index
 */
fn dump_mem(core: Core, addr: usize) -> Core {
    let range = 10;
    let mut i = 0;
    println!("{:11} {:5} {:3}", "Memory", "Dec",  "Hex");
    println!("{:11} {:5} {:3}", "------", "---", "---");
    while i < range {
        println!("{:#010x}: {:<#5} {:<#02x}", addr+i, core.memory[addr+i], core.memory[addr+i]);
        i += 1;
    }
    return core;
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

        if funct3 == Funct3::ADDI as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] + signed_i_imm;
        }
        else if funct3 == Funct3::SLTI as u32 {
            core.regs[rd as usize] = if core.regs[rs1 as usize] < signed_i_imm {
                1
            } else {
                0
            };
        }
        else if funct3 == Funct3::SLTIU as u32 {
            core.regs[rd as usize] =
                if (core.regs[rs1 as usize] as u32) < signed_i_imm as u32 {
                    1
                } else {
                    0
                };
        }
        else if funct3 == Funct3::ANDI as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] & signed_i_imm;
        }
        else if funct3 == Funct3::ORI as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] | signed_i_imm;
        }
        else if funct3 == Funct3::XORI as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] ^ signed_i_imm;
        }
        else if funct3 == Funct3::SLLI as u32 {
            let shamt = i_imm & 0b11111;
            core.regs[rd as usize] = core.regs[rs1 as usize] << shamt;
        }
        else if funct3 == Funct3::SRXI as u32 {
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
        let target_addr = (((core.regs[32] + imm) as usize)%MEMSIZE) as i32;

        if funct3 == Funct3::BEQ as u32 {
            if core.regs[rs1] == core.regs[rs2] {
                core.regs[32] = target_addr;
            }
        }
        else if funct3 == Funct3::BNE as u32 {
            if core.regs[rs1] != core.regs[rs2] {
                core.regs[32] = target_addr;
            }
        }
        else if funct3 == Funct3::BLT as u32 {
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
    else if opcode == Opcode::Load as u32 {
        let imm = take_range(31,20,ins);
        let rs1 = take_range(19,15,ins) as usize;
        let width = take_range(14,12,ins);
        let rd = take_range(11,7,ins) as usize;
        let target_addr = ((sign_extend(imm,12)+core.regs[rs1]) as usize)%MEMSIZE;
        println!("imm: {}", imm);
        println!("rs1: {}", rs1);
        println!("rd: {}", rd);
        println!("width: {}", width);
        println!("target_addr: {}", target_addr);
        if width == Funct3::LB as u32 {
            println!("lb");
            core.regs[rd] = core.memory[target_addr] as i32;
        }
        else if width == Funct3::LH as u32 {
            println!("lh");
            let b1 = core.memory[target_addr] as u16;
            let b2 = core.memory[target_addr+1] as u16;
            core.regs[rd] = ((b2<<8) | b1) as i32;
        }
        else if width == Funct3::LW as u32 {
            println!("lw");
            let b1 = core.memory[target_addr] as u32;
            let b2 = core.memory[target_addr+1] as u32;
            let b3 = core.memory[target_addr+2] as u32;
            let b4 = core.memory[target_addr+3] as u32;
            core.regs[rd] = ((b4<<24) | (b3<<16) | (b2<<8) | b1) as i32;
        }
    }
    else if opcode == Opcode::Store as u32 {
        let imm11_5 = take_range(31,25,ins);
        let rs2 = take_range(24,20,ins) as usize;
        let rs1 = take_range(19,15,ins) as usize;
        let width = take_range(14,12,ins);
        let imm4_0 = take_range(11,7,ins);
        let imm = (imm11_5<<5) | imm4_0;
        let target_addr = ((sign_extend(imm,12)+core.regs[rs1]) as usize)%MEMSIZE;
        if width == Funct3::SB as u32 {
            println!("sb");
            core.memory[target_addr] = core.regs[rs2] as u8;
        }
        else if width == Funct3::SH as u32 {
            println!("sh");
            core.memory[target_addr] = core.regs[rs2] as u8;
            core.memory[target_addr+1] = (core.regs[rs2]>>8) as u8;
        }
        else if width == Funct3::SW as u32 {
            println!("sw");
            core.memory[target_addr] = core.regs[rs2] as u8;
            core.memory[target_addr+1] = (core.regs[rs2]>>8) as u8;
            core.memory[target_addr+2] = (core.regs[rs2]>>16) as u8;
            core.memory[target_addr+3] = (core.regs[rs2]>>24) as u8;
        }
    }
    else {
        println!("Unknown opcode: {}", opcode);
    }
    return core;
}

fn main() {
    let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
    // 00008703            lb  a4,0(ra)
    // 00209703            lh  a4,2(ra)
    // 0080a703            lw  a4,8(ra) // 0000a703 lw a4,0(ra)
    // 00208023            sb  sp,0(ra)
    // 00209223            sh  sp,4(ra)
    // 0020a423            sw  sp,8(ra)
    let test = 0x0000a703;
    core.regs[1] = 4;
    core.regs[2] = 1;
    core.memory[4] = 0x1;
    core.memory[5] = 0x2;
    core = eval(test, core);
    core = dump_regs(core);
    dump_mem(core, 0);
}
