    .text
    .globl trap
    .type trap,@function
    .balign 256
trap:
    j   trap_entry
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler          /* software interrupt */
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler        /* timer interrupt */
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler    /* external interrupt */
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .balign 8
    j   some_handler
    .size trap,.-trap

    .global ExcHandler
    .balign 8