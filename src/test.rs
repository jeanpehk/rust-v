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

    #[test]
    fn andi_a4_ra_240() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x0f00f713, core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = 16;
        core = eval(0x0f00f713, core);
        assert_eq!(16, core.regs[14]);
    }

    #[test]
    fn ori_a4_ra_minus_241() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0xf0f0e713, core);
        assert_eq!(-241, core.regs[14]);

        core.regs[1] = 16;
        core = eval(0xf0f0e713, core);
        assert_eq!(-225, core.regs[14]);
    }

    #[test]
    fn xori_a4_ra_minus_241() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0xf0f0c713, core);
        assert_eq!(-241, core.regs[14]);

        core.regs[1] = 156;
        core = eval(0xf0f0c713, core);
        assert_eq!(-109, core.regs[14]);
    }

    #[test]
    fn slli_a4_ra_7() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x00709713, core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = 156;
        core = eval(0x00709713, core);
        assert_eq!(19968, core.regs[14]);

        core.regs[1] = -1;
        core = eval(0x00709713, core);
        assert_eq!(-128, core.regs[14]);
    }

    #[test]
    fn srli_a4_ra_1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x0010d713, core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = 5;
        core = eval(0x0010d713, core);
        assert_eq!(2, core.regs[14]);

        core.regs[1] = -10;
        core = eval(0x0010d713, core);
        assert_eq!(2147483643, core.regs[14]);

    }

    #[test]
    fn srai_a4_ra_14() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 5;
        core = eval(0x40e0d713, core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -10;
        core = eval(0x40e0d713, core);
        assert_eq!(-1, core.regs[14]);

        core.regs[1] = 32768;
        core = eval(0x40e0d713, core);
        assert_eq!(2, core.regs[14]);
    }

    #[test]
    fn lui_ra_0x7ffff() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x7ffff0b7, core);
        assert_eq!(0x7ffff000, core.regs[1]);
    }

    #[test]
    fn auipc_a0_2() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core = eval(0x00002517, core);
        core.regs[32] = 0;
        assert_eq!(0x00002000, core.regs[10]);

        core.regs[32] = 4;
        core = eval(0x00002517, core);
        assert_eq!(0x00002004, core.regs[10]);
    }
}
