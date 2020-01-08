# Rust on RISC-V

* [Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html#conditional-compilation)
* [Built-in attributes](https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index)

## Spike Simulator

* [Homebrew riscv-tools](https://github.com/riscv/homebrew-riscv)

`spike -d --isa=RV32IMAC target/riscv32imac-unknown-none-elf/debug/riscv-demo`

See [start_pc](https://github.com/riscv/riscv-isa-sim/blob/2704790df5d16868571bacf4c521df4bac87f452/riscv/sim.cc#L185)


spike/mtrap.c:
```
static uintptr_t mcall_console_putchar(uint8_t ch)
{
  if (uart) {
    uart_putchar(ch);
  } else if (uart16550) {
    uart16550_putchar(ch);
  } else if (htif) {
    htif_console_putchar(ch);
  }
  return 0;
}
  // Send data to host via HTIF


  volatile uint64_t tohost __attribute__((section(".htif")));
  __set_tohost(dev=1, cmd=1, char);
    tohost = (((uint64_t)(dev) << 56) | ((uint64_t)(cmd) << 48) | (uint64_t)(payload))
```

## QEMU Emulator

### Spike

[docs](https://www.sifive.com/blog/risc-v-qemu-part-2-the-risc-v-qemu-port-is-upstream)

### VirtIO
