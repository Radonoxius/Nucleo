.text

    .syntax unified
    .global _sbss
    .global _ebss

    .global _sdata
    .global _edata
    .global _sidata

    .global main
    .global Reset

    .type Reset,%function
    .thumb_func
    Reset:

        _init_bss:
            movs r2, #0
            ldr r0, =_sbss
            ldr r1, =_ebss

        1:
            cmp r1, r0
            beq _init_data
            strb r2, [r0]
            add r0, #1
            b 1b

        _init_data:
            ldr r0, =_sdata
            ldr r1, =_edata
            ldr r2, =_sidata

        1:
            cmp r0, r1
            beq _main_trampoline
            ldrb r3, [r2]
            strb r3, [r0]
            add r0, #1
            add r2, #1
            b 1b

        _main_trampoline:
            ldr r0, =main
            bx r0