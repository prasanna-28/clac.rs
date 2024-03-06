use std::collections::HashMap;

pub fn generate_assembly(ir_code: Vec<String>) -> String {
    let mut asm_code = String::new();
    let mut func_map: HashMap<String, Vec<String>> = HashMap::new();

    asm_code.push_str("extern printf\n");
    asm_code.push_str("\n");
    asm_code.push_str("section .data\n");
    asm_code.push_str("    format db '%ld', 10, 0\n");
    asm_code.push_str("    newline db 10\n");
    asm_code.push_str("\n");
    asm_code.push_str("section .text\n");
    asm_code.push_str("global main\n");
    asm_code.push_str("\n");

    let _push_stack = |asm_code: &mut String, value: &str| {
        asm_code.push_str(&format!("    push {}\n", value));
    };

    let _pop_stack = |asm_code: &mut String| {
        asm_code.push_str("    pop rax\n");
    };


    let mut i = 0;
    while i < ir_code.len() {
        let token = &ir_code[i];
        if token == "FUNCSTART" {
            let func_name = &ir_code[i + 1];
            let func_end = ir_code.iter().skip(i).position(|x| x == "FUNCEND").unwrap() + i;
            let func_body = ir_code[i + 2..func_end].to_vec();
            func_map.insert(func_name.clone(), func_body);
    
            asm_code.push_str(&format!("{}:\n", func_name));
            asm_code.push_str("    pop rcx\n");
            for func_token in &func_map[func_name] {
                match func_token.as_str() {
                    "PLUS" => {
                        asm_code.push_str("    pop rbx\n");
                        asm_code.push_str("    pop rax\n");
                        asm_code.push_str("    add rax, rbx\n");
                        asm_code.push_str("    push rax\n");
                    }
                    "MINUS" => {
                        asm_code.push_str("    pop rbx\n");
                        asm_code.push_str("    pop rax\n");
                        asm_code.push_str("    sub rax, rbx\n");
                        asm_code.push_str("    push rax\n");
                    }
                    "TIMES" => {
                        asm_code.push_str("    pop rbx\n");
                        asm_code.push_str("    pop rax\n");
                        asm_code.push_str("    imul rax, rbx\n");
                        asm_code.push_str("    push rax\n");
                    }
                    "DIVIDE" => {
                        asm_code.push_str("    pop rbx\n");
                        asm_code.push_str("    pop rax\n");
                        asm_code.push_str("    xor rdx, rdx\n");
                        asm_code.push_str("    idiv rbx\n");
                        asm_code.push_str("    push rax\n");
                    }
                    _ => {
                        asm_code.push_str(&format!("    push {}\n", func_token));
                    }
                }
            }
            asm_code.push_str("    push rcx\n");
            asm_code.push_str("    ret\n");
            i = func_end;
        }
        i += 1;
    }
    asm_code.push_str("main:\n");
    asm_code.push_str("    push rbp\n");
    asm_code.push_str("    mov rbp, rsp\n");
    asm_code.push_str("    sub rsp, 32\n");
    
    let mut func_in = false;
    for token in &ir_code {
        if token == "FUNCSTART" {
            func_in = true;
            continue;
        }
    
        if token == "FUNCEND" {
            func_in = false;
            continue;
        }
    
        if func_in {
            continue;
        }
    
        match token.as_str() {
            "PLUS" => {
                asm_code.push_str("    pop rbx\n");
                asm_code.push_str("    pop rax\n");
                asm_code.push_str("    add rax, rbx\n");
                asm_code.push_str("    push rax\n");
            }
            "MINUS" => {
                asm_code.push_str("    pop rbx\n");
                asm_code.push_str("    pop rax\n");
                asm_code.push_str("    sub rax, rbx\n");
                asm_code.push_str("    push rax\n");
            }
            "TIMES" => {
                asm_code.push_str("    pop rbx\n");
                asm_code.push_str("    pop rax\n");
                asm_code.push_str("    imul rax, rbx\n");
                asm_code.push_str("    push rax\n");
            }
            "DIVIDE" => {
                asm_code.push_str("    pop rbx\n");
                asm_code.push_str("    pop rax\n");
                asm_code.push_str("    xor rdx, rdx\n");
                asm_code.push_str("    idiv rbx\n");
                asm_code.push_str("    push rax\n");
            }
            _ => {
                if func_map.contains_key(token) {
                    asm_code.push_str(&format!("    call {}\n", token));
                } else {
                    asm_code.push_str(&format!("    push {}\n", token));
                }
            }
        }
    }



    asm_code.push_str("    ; string buffer space alloc\n");
    asm_code.push_str("    sub rsp, 20\n");
    asm_code.push_str("\n");
    
    asm_code.push_str("    mov rdi, rax\n");
    asm_code.push_str("    mov rsi, rsp\n");
    asm_code.push_str("    call int_to_string\n");
    asm_code.push_str("    mov rsi, rax\n"); // move addr
    asm_code.push_str("\n");
    
    asm_code.push_str("    ; syscall string print\n");
    asm_code.push_str("    mov rax, 1\n");
    asm_code.push_str("    mov rdi, 1\n");
    asm_code.push_str("    mov rdx, 20\n");
    asm_code.push_str("    sub rdx, rsi\n"); // str length
    asm_code.push_str("    add rdx, rsp\n");
    asm_code.push_str("    syscall\n");
    asm_code.push_str("\n");
    
    asm_code.push_str("    ; newline print\n");
    asm_code.push_str("    mov rax, 1\n");
    asm_code.push_str("    mov rdi, 1\n");
    asm_code.push_str("    mov rsi, newline\n");
    asm_code.push_str("    mov rdx, 1\n");
    asm_code.push_str("    syscall\n");
    asm_code.push_str("\n");
    
    asm_code.push_str("    ; stack cleanup\n");
    asm_code.push_str("    add rsp, 20\n");
    asm_code.push_str("\n");
    
    asm_code.push_str("    ; exit\n");
    asm_code.push_str("    mov rsp, rbp\n");
    asm_code.push_str("    pop rbp\n");
    asm_code.push_str("    mov rax, 60\n");
    asm_code.push_str("    xor rdi, rdi\n");
    asm_code.push_str("    syscall\n");
    asm_code.push_str("\n");
    
    asm_code.push_str("; i64 to str\n");
    asm_code.push_str("int_to_string:\n");
    asm_code.push_str("    mov rcx, rsi\n");
    asm_code.push_str("    add rcx, 19\n");
    asm_code.push_str("    mov byte [rcx], 0\n");
    asm_code.push_str("    mov rax, rdi\n");
    asm_code.push_str("    mov rdi, rcx\n");
    asm_code.push_str("    mov r8, 10\n");
    asm_code.push_str("    mov rbx, 0\n");
    asm_code.push_str("\n");
    asm_code.push_str(".loop:\n");
    asm_code.push_str("    xor rdx, rdx\n");
    asm_code.push_str("    div r8\n");
    asm_code.push_str("    add dl, '0'\n");
    asm_code.push_str("    dec rcx\n");
    asm_code.push_str("    mov [rcx], dl\n");
    asm_code.push_str("    cmp rax, 0\n");
    asm_code.push_str("    jne .loop\n");
    asm_code.push_str("\n");
    asm_code.push_str("    mov rax, rcx\n");
    asm_code.push_str("    ret\n");

    asm_code
}