# rust-v

A simple RISC-V Core in Rust.

## Features

- RV32I
  - FENCE is not implemented since this is only single core.
  - No ECALL / EBREAK since no implementation for privileged spec.

## Tests

Run simple unit tests with
```
cargo test
```

Also passes the *rv32ui-p* tests from https://github.com/riscv-software-src/riscv-tests.
- Done with a dumb elf reader that loads instructions into memory and figures out the fail / pass addresses.
- System level instructions are ignored.

## Specs

https://riscv.org/technical/specifications/
