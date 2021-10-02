# rust-v

A simple RISC-V Core in Rust.

## Features

- RV32I
  - FENCE is not implemented since this is only single core.
  - No ECALL / EBREAK since no implementation for privileged spec.

## Tests

Run simple unit tests with
```
cargo t unit_tests
```

### riscv-tests

To run the *rv32ui-p* tests from https://github.com/riscv-software-src/riscv-tests
you need to:
1. Install [riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain).
2. Set RISCV environment variable to RISC-V tools install path.
3. Run:
```
./get_riscv_tests.sh
cargo t riscv_tests
```

The tests are run with a dumb elf reader that loads instructions into memory and
figures out the fail / pass addresses. System level instructions are ignored.

## Specs

https://riscv.org/technical/specifications/
