use crate::Core;
use crate::constants::START_ADDR;

pub fn load_elf(core: &mut Core, elf: &Vec<u8>) {
    /*
     * ELF Header
     */
    if (elf[0x0],elf[0x1],elf[0x2],elf[0x3]) != (0x7f,0x45,0x4c,0x46) {
        panic!("File not ELF");
    }
    if elf[0x4] != 1 {
        panic!("Can only run 32bit programs.");
    }
    if elf[0x12] != 0xf3 {
        panic!("ELF not RISC-V architecture.");
    }

    let e_phentsize = take2(&elf, 0x2a) as usize; // size of entry
    let e_phnum = take2(&elf, 0x2c); // number of entries
    let e_phoff = take4(&elf, 0x1c) as usize; // program header offset

    /*
     * Program Header
     */
    let mut index = e_phoff;
    for _ in 0..e_phnum {
        let p_offset = take4(&elf, index+0x04) as usize;
        let p_vaddr = take4(&elf, index+0x08);
        let p_memsz = take4(&elf, index+0x14) as usize;

        for i in 0..p_memsz {
            // we imagine 0x0 = 0x80000000
            core.memory[(p_vaddr as u32-START_ADDR) as usize+i] = elf[p_offset+i];
        }
        index += e_phentsize;
    }
}

// very hacky very bad k
pub fn get_riscv_tests_addrs(elf: &Vec<u8>) -> (u32, u32) {
    let e_shstrndx = take2(&elf, 0x32); // size of entry
    let e_shoff = take4(&elf, 0x20) as usize; // start of section header
    let e_shentsize = take2(&elf, 0x2e) as usize; // size of entry
    let e_shnum = take2(&elf, 0x30); // size of entry

    // table for section header names
    let shstrtab_addr = e_shoff as i32 + e_shentsize as i32 * e_shstrndx as i32 + 0x10;
    let shstrtab = take4(&elf, shstrtab_addr as usize) as usize;

    let mut index = e_shoff;
    let mut sht_symtab = 0;
    let mut sht_size = 0;
    let mut sht_entsize = 0;
    let mut strtab = 0;
    for _ in 0..e_shnum {
        let sh_name = take4(&elf, index) as usize;
        let sh_size = take4(&elf, index+0x14) as usize;
        let sh_type = take4(&elf, index+0x04);
        let sh_offset = take4(&elf, index+0x10) as usize;
        let sh_entsize = take4(&elf, index+0x24) as usize;

        if sh_type == 0x2 { // symtab
            sht_symtab = sh_offset;
            sht_size = sh_size;
            sht_entsize = sh_entsize;
        }
        else if sh_type == 0x3 && read(&elf, shstrtab+sh_name) == ".strtab" {
            strtab = sh_offset;
        }

        index += e_shentsize;
    }

    let mut ix = sht_symtab;
    let mut fail_addr = 0;
    let mut pass_addr = 0;
    while ix < ix+sht_size {
        let st_name = take4(&elf, ix) as usize;
        let st_value = take4(&elf, ix+0x04);
        if st_name != 0 {
            let name = read(&elf, strtab+st_name);
            if name == "fail" {
                fail_addr = st_value;
                if pass_addr != 0 { break };
            }
            else if name == "pass" {
                pass_addr = st_value;
                if fail_addr != 0 { break };
            }
        }
        ix += sht_entsize;
    }
    return (pass_addr-START_ADDR, fail_addr-START_ADDR);
}

fn take4(elf: &Vec<u8>, i: usize) -> u32 {
    return ((elf[i+3] as u32) << 24)
        | ((elf[i+2] as u32) << 16)
        | ((elf[i+1] as u32) << 8)
        | (elf[i] as u32);
}

fn take2(elf: &Vec<u8>, i: usize) -> u32 {
    return ((elf[i+1] as u32) << 8)
            | (elf[i] as u32);
}

fn read(elf: &Vec<u8>, i: usize) -> String {
    let mut name = String::from("");
    let mut index = i;
    while elf[index] != 0 {
        name.push(elf[index] as char);
        index += 1;
    }
    return name;
}
