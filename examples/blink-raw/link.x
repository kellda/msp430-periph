MEMORY
{
  /* Where the RAM (random access memory), ROM (read-only memory) and interrupt
   * vectors start in the MSP430FR5969 as well as their size */
  RAM : ORIGIN = 0x1C00, LENGTH = 0x0800
  ROM : ORIGIN = 0x4400, LENGTH = 0xBB80
  VECTORS : ORIGIN = 0xFFCC, LENGTH = 0x34
}

/* The stack starts at the end of the RAM and grows downwards
 * This will be used by our initialisation code */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  /* Put interrupts vectors and reset vector in the `.vector_table` section, in
   * the `VECTORS` memory region */
  .vector_table ORIGIN(VECTORS) : ALIGN(2)
  {
    KEEP(*(.vector_table.interrupts));
    KEEP(*(.__RESET_VECTOR));
  } > VECTORS

  /* Put startup code followed by all other code in the `.text` section, in the
   * `ROM` memory region */
  .text ORIGIN(ROM) :
  {
    KEEP(*(.start));

    *(.text .text.*);
  } > ROM

  /* Put read-only data in the `.rodata` section, also in the `ROM` */
  .rodata : ALIGN(2)
  {
    *(.rodata .rodata.*);
    . = ALIGN(2);
  } > ROM

  /* Put zeroed data in the `.bss` section, in the `RAM` memory region
   * `_sbss` and `_ebss` allows to know the start and the end of this section
   * to initialize it from the Rust code (not implemented) */
  .bss : ALIGN(2)
  {
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(2);
    _ebss = .;
  } > RAM

  /* Put initialized data in the `.data` section, both in the `RAM` to reserve
   * space for it and in the `ROM` to know the initial values
   * `_sdata` and `_edata` allows to know the start and the end of this section
   * to initialize it from the Rust code (not implemented) */
  .data : ALIGN(2)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(2);
    _edata = .;
  } > RAM AT > ROM
}
