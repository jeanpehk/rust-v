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
    Store = 0b0100011,
    MiscMem = 0b0001111
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
    // SRL or SRA
    const SRX: Funct3 = Funct3::Five;
    const _SUB: Funct3 = Funct3::Zero;

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

    const FENCE: Funct3 = Funct3::Zero;
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
        let IType { imm, rs1, rd, funct3 } = get_i_type(ins);
        let signed_imm = sign_extend(imm, 12);

        if funct3 == Funct3::ADDI as u32 {
            core.regs[rd] = core.regs[rs1] + signed_imm;
        }
        else if funct3 == Funct3::SLTI as u32 {
            core.regs[rd] = if core.regs[rs1] < signed_imm {1} else {0};
        }
        else if funct3 == Funct3::SLTIU as u32 {
            core.regs[rd] =
                if (core.regs[rs1] as u32) < signed_imm as u32 {1} else {0};
        }
        else if funct3 == Funct3::ANDI as u32 {
            core.regs[rd] = core.regs[rs1] & signed_imm;
        }
        else if funct3 == Funct3::ORI as u32 {
            core.regs[rd] = core.regs[rs1] | signed_imm;
        }
        else if funct3 == Funct3::XORI as u32 {
            core.regs[rd] = core.regs[rs1] ^ signed_imm;
        }
        else if funct3 == Funct3::SLLI as u32 {
            let shamt = imm & 0b11111;
            core.regs[rd] = core.regs[rs1] << shamt;
        }
        else if funct3 == Funct3::SRXI as u32 {
            let arithmetic = take_range(30,30, ins);
            let shamt = imm & 0b11111;
            // SRAI
            if arithmetic == 1 {
                core.regs[rd] = core.regs[rs1] >> shamt;
            }
            // SRLI
            else {
                let ans = core.regs[rs1] as u32 >> shamt;
                core.regs[rd] = ans as i32;
            }
        }
        else {
            println!("Unknown funct3 in op_imm: {}", funct3);
        }
    }
    else if opcode == Opcode::Op as u32 {
        let RType { funct7, rs2, rs1, funct3, rd } = get_r_type(ins);

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
            core.regs[rd] = if core.regs[rs1] < core.regs[rs2] {1} else {0};
        }
        else if funct3 == Funct3::SLTU as u32 {
            let (lhs, rhs) = (core.regs[rs1] as u32, core.regs[rs2] as u32);
            core.regs[rd] = if lhs < rhs {1} else {0};
        }
        else if funct3 == Funct3::XOR as u32 {
            core.regs[rd] = core.regs[rs1] ^ core.regs[rs2];
        }
        else if funct3 == Funct3::SLL as u32 {
            let shamt = core.regs[rs2] & 0b11111;
            core.regs[rd] = core.regs[rs1] << shamt;
        }
        else if funct3 == Funct3::SRX as u32 {
            let shamt = core.regs[rs2] & 0b11111;
            // SRL
            core.regs[rd] = if funct7 == 0 {
                (core.regs[rs1] as u32 >> shamt) as i32
            }
            // SRA
            else {
                core.regs[rs1] >> shamt
            };
        }
        else if funct3 == Funct3::OR as u32 {
            core.regs[rd] = core.regs[rs1] | core.regs[rs2];
        }
        else if funct3 == Funct3::AND as u32 {
            core.regs[rd] = core.regs[rs1] & core.regs[rs2];
        }
        else {
            println!("Unknown Funct3 in Opcode Op: {}", funct3);
        }
    }
    else if opcode == Opcode::Lui as u32 {
        let UType { rd, imm } = get_u_type(ins);
        core.regs[rd] = (imm << 12) as i32;
    }
    else if opcode == Opcode::Auipc as u32 {
        let UType { rd, imm } = get_u_type(ins);
        core.regs[rd] = core.regs[32] + ((imm<<12) as i32);
    }
    else if opcode == Opcode::Jal as u32 {
        let JType { imm, rd } = get_j_type(ins);
        let signed = sign_extend(imm, 21);
        core.regs[rd] = core.regs[32]+4;
        core.regs[32] = core.regs[32]+signed;
    }
    else if opcode == Opcode::Jalr as u32 {
        let IType { imm, rs1, rd, funct3: _ } = get_i_type(ins);

        let imm = sign_extend(imm, 12);
        let val = imm+core.regs[rs1];

        core.regs[rd] = core.regs[32]+4;
        core.regs[32] = if val%2 == 0 {val} else {val-1};
    }
    else if opcode == Opcode::Branch as u32 {
        let BType { imm, rs2, rs1, funct3 } = get_b_type(ins);

        let imm = sign_extend(imm,13);
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
        else {
            println!("Unknown Funct3 in Opcode Branch: {}", funct3);
        }
    }
    else if opcode == Opcode::Load as u32 {
        let IType { imm, rs1, rd, funct3: width } = get_i_type(ins);

        let target_addr = ((sign_extend(imm,12)+core.regs[rs1]) as usize)%MEMSIZE;
        if width == Funct3::LB as u32 {
            core.regs[rd] = (core.memory[target_addr] as i8) as i32;
        }
        if width == Funct3::LBU as u32 {
            core.regs[rd] = core.memory[target_addr] as i32;
        }
        else if width == Funct3::LH as u32 {
            let b1 = core.memory[target_addr] as u16;
            let b2 = core.memory[target_addr+1] as u16;
            core.regs[rd] = (((b2<<8) | b1) as i16) as i32;
        }
        else if width == Funct3::LHU as u32 {
            let b1 = core.memory[target_addr] as u16;
            let b2 = core.memory[target_addr+1] as u16;
            core.regs[rd] = ((b2<<8) | b1) as i32;
        }
        else if width == Funct3::LW as u32 {
            let b1 = core.memory[target_addr] as u32;
            let b2 = core.memory[target_addr+1] as u32;
            let b3 = core.memory[target_addr+2] as u32;
            let b4 = core.memory[target_addr+3] as u32;
            core.regs[rd] = ((b4<<24) | (b3<<16) | (b2<<8) | b1) as i32;
        }
        else {
            println!("Unknown width in Opcode Load: {}", width);
        }
    }
    else if opcode == Opcode::Store as u32 {
        let SType { imm, rs2, rs1, funct3: width } = get_s_type(ins);
        let target_addr = ((sign_extend(imm,12)+core.regs[rs1]) as usize)%MEMSIZE;
        if width == Funct3::SB as u32 {
            core.memory[target_addr] = core.regs[rs2] as u8;
        }
        else if width == Funct3::SH as u32 {
            core.memory[target_addr] = core.regs[rs2] as u8;
            core.memory[target_addr+1] = (core.regs[rs2]>>8) as u8;
        }
        else if width == Funct3::SW as u32 {
            core.memory[target_addr] = core.regs[rs2] as u8;
            core.memory[target_addr+1] = (core.regs[rs2]>>8) as u8;
            core.memory[target_addr+2] = (core.regs[rs2]>>16) as u8;
            core.memory[target_addr+3] = (core.regs[rs2]>>24) as u8;
        }
        else {
            println!("Unknown width in Opcode Store: {}", width);
        }
    }
    else if opcode == Opcode::MiscMem as u32 {
        let funct3 = take_range(14,12,ins);
        if funct3 == Funct3::FENCE as u32 {
            /*
             * We do single core no cache so nothing to see here.
             */
        }
        else {
            println!("Unknown funct3 in Opcode MiscMem: {} ", funct3);
        }

    }
    else {
        println!("Unknown opcode: {}", opcode);
    }
    return core;
}

fn main() {
    let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
    // 0000a703 lw a4,0(ra)
    let test = 0x0000a703;
    core.regs[1] = 4;
    core.regs[2] = 1;
    core.memory[4] = 0x1;
    core.memory[5] = 0x2;
    core = eval(test, core);
    core = dump_regs(core);
    dump_mem(core, 0);
}
