use crate::Core;

pub fn parse_elf(core: &mut Core, elf: Vec<u8>) {
    /*
     * Can we read an elf and produce something
     * that the simulator can run??
     *
     * Loading Process:
     * 1. Header
     *   - elf header is parsed
     *   - program header is parsed
     * 2. Mapping
     *   - file is mapped in memory
     * 3. Execution
     *   - so here just return
     *
     * - can we run riscv-tests??
     */

    println!("~~~~~~VERY HIGH CLASS ELF PARSER~~~~~~");

    // Check that ELF
    if (elf[0x0],elf[0x1],elf[0x2],elf[0x3]) == (0x7f,0x45,0x4c,0x46) {
        println!("iself");
    }
    else {
        println!("notelf");
    }

    let is_32 = if elf[0x4] == 1 { true } else { false };
    println!("32bit: {}", is_32);

    // Check that RISC-V
    if elf[0x12] == 0xf3 {
        println!("RISC-V");
    }
    else {
        println!("not RISC-V");
    }

    let e_phoff = elf[0x1c] as usize; // program header offset
    println!("e_phoff: {:#x}", e_phoff);

    let e_phnum = elf[0x2c] as usize; // number of entries in program header table
    println!("e_phnum: {:#x}", e_phnum);

    let e_phentsize = elf[0x2a] as usize; // size of a program header table entry
    println!("e_phentsize: {:#x}", e_phentsize);

    // load segments
    let mut index = e_phoff;
    for i in 0..e_phnum {
        println!("----------------------------------");
        println!("Segment {} at {:#x}", i, index);

        let type_of_segment = toint_4(&elf, index);
        println!("- type: {:#x}", type_of_segment);

        let p_offset = toint_4(&elf, index+0x04) as usize;
        println!("- p_offset: {:#x}", p_offset);

        let p_vaddr = toint_4(&elf, index+0x08);
        println!("- p_vaddr: {:#x}", p_vaddr);

        let p_paddr = toint_4(&elf, index+0x0c);
        println!("- p_paddr: {:#x}", p_paddr);

        let p_filesz = toint_4(&elf, index+0x10);
        println!("- p_filesz: {:#x}", p_filesz);

        let p_memsz = toint_4(&elf, index+0x14) as usize;
        println!("- p_memsz: {:#x}", p_memsz);

        let p_flags = toint_4(&elf, index+0x18);
        println!("- p_flags: {:#x}", p_flags);

        let p_align = toint_4(&elf, index+0x1c);
        println!("- p_align: {:#x}", p_align);

        let end = elf[index+0x20];
        println!("- end: {:#x}", end);

        let ins = toint_4(&elf, p_offset);
        println!("is this ins: {:#x}", ins);

        //let load = &elf[p_offset..p_offset+p_memsz];
        for i in 0..p_memsz {
            // we imagine core mem 0 = 0x80000000
            core.memory[(p_vaddr as u32-0x80000000) as usize+i] = elf[p_offset+i];
        }

        index += e_phentsize;
    }
    println!("----------------------------------");
}

fn toint_4(elf: &Vec<u8>, i: usize) -> i32 {
    return (((elf[i+3] as u32) << 24)
        | ((elf[i+2] as u32) << 16)
        | ((elf[i+1] as u32) << 8)
        | (elf[i] as u32)) as i32;

}
