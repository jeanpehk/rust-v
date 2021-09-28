use crate::Core;

pub fn parse_elf(core: &mut Core, elf: Vec<u8>) {
    if (elf[0x0],elf[0x1],elf[0x2],elf[0x3]) != (0x7f,0x45,0x4c,0x46) {
        panic!("File not ELF");
    }
    if elf[0x4] != 1 {
        panic!("Can only run 32bit programs.");
    }
    if elf[0x12] != 0xf3 {
        panic!("ELF not RISC-V architecture.");
    }

    let e_phoff = elf[0x1c] as usize; // program header offset
    let e_phnum = elf[0x2c] as usize; // number of entries in program header table
    let e_phentsize = elf[0x2a] as usize; // size of a program header table entry

    // load segments
    let mut index = e_phoff;
    for _ in 0..e_phnum {
        let p_offset = take4(&elf, index+0x04) as usize;
        let p_vaddr = take4(&elf, index+0x08);
        let p_memsz = take4(&elf, index+0x14) as usize;

        for i in 0..p_memsz {
            // we imagine 0x0 = 0x80000000
            core.memory[(p_vaddr as u32-0x80000000) as usize+i] = elf[p_offset+i];
        }

        index += e_phentsize;
    }
}

fn take4(elf: &Vec<u8>, i: usize) -> i32 {
    return (((elf[i+3] as u32) << 24)
        | ((elf[i+2] as u32) << 16)
        | ((elf[i+1] as u32) << 8)
        | (elf[i] as u32)) as i32;
}
