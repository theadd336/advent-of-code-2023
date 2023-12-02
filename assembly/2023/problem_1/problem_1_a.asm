extern int_to_ascii
extern print
extern exit
extern open_file
extern read_bytes

section .rodata
input_file:
    db "assembly/2023/problem_1/input.txt", 0

section .bss
answer: resq 2

section .text
    global _start

_start:
    xor rax, rax ; 0 rax to start
    mov rdi, -100 ; Set to -100 for current working dir
    mov rsi, input_file ; Set RSI to the input pointer
    call open_file ; Open the input file
    cmp rax, 0 ; Make sure we have a valid file
    jl _exit_no_file ; If not, exit
    
    ; Set up some variables and stack space
    mov rdi, rax ; Store the file in rdi (it is the input to read)
    mov rsi, rsp ; Store our current pointer
    sub rsp, 1 ; Set up 1 bytes of storage on the stack
    mov rdx, 1 ; Allow 1 bytes to write

    xor r8, r8 ; Store the last number of the line
    xor rbx, rbx ; 0 rbx
    xor r14, r14 ; 0 r14 for initial loop
    xor r10, r10 ; 0 r10 to store the boolean for if we've seen two numbers or not
    xor r12, r12 ; r12 will store the total answer
    xor r13, r13 ; r13 will be used for the evaluate

_loop_over_string:
    call read_bytes ; Call read_bytes to read the file
    test rax, rax ; Check if we read anything
    jz _print_answer ; If we read nothing, we're done.
    mov r13b, byte [rsi] ; Move the current char into r13b
    cmp r13b, 10 ; Check if we're at a new line boundary
    je _handle_line_end
    sub r13b, '0' ; Subtract the offset for 0 from the byte 
    cmp r13b, 10 ; Check if the value is a number
    jge _loop_over_string ; If a letter, continue
    cmp r10, 1 ; Check if we are on the first or second value
    je _store_second_number ; If second, jump to _add the second number and handle that
    inc r10 ; Otherwise, increment r10 for the first number
    mov r14b, r13b ; Move the value from r13b into r14b (first number)
    mov r8b, r13b ; Save off the first number as the second in case there is only one number
    jmp _loop_over_string

_store_second_number:
    mov r8b, r13b ; Update the new second number on the line
    jmp _loop_over_string

_handle_line_end:
    imul r14, 10 ; Multiply r14 by 10 and store to r14
    add r14, r8 ; Add r8 (second number) to r14 (first number * 10)
    add r12, r14 ; Add the total to the running sum
    xor r14, r14 ; Reset r14
    xor r10, r10 ; Reset r10
    xor r8, r8
    jmp _loop_over_string
    
_print_answer:
    mov rdi, answer
    mov rsi, 8
    mov rdx, r12
    call int_to_ascii
    mov rsi, 8
    call print
    mov rsi, 0
    call exit

_exit_no_file:
    mov rdi, answer
    mov rsi, 8
    mov rdx, rax
    call int_to_ascii
    call print
    mov rsi, 1
    call exit
