extern int_to_ascii
extern print
extern exit

section .rodata
input_string:
    db ""
    input_string_len equ $ - input_string

section .bss
answer: resq 1

section .text
    global _start

_start:
    mov rsi, 0
    call exit
