#[cfg(test)]
mod tests {
    use crate::MEMSIZE;
    use crate::Core;
    use crate::eval;

    #[test]
    fn addi_sp_sp_minus_one() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0xfff10113, &mut core);
        assert_eq!(-1, core.regs[2]);
    }

    #[test]
    fn addi_a4_ra_1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x00108713, &mut core);
        assert_eq!(1, core.regs[14]);
    }

    #[test]
    fn addi_a4_ra_7() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        // ra = -3
        core.regs[1] = -3;
        eval(0x00708713, &mut core);
        // 7 + (-3) == 4
        assert_eq!(4, core.regs[14]);
    }

    #[test]
    fn slti_a4_ra_0() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x0000a713, &mut core);
        assert_eq!(0, core.regs[14]);
    }

    #[test]
    fn slti_a4_ra_1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x0010a713, &mut core);
        assert_eq!(1, core.regs[14]);

        core.regs[1] = 1;
        eval(0x0010a713, &mut core);
        assert_eq!(0, core.regs[14]);
    }

    #[test]
    fn sltiu_a4_ra_minus_2048() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x8000b713, &mut core);
        assert_eq!(1, core.regs[14]);

        core.regs[1] = -1;
        eval(0x8000b713, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -2048;
        eval(0x8000b713, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -2049;
        eval(0x8000b713, &mut core);
        assert_eq!(1, core.regs[14]);
    }

    #[test]
    fn andi_a4_ra_240() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x0f00f713, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = 16;
        eval(0x0f00f713, &mut core);
        assert_eq!(16, core.regs[14]);
    }

    #[test]
    fn ori_a4_ra_minus_241() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0xf0f0e713, &mut core);
        assert_eq!(-241, core.regs[14]);

        core.regs[1] = 16;
        eval(0xf0f0e713, &mut core);
        assert_eq!(-225, core.regs[14]);
    }

    #[test]
    fn xori_a4_ra_minus_241() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0xf0f0c713, &mut core);
        assert_eq!(-241, core.regs[14]);

        core.regs[1] = 156;
        eval(0xf0f0c713, &mut core);
        assert_eq!(-109, core.regs[14]);
    }

    #[test]
    fn slli_a4_ra_7() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x00709713, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = 156;
        eval(0x00709713, &mut core);
        assert_eq!(19968, core.regs[14]);

        core.regs[1] = -1;
        eval(0x00709713, &mut core);
        assert_eq!(-128, core.regs[14]);
    }

    #[test]
    fn srli_a4_ra_1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x0010d713, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = 5;
        eval(0x0010d713, &mut core);
        assert_eq!(2, core.regs[14]);

        core.regs[1] = -10;
        eval(0x0010d713, &mut core);
        assert_eq!(2147483643, core.regs[14]);

    }

    #[test]
    fn srai_a4_ra_14() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 5;
        eval(0x40e0d713, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -10;
        eval(0x40e0d713, &mut core);
        assert_eq!(-1, core.regs[14]);

        core.regs[1] = 32768;
        eval(0x40e0d713, &mut core);
        assert_eq!(2, core.regs[14]);
    }

    #[test]
    fn lui_ra_0x7ffff() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x7ffff0b7, &mut core);
        assert_eq!(0x7ffff000, core.regs[1]);
    }

    #[test]
    fn auipc_a0_2() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        eval(0x00002517, &mut core);
        core.regs[32] = 0;
        assert_eq!(0x00002000, core.regs[10]);

        core.regs[32] = 4;
        eval(0x00002517, &mut core);
        assert_eq!(0x00002004, core.regs[10]);
    }

    #[test]
    fn add_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 1;
        core.regs[2] = -5;
        eval(0x00208733, &mut core);
        assert_eq!(-4, core.regs[14]);

        core.regs[1] = 159;
        core.regs[2] = 123;
        eval(0x00208733, &mut core);
        assert_eq!(282, core.regs[14]);
    }

    #[test]
    fn sub_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 123;
        core.regs[2] = 23;
        eval(0x40208733, &mut core);
        assert_eq!(100, core.regs[14]);

        core.regs[1] = -12;
        core.regs[2] = -5;
        eval(0x40208733, &mut core);
        assert_eq!(-7, core.regs[14]);
    }

    #[test]
    fn slt_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 2;
        core.regs[2] = 3;
        eval(0x0020a733, &mut core);
        assert_eq!(1, core.regs[14]);

        core.regs[1] = 3;
        core.regs[2] = 3;
        eval(0x0020a733, &mut core);
        assert_eq!(0, core.regs[14]);

        core.regs[1] = -1;
        core.regs[2] = 3;
        eval(0x0020a733, &mut core);
        assert_eq!(1, core.regs[14]);
    }

    #[test]
    fn sltu_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 2;
        core.regs[2] = 3;
        eval(0x0020b733, &mut core);
        assert_eq!(1, core.regs[14]);

        core.regs[1] = -1;
        core.regs[2] = 3;
        eval(0x0020b733, &mut core);
        assert_eq!(0, core.regs[14]);
    }

    #[test]
    fn and_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = -4723;
        core.regs[2] = 32489;
        eval(0x0020f733, &mut core);
        assert_eq!(27785, core.regs[14]);

        core.regs[1] = 17;
        core.regs[2] = 7;
        eval(0x0020f733, &mut core);
        assert_eq!(1, core.regs[14]);
    }

    #[test]
    fn or_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = -4723;
        core.regs[2] = 32489;
        eval(0x0020e733, &mut core);
        assert_eq!(-19, core.regs[14]);

        core.regs[1] = 17;
        core.regs[2] = 7;
        eval(0x0020e733, &mut core);
        assert_eq!(23, core.regs[14]);
    }

    #[test]
    fn xor_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = -4723;
        core.regs[2] = 32489;
        eval(0x0020c733, &mut core);
        assert_eq!(-27804, core.regs[14]);

        core.regs[1] = 17;
        core.regs[2] = 7;
        eval(0x0020c733, &mut core);
        assert_eq!(22, core.regs[14]);
    }

    #[test]
    fn sll_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = -4723;
        core.regs[2] = 32489;
        eval(0x00209733, &mut core);
        assert_eq!(-2418176, core.regs[14]);

        core.regs[1] = 17;
        core.regs[2] = 7;
        eval(0x00209733, &mut core);
        assert_eq!(2176, core.regs[14]);
    }

    #[test]
    fn srl_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = -4723;
        core.regs[2] = 32489;
        eval(0x0020d733, &mut core);
        assert_eq!(8388598, core.regs[14]);

        core.regs[1] = 1024;
        core.regs[2] = 4;
        eval(0x0020d733, &mut core);
        assert_eq!(64, core.regs[14]);
    }

    #[test]
    fn sra_a4_ra_sp() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = -4723;
        core.regs[2] = 32489;
        eval(0x4020d733, &mut core);
        assert_eq!(-10, core.regs[14]);

        core.regs[1] = 1024;
        core.regs[2] = 4;
        eval(0x4020d733, &mut core);
        assert_eq!(64, core.regs[14]);
    }

    #[test]
    fn jal_tp_16() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[32] = 0;
        eval(0x0100026f, &mut core);
        assert_eq!(16, core.regs[32]);
        assert_eq!(4, core.regs[4]);

        core.regs[32] = 4;
        eval(0x0100026f, &mut core);
        assert_eq!(20, core.regs[32]);
        assert_eq!(8, core.regs[4]);
    }

    #[test]
    fn jalr_t0_t1() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[32] = 0;
        // jalr t0,t1,0
        eval(0x000302e7, &mut core);
        assert_eq!(0, core.regs[32]);
        assert_eq!(4, core.regs[5]);

        core.regs[32] = 4;
        core.regs[6] = 4;
        eval(0x000302e7, &mut core);
        assert_eq!(4, core.regs[32]);
        assert_eq!(8, core.regs[5]);

        core.regs[32] = 0;
        core.regs[6] = 4;
        // jalr t0,t1,-4
        eval(0xffc302e7, &mut core);
        assert_eq!(0, core.regs[32]);
        assert_eq!(4, core.regs[5]);
    }

    #[test]
    fn beq_ra_sp_12() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[32] = 4;
        eval(0x00208663, &mut core);
        assert_eq!(16, core.regs[32]);
    }

    #[test]
    fn bne_ra_sp_12() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[32] = 4;
        core.regs[1] = 1;
        core.regs[2] = 2;
        eval(0x00209663, &mut core);
        assert_eq!(16, core.regs[32]);
    }

    #[test]
    fn blt_ra_sp_12() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[32] = 4;
        core.regs[1] = 1;
        core.regs[2] = 2;
        eval(0x0020c663, &mut core);
        assert_eq!(16, core.regs[32]);

        core.regs[32] = 4;
        core.regs[1] = -1;
        core.regs[2] = 2;
        eval(0x0020c663, &mut core);
        assert_eq!(16, core.regs[32]);
    }

    #[test]
    fn bltu_ra_sp_12() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[32] = 4;
        core.regs[1] = -1;
        core.regs[2] = 2;
        eval(0x0020e663, &mut core);
        assert_ne!(16, core.regs[32]);
    }

    #[test]
    fn bge_ra_sp_12() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 2;
        core.regs[2] = 1;
        eval(0x0020d663, &mut core);
        assert_eq!(12, core.regs[32]);

        core.regs[32] = 0;
        core.regs[1] = -1;
        core.regs[2] = 2;
        eval(0x0020d663, &mut core);
        assert_ne!(12, core.regs[32]);
    }

    #[test]
    fn bgeu_ra_sp_12() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 2;
        core.regs[2] = 1;
        eval(0x0020f663, &mut core);
        assert_eq!(12, core.regs[32]);

        core.regs[32] = 0;
        core.regs[1] = -1;
        core.regs[2] = 2;
        eval(0x0020f663, &mut core);
        assert_eq!(12, core.regs[32]);
    }

    #[test]
    fn lb_a4_0_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 4;
        core.regs[14] = 0;
        core.memory[4] = 127; // => 0b01111111
        eval(0x00008703, &mut core);
        assert_eq!(127, core.regs[14]);

        core.regs[1] = 4;
        core.regs[14] = 0;
        core.memory[4] = 255; // 0b11111111 => -1
        eval(0x00008703, &mut core);
        assert_eq!(-1, core.regs[14]);
    }

    #[test]
    fn lbu_a4_0_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 4;
        core.regs[14] = 0;
        core.memory[4] = 127;
        eval(0x0000c703, &mut core);
        assert_eq!(127, core.regs[14]);

        core.regs[1] = 4;
        core.regs[14] = 0;
        core.memory[4] = 255;
        eval(0x0000c703, &mut core);
        assert_eq!(255, core.regs[14]);
    }

    #[test]
    fn lh_a4_2_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 2;
        core.regs[14] = 0;
        core.memory[4] = 0b00001110;
        core.memory[5] = 0b1; // mem[4-5] = 00000001 00001110 = 270
        eval(0x00209703, &mut core);
        assert_eq!(270, core.regs[14]);

        core.regs[1] = 2;
        core.regs[14] = 0;
        core.memory[4] = 0b11111111;
        core.memory[5] = 0b11111111; // => mem[4-5] = 0xffff = -1
        eval(0x00209703, &mut core);
        assert_eq!(-1, core.regs[14]);
    }

    #[test]
    fn lhu_a4_2_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 2;
        core.regs[14] = 0;
        core.memory[4] = 0b00001110;
        core.memory[5] = 0b1;
        eval(0x0020d703, &mut core);
        assert_eq!(270, core.regs[14]);

        core.regs[1] = 2;
        core.regs[14] = 0;
        core.memory[4] = 0b11111111;
        core.memory[5] = 0b11111111;
        eval(0x0020d703, &mut core);
        assert_eq!(0xffff, core.regs[14]);
    }

    #[test]
    fn lw_a4_8_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 0;
        core.regs[14] = -1;
        core.memory[8] = 0b1;
        core.memory[9] = 0b1;
        core.memory[10] = 0b1;
        core.memory[11] = 0b1; // => mem[8-11] = 0x1010101
        eval(0x0080a703, &mut core);
        assert_eq!(0x1010101, core.regs[14]);

        core.regs[1] = 0;
        core.regs[14] = -1;
        core.memory[8] = 0xff;
        core.memory[9] = 0xff;
        core.memory[10] = 0xff;
        core.memory[11] = 0xff; // => mem[8-11] = 0xffffffff = -1
        eval(0x0080a703, &mut core);
        assert_eq!(-1, core.regs[14]);
    }

    #[test]
    fn sb_sp_0_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 4;
        core.regs[2] = 1;
        eval(0x00208023, &mut core);
        assert_eq!(1, core.memory[4]);

        core.regs[1] = 4;
        core.regs[2] = -1;
        eval(0x00208023, &mut core);
        assert_eq!(0xff, core.memory[4]);
    }

    #[test]
    fn sh_sp_4_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 0;
        core.regs[2] = 1048575; // 2**20 -1
        eval(0x00209223, &mut core);
        assert_eq!(0xff, core.memory[4]);
        assert_eq!(0xff, core.memory[5]);

        core.regs[1] = 0;
        core.regs[2] = -2;
        eval(0x00209223, &mut core);
        assert_eq!(0xfe, core.memory[4]);
        assert_eq!(0xff, core.memory[5]);
    }

    #[test]
    fn sw_sp_8_ra() {
        let mut core = Core { memory: [0;MEMSIZE], regs: [0;33] };
        core.regs[1] = 0;
        core.regs[2] = 2490785; // = 00100110 00000001 10100001
        eval(0x0020a423, &mut core);
        assert_eq!(0b10100001, core.memory[8]);
        assert_eq!(0b1, core.memory[9]);
        assert_eq!(0b100110, core.memory[10]);
        assert_eq!(0b0, core.memory[11]);
    }
}
