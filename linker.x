MEMORY
{
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 512K
  RAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 128K
}

ENTRY(Reset);

EXTERN(RESET_VECTOR);
EXTERN(EXTI13_INTERRUPT_HANDLER);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    KEEP(*(.vector_table.reset_vector));

    . = ORIGIN(FLASH) + 0xE0;
    KEEP(*(.vector_table.EXTI13_INTERRUPT_HANDLER));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.* .comment);
  }
}