#![ allow( dead_code ) ]

mod test;

const MEMSIZE: usize = 1024;
const REG_NAMES: [&str; 33] = [
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

/*
 * Main structure for core state
 */
struct Core {
    memory: [u8; MEMSIZE],
    regs: [i32;33]
}

enum Opcode {
    OppImm = 0b0010011
}

enum Funct3 {
    Addi = 0b000,
    Slti = 0b010,
    Sltiu = 0b011
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
        let signed_imm = sign_extend(i_imm,12);

        if funct3 == Funct3::Addi as u32 {
            core.regs[rd as usize] = core.regs[rs1 as usize] + signed_imm;
        }
        else if funct3 == Funct3::Slti as u32 {
            core.regs[rd as usize] = if core.regs[rs1 as usize] < signed_imm {
                1
            } else {
                0
            };
        }
        else if funct3 == Funct3::Sltiu as u32 {
            core.regs[rd as usize] =
                if (core.regs[rs1 as usize] as u32) < signed_imm as u32 {
                    1
                } else {
                    0
                };
        }
        else {
            println!("Unknown funct3 in op_imm: {}", funct3);
        }
    }
    else {
        println!("Unknown opcode: {}", opcode);
    }
    return core;
}


fn main() {
    let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
//    let test = 0x0000b713; // sltiu a4 ra 0
    let test = 0x8000b713; // sltiu a4 ra -2048
    core.regs[1] = 0;
    core = eval(test, core);
    dump_regs(core);
}
