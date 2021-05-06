MEMORY
{
  RAM : ORIGIN = 0x1C00, LENGTH = 0x0800
  ROM : ORIGIN = 0x4400, LENGTH = 0xBB80
  VECTORS : ORIGIN = 0xFFCC, LENGTH = 0x34
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .vector_table ORIGIN(VECTORS) : ALIGN(2)
  {
    KEEP(*(.vector_table.interrupts));
    KEEP(*(.__RESET_VECTOR));
  } > VECTORS

  .text ORIGIN(ROM) :
  {
    KEEP(*(.start));

    *(.text .text.*);
  } > ROM

  .rodata : ALIGN(2)
  {
    *(.rodata .rodata.*);
    . = ALIGN(2);
  } > ROM

  .bss : ALIGN(2)
  {
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(2);
    _ebss = .;
  } > RAM

  .data : ALIGN(2)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(2);
    _edata = .;
  } > RAM AT > ROM
}
