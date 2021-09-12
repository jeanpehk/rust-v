#![ allow( dead_code ) ]

/*
 * Types for core state
 */

struct Core {
    //
}

/*
 * Types for instuction
 */

enum Opcode {
    OppImm = 0b0010011
}

struct InsAddi {
    rd: u32,
    rs1: u32,
    imm: u32
}

enum Ins {
    Addi(InsAddi)
}

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
 * dump 10 bytes starting from index
 */
fn dump_mem(mem: [u8;MEMSIZE], addr: usize) {
    let range = 10;
    let mut i = 0;
    while i < range {
        println!("{:#010x}: {:#02x}", addr+i, mem[addr+i]);
        i += 1;
    }
}

fn dump_regs(regs: [i32;33]) {
    println!("\n{:6} {:<10} {}", "Name", "Dec", "Hex");
    println!(  "{:6} {:<10} {}", "----", "---", "---");
    for i in 0..=32 {
        println!("{:6} {:<10} {:#010x}", REG_NAMES[i], regs[i], regs[i]);
    }
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

fn parse_ins(ins: u32) -> Ins {
    let opcode = take_range(6, 0, ins);
    println!("opcode: {}", opcode);

    if opcode == Opcode::OppImm as u32 {
        let immb31 = take_range(31,31,ins);
        println!("immb31: {}", immb31);
        return Ins::Addi(InsAddi {
            rd: take_range(11,7,ins),
            rs1: take_range(19,15,ins),
            imm: take_range(31,20,ins)
        });
    }

    else {
        println!("Unknown opcode: {}", opcode);
    }

    let dummy = InsAddi { rd: 0, rs1: 0, imm: 0 };
    return Ins::Addi(dummy);
}

fn eval(ins: Ins, mut regs: [i32;33]) -> [i32;33] {
    match ins {
        Ins::Addi(InsAddi{rd, rs1, imm}) => {
            println!("rd: {}", REG_NAMES[rd as usize]);
            println!("rs: {}", REG_NAMES[rs1 as usize]);
            println!("imm: {}", sign_extend(imm, 12));
            regs[rd as usize] = regs[rs1 as usize] + sign_extend(imm,12);
        }
    }
    return regs;
}

fn main() {
    let mut memory: [u8;MEMSIZE] = [0;MEMSIZE];
    let mut regs: [i32;33] = [0;33];
    memory[0] = 1;
    let test = 0xfff10113; // addi sp sp -1
    let ins: Ins = parse_ins(test);
    regs = eval(ins, regs);
    dump_regs(regs);
}

