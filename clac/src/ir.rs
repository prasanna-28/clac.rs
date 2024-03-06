use std::collections::HashMap;
use std::collections::HashSet;

pub fn convert_to_ir(tokens: Vec<String>) -> Vec<String> {
    let mut ir_code: Vec<String> = Vec::new();

    let mut in_func = false;

    let mut func_name = false;

    let mut func_set: HashSet<String> = HashSet::new();


    // TODO: let mut curr_stack_size = 0;
    // let mut curr_stack_frame:String = "".to_string();
    // let mut func_lengths: HashMap<String, i32> = HashMap::new();

    let mut tok_map = HashMap::new();
    
    tok_map.insert("+".to_string(), "PLUS".to_string());
    tok_map.insert("-".to_string(), "MINUS".to_string());
    tok_map.insert("*".to_string(), "TIMES".to_string());
    tok_map.insert("/".to_string(), "DIVIDE".to_string());
    tok_map.insert("%".to_string(), "MOD".to_string());
    tok_map.insert("**".to_string(), "POWER".to_string());
    tok_map.insert("<".to_string(), "LESSTHAN".to_string());
    tok_map.insert("<=".to_string(), "LESSTHANEQUAL".to_string());
    tok_map.insert(">".to_string(), "GREATERTHAN".to_string());
    tok_map.insert(">=".to_string(), "GREATERTHANEQUAL".to_string());
    tok_map.insert("==".to_string(), "EQUAL".to_string());
    tok_map.insert("!=".to_string(), "NOTEQUAL".to_string());
    tok_map.insert("drop".to_string(), "DROP".to_string());
    tok_map.insert("swap".to_string(), "SWAP".to_string());
    tok_map.insert("rot".to_string(), "ROT".to_string());
    tok_map.insert("pick".to_string(), "PICK".to_string());
    tok_map.insert("print".to_string(), "PRINT".to_string());
    tok_map.insert("quit".to_string(), "QUIT".to_string());
    tok_map.insert("if".to_string(), "IF".to_string());
    tok_map.insert("skip".to_string(), "SKIP".to_string());

    for token in tokens.iter(){

        if func_name{
            if let Some(_value) = tok_map.get(token) {
                panic!("Error Parsing: Cannot use built in token {} as function name.", token);
            }else if token == ":" || token == ";"{
                panic!("Error Parsing: Function declarators cannot be used as function names.");
            }else if func_set.contains(token) {
                panic!("Error Parsing: {} declared multiple times.", token);
            }else{
                func_set.insert(token.to_string());
                ir_code.push(token.to_string());
            }
            func_name = false;
            continue;
        }

        if token == ":" {
            if !in_func {
                ir_code.push("FUNCSTART".to_string());
                in_func = true;
                func_name = true;
            }else{
                panic!("Error Parsing: ':' found within ':' ';' block.");
            }
            continue;
        }

        if token == ";" {
            if in_func {
                ir_code.push("FUNCEND".to_string());
                in_func = false;
            }else{
                panic!("Error Parsing: ';' found without corresponding ':'");
            }
            continue;
        }

        if token.parse::<i32>().is_ok() {
            ir_code.push(token.to_string());
            continue;
        }

        if let Some(value) = tok_map.get(token) {
            ir_code.push(value.to_string());
            continue;
        }

        if func_set.contains(token){
            ir_code.push(token.to_string());
            continue;
        }

        panic!("Error Parsing: {} is not a valid token.", token);

    }

    if in_func {panic!("Error Parsing: ':' found wihtout corresponding ';'")}

    ir_code
}