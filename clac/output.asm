extern printf

section .data
    format db '%ld', 10, 0
    newline db 10

section .text
global main

ADD4:
    pop rcx
    push 4
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push rcx
    ret
SUB2:
    pop rcx
    push 2
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    push rcx
    ret
main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    push 4
    push 4
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push 8
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    call ADD4
    call SUB2
    push 1
    pop rbx
    pop rax
    add rax, rbx
    push rax
    ; string buffer space alloc
    sub rsp, 20

    mov rdi, rax
    mov rsi, rsp
    call int_to_string
    mov rsi, rax

    ; syscall string print
    mov rax, 1
    mov rdi, 1
    mov rdx, 20
    sub rdx, rsi
    add rdx, rsp
    syscall

    ; newline print
    mov rax, 1
    mov rdi, 1
    mov rsi, newline
    mov rdx, 1
    syscall

    ; stack cleanup
    add rsp, 20

    ; exit
    mov rsp, rbp
    pop rbp
    mov rax, 60
    xor rdi, rdi
    syscall

; i64 to str
int_to_string:
    mov rcx, rsi
    add rcx, 19
    mov byte [rcx], 0
    mov rax, rdi
    mov rdi, rcx
    mov r8, 10
    mov rbx, 0

.loop:
    xor rdx, rdx
    div r8
    add dl, '0'
    dec rcx
    mov [rcx], dl
    cmp rax, 0
    jne .loop

    mov rax, rcx
    ret
