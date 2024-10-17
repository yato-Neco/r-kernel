OUTPUT_ARCH( "riscv" )
ENTRY( _entry )


SECTIONS
{
  . = 0x80200000;

  .text : {
    *(.entry)
    *(.text .text.*)
  }

  

  .rodata : {
    *(.rdata .rodata .rodata.*)
  }

  .data : {
    *(.data .data.*)
  }

  .bss : {
    *(.bss bss.*)
  }
  . = ALIGN(16);
  . += 128 * 1024; /* 128KB */
  __stack_top = .;




    . = ALIGN(4096);
    __free_ram = .;
    . += 64 * 1024 * 1024; /* 64MB */
    __free_ram_end = .;
    
}

