section .rodata
newline: db 10

section .text
; Prints a string to stdout.
; rdi - pointer to the string
; rsi - length of the string
; Will blow out rdx and rax
global print
print:
    mov rax, 1 ; system call 1 = write
    mov rdx, rsi ; rdx = length of the string (second param)
    mov rsi, rdi ; rsi = pointer to the string (first param)
    mov rdi, 1 ; fd 1 = stdout
    syscall
    ; Print a newline
    mov rdx, 1
    mov rsi, newline
    mov rax, 1
    mov rdi, 1
    syscall
    ret

; Converts an int to an ascii integer
; rdi - pointer to the output string location (restored)
; rsi - max length of the string to write
; rdx - input number
; rax [output] - 0 on success, 1 on failure
; rdx [output] - length of the string written
global int_to_ascii
int_to_ascii:
    push rdi
    add rdi, rsi
    mov byte [rdi], 0
    mov rax, rdx
    mov rcx, 10
    xor r8, r8
    xor r11, r11
    ; handle negatives here
    mov r9, rdx
    mov r10, 1 << 63
    and r9, r10
    cmp r9, r10
    jne .next_digit
    dec eax
    xor eax, 0xFFFFFFFFFFFFFFFF
    mov r11, 1
.next_digit:
    xor rdx, rdx
    div rcx
    add dl, '0'
    dec rdi
    mov [rdi], dl
    inc r8
    test rax, rax
    jnz .next_digit
    cmp r11, 1
    jne .ret
    dec rdi
    mov byte [rdi], '-'
    inc r8
.ret:
    mov rdx, r8
    pop rdi
    ret

; Opens a file with the following values
; Inputs: 
; rdi - dirfd (use -100 for current working dir)
; rsi - pathname (name of the file)
; Outputs: 
; rax - < 0 on error, file descriptor on success
global open_file
open_file:
    mov rax, 257 ; 257 is syscall for openat
    xor rdx, rdx ; No flags (read only mode)
    xor r10, r10 ; Default mode
    syscall
    ret

; Reads some bytes from an open file descriptor
; Inputs: 
; rdi - file descriptor
; rsi - pointer to the buffer to write
; rdx - size of the buffer to write
; Outputs:
; rax - bytes read (0 if file is finished) 
global read_bytes
read_bytes:
    mov rax, 0 ; 0 is sys_read
    syscall
    ret

; Exits with the return code specified in rsi
global exit
exit:
    mov     rax, 60               ; system call 60 is exit
    mov     rdi, rsi              ; return code 0
    syscall
    ret ; Should never be hit

