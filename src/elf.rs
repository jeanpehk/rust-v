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

    let e_phoff = take4(&elf, 0x1c) as usize; // program header offset
    let e_phnum = take2(&elf, 0x2c); // number of entries

    let e_phentsize = take2(&elf, 0x2a) as usize; // size of entry

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

    let e_shoff = take4(&elf, 0x20) as usize; // start of section header
    let e_shentsize = take2(&elf, 0x2e) as usize; // size of entry
    let e_shnum = take2(&elf, 0x30); // size of entry
    let e_shstrndx = take2(&elf, 0x32);
    println!("e_shoff: {:#x}", e_shoff);
    println!("e_shentsize: {:#x}", e_shentsize);
    println!("e_shnum: {:#x}", e_shnum);
    println!("e_shstrndx: {:#x}", e_shstrndx);

    let mut index = e_shoff;
    for _ in 0..e_shnum {
        println!("----------------------------------");
        let sh_name = take4(&elf, index); // read(&elf, index);
        let sh_type = take4(&elf, index+0x04);
        let sh_addr = take4(&elf, index+0x0c);
        println!("- sh_name: {:#x}", sh_name);
        println!("- sh_type: {:#x}", sh_type);
        println!("- sh_addr: {:#x}", sh_addr);

        if sh_type == 0x2 { // symtab (todo)
        }

        index += e_shentsize;
    }
    println!("----------------------------------");
}

fn take4(elf: &Vec<u8>, i: usize) -> i32 {
    return (((elf[i+3] as u32) << 24)
        | ((elf[i+2] as u32) << 16)
        | ((elf[i+1] as u32) << 8)
        | (elf[i] as u32)) as i32;
}

fn take2(elf: &Vec<u8>, i: usize) -> i32 {
    return (((elf[i+1] as u32) << 8)
            | (elf[i] as u32)) as i32;
}

/*
fn read(elf: &Vec<u8>, i: usize) -> String {
    let mut name = String::from("");
    let mut index = i;
    while elf[index] != 0 {
        name.push(elf[index] as char);
        index += 1;
    }
    return name;
}
*/
