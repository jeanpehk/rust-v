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
    let mut sht_symtab = 0;
    let mut sht_size = 0;
    let mut sht_entsize = 0;
    let mut sht_strtab = 0;
    let mut sht_strsize = 0;
    for _ in 0..e_shnum {
      //  println!("----------------------------------");
        let sh_name = take4(&elf, index); // read(&elf, index);
        let sh_type = take4(&elf, index+0x04);
        let sh_addr = take4(&elf, index+0x0c);
        let sh_offset = take4(&elf, index+0x10) as usize;
        let sh_size = take4(&elf, index+0x14) as usize;
        let sh_info = take4(&elf, index+0x1c);
        let sh_entsize = take4(&elf, index+0x24) as usize;

        /*
        println!("- sh_name: {:#x}", sh_name);
        println!("- sh_type: {:#x}", sh_type);
        println!("- sh_addr: {:#x}", sh_addr);
        println!("- sh_offset: {:#x}", sh_offset);
        println!("- sh_size: {:#x}", sh_size);
        println!("- sh_info: {:#x}", sh_info);
        println!("- sh_entsize: {:#x}", sh_entsize);
        */

        if sh_type == 0x2 { // symtab
            sht_symtab = sh_offset;
            sht_size = sh_size;
            sht_entsize = sh_entsize;
        }
        else if sh_type == 0x3 { // string table
            if sht_strtab == 0 { // HACK HACK HACK FIX FIX FIX
                sht_strtab = sh_offset;
                sht_strsize = sh_size;
            }
        }

        index += e_shentsize;
    }

    let mut ix = sht_symtab;
    let mut fail_addr = 0;
    let mut pass_addr = 0;
    while ix < ix+sht_size {
        let st_name = take4(&elf, ix) as usize;
        let st_value = take4(&elf, ix+0x04);
        let st_size = take4(&elf, ix+0x08);
        let st_info = elf[ix+0x0c];
        let st_other = elf[ix+0xd];
        let st_shndx = take2(&elf, ix+0xe);
        if st_name != 0 {
            let ch = elf[sht_strtab+st_name];
            let ch2 = elf[sht_strtab+st_name+1];

            if ch as char == 'f' && ch2 as char == 'a' {
                fail_addr = st_value;
                if pass_addr != 0 { break };
            }
            else if ch as char == 'p' && ch2 as char == 'a' {
                pass_addr = st_value;
                if fail_addr != 0 { break };
            }
        }

        ix += sht_entsize;
    }
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
