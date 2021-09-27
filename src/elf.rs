pub fn parse_elf(elf: Vec<u8>) -> Vec<u32> {
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

    // Check that ELF
    if (elf[0x0],elf[0x1],elf[0x2],elf[0x3]) == (0x7f,0x45,0x4c,0x46) {
        println!("elf!");
    }
    else {
        println!("noelf :(");
    }

    let is_32 = if elf[0x4] == 1 { true } else { false };
    println!("is_32: {}", is_32);

    // Check that RISC-V
    if elf[0x12] == 0xf3 {
        println!("riskk");
    }
    else {
        println!("noriskk :(");
    }

    let e_phoff = elf[0x1c]; // start of program header
    println!("e_phoff: {:#x}", e_phoff);

    let e_shoff = elf[0x20]; // start of section header
    println!("e_shoff: {:#x}", e_shoff);

    let program: Vec<u32> = (0..10).collect();
    return program;
}
