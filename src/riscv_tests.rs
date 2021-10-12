// https://github.com/riscv-software-src/riscv-tests
// - rv32ui-p-*
// - assumes the tests are compiled into elfs at ./riscv-tests/isa/
#[cfg(test)]
mod riscv_tests {
    use std::fs;

    use colored::*;

    use crate::Core;
    use crate::init;
    use crate::eval;
    use crate::elf::*;

    const FOLDER: &str = "./riscv-tests/isa/";
    const PREFIX: &str = "rv32ui-p-";

    #[test]
    fn run_riscv_tests() {
        let mut i = 0;
        let mut success = 0;
        println!("ATTEMPTING TO RUN TEST SET `RISCV-TESTS`\n");
        let paths = fs::read_dir(FOLDER).unwrap();
        for item in paths {
            if let Ok(item) = item {
                if let Ok(st) = item.file_name().into_string() {
                    if st.starts_with(PREFIX) {
                        if let None = item.path().extension() {
                            let elf: Vec<u8> = fs::read(item.path())
                                .expect("Couldn't read file");
                            let mut core = init();
                            load_elf(&mut core, &elf);
                            if !(st == "rv32ui-p-simple") { // this has no fail/pass addrs
                                let (pass_addr, fail_addr) = get_riscv_tests_addrs(&elf);
                                println!("Running set {}: {}", i+1, st);
                                let res = execute_riscv_test(&mut core, pass_addr, fail_addr);
                                assert_eq!(res, 1);
                                success += execute_riscv_test(&mut core, pass_addr, fail_addr);
                                i += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("\nRan a total of {} tests", i);
        println!("- {} tests {}", success, "successful".green());
        if i - success != 0 {
            println!("- {} tests {}", i - success, "failed".red());
        }
    }

    fn execute_riscv_test(core: &mut Core, pass_addr: u32, fail_addr: u32) -> u32 {
        loop {
            let pc = core.regs[32] as usize;
            let ins = ((core.memory[pc+3] as u32) << 24)
                | ((core.memory[pc+2] as u32) << 16)
                | ((core.memory[pc+1] as u32) << 8)
                | core.memory[pc] as u32;
            if ins == 0 {
                println!("- {}", "testset failed: reg[pc] == 0".red());
                return 0;
            };
            eval(ins, core);
            if pc == pass_addr as usize {
                println!("- {}", "testset ran successfully!".green());
                return 1;
            }
            else if pc == fail_addr as usize {
                println!("- {}", "testset failed: reg[pc] == fail_addr".red());
                return 0;
            }
        }
    }
}
