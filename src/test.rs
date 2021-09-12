#[cfg(test)]
mod tests {
    use crate::MEMSIZE;
    use crate::Core;
    use crate::eval;

    #[test]
    fn addi_sp_sp_minus_one() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0xfff10113, core);
        assert_eq!(-1, core.regs[2]);
    }

    #[test]
    fn addi_a4_ra_1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x00108713, core);
        assert_eq!(1, core.regs[14]);
    }

    #[test]
    fn addi_a4_ra_7() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        // ra = -3
        core.regs[1] = -3;
        core = eval(0x00708713, core);
        // 7 + (-3) == 4
        assert_eq!(4, core.regs[14]);
    }

    #[test]
    fn slti_a4_ra_0() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x0000a713, core);
        assert_eq!(0, core.regs[14]);
    }

    #[test]
    fn slti_a4_ra_1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x0010a713, core);
        assert_eq!(1, core.regs[14]);

        core.regs[1] = 1;
        core = eval(0x0010a713, core);
        assert_eq!(0, core.regs[14]);
    }

    #[test]
    fn sltiu_a4_ra_minus_2048() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x8000b713, core);
        assert_eq!(1, core.regs[14]);

        core.regs[1] = -1;
        core = eval(0x8000b713, core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -2048;
        core = eval(0x8000b713, core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -2049;
        core = eval(0x8000b713, core);
        assert_eq!(1, core.regs[14]);
    }
}
