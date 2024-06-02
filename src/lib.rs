pub mod load {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;
    use std::error::Error;

    pub fn load_file(file_path: &str) -> Result<String, Box<dyn Error>> {
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let contents = sanitize(contents);
        Ok(contents)
    }

    //sanitize
    pub fn sanitize(file_content: String) -> String {
        let mut sanitized_content = String::new();
        for line in file_content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("//") {
                continue;
            }
            if line.contains("//") {
                let comment_index = line.find("//").unwrap();
                let line = &line[..comment_index];
                sanitized_content.push_str(line);
            } else {
                sanitized_content.push_str(line);
            }
            sanitized_content.push_str("\n");
        }
        sanitized_content
    }

    pub fn write_file(file_path: String, content: String) -> Result<(), Box<dyn Error>> {
        std::fs::write(file_path, content)?;
        Ok(())
    }
}

//instructions
pub mod instructions {
    pub fn add() -> &'static str{
        "//add\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M+D\n@SP\nM=M+1\n"
    }
    
    pub fn sub() -> &'static str{
        "//sub\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1\n"
    }

    static mut LABEL_COUNT: usize = 0;

fn unique_label(prefix: &str) -> String {
    unsafe {
        let label = format!("{}{}", prefix, LABEL_COUNT);
        LABEL_COUNT += 1;
        label
    }
}

pub fn neg() -> &'static str {
    "//neg\n@SP\nM=M-1\nA=M\nM=-M\n@SP\nM=M+1\n"
}

pub fn not() -> &'static str{
    "//not\n@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1\n"
}

pub fn and() -> &'static str{
    "//and\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D&M\n@SP\nM=M+1\n"
}

pub fn or() -> &'static str{
    "//or\n@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D|M\n@SP\nM=M+1\n"
}

pub fn eq() -> String {
    let label_true = unique_label("EQ_TRUE");
    let label_end = unique_label("EQ_END");
    format!(
        "//eq\n@SP\n\
        M=M-1\n\
        A=M\n\
        D=M\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        D=M-D\n\
        @{}\n\
        D;JEQ\n\
        @SP\n\
        A=M\n\
        M=0\n\
        @{}\n\
        0;JMP\n\
        ({})\n\
        @SP\n\
        A=M\n\
        M=-1\n\
        ({})\n\
        @SP\n\
        M=M+1\n",
        label_true, label_end, label_true, label_end
    )
}

pub fn gt() -> String {
    let label_true = unique_label("GT_TRUE");
    let label_end = unique_label("GT_END");
    format!(
        "//gt\n@SP\n\
        M=M-1\n\
        A=M\n\
        D=M\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        D=M-D\n\
        @{}\n\
        D;JGT\n\
        @SP\n\
        A=M\n\
        M=0\n\
        @{}\n\
        0;JMP\n\
        ({})\n\
        @SP\n\
        A=M\n\
        M=-1\n\
        ({})\n\
        @SP\n\
        M=M+1\n",
        label_true, label_end, label_true, label_end
    )
}

pub fn lt() -> String {
    let label_true = unique_label("LT_TRUE");
    let label_end = unique_label("LT_END");
    format!(
        "//lt\n@SP\n\
        M=M-1\n\
        A=M\n\
        D=M\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        D=M-D\n\
        @{}\n\
        D;JLT\n\
        @SP\n\
        A=M\n\
        M=0\n\
        @{}\n\
        0;JMP\n\
        ({})\n\
        @SP\n\
        A=M\n\
        M=-1\n\
        ({})\n\
        @SP\n\
        M=M+1\n",
        label_true, label_end, label_true, label_end
    )
}
    
    //push function for constant, local, argument, this, that, temp, pointer, static
    pub fn push(segment: &str, index: &str) -> String {
        let mut asm = String::with_capacity(100); // Preallocate with an estimated capacity
        
        match segment {
            "constant" => {
                asm.push_str(&format!("//push constant {}\n@{}\n", index, index));
                asm.push_str("D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "local" => {
                asm.push_str(&format!("//push local {}\n@{}\n", index, index));
                asm.push_str("D=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "argument" => {
                asm.push_str(&format!("//push argument {}\n@{}\n", index, index));
                asm.push_str("D=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "this" => {
                asm.push_str(&format!("//push this {}\n@{}\n", index, index));
                asm.push_str("D=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "that" => {
                asm.push_str(&format!("//push that {}\n@{}\n", index, index));
                asm.push_str("D=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "temp" => {
                asm.push_str(&format!("//push temp {}\n@{}\n", index, index));
                asm.push_str("D=A\n@5\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "pointer" => {
                asm.push_str(&format!("//push pointer {}\n@{}\n", index, index));
                asm.push_str("D=A\n@3\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "static" => {
                asm.push_str(&format!("//push static\n@{}.{}\n", "test", index));
                asm.push_str("D=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "saveCaller" => { //CUSTOM SEGMENT: just for saving LCL, ARG, THIS, THAT
                asm.push_str(&format!("//push saveCaller {}\n@{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index, index));
            }
            _ => {
                println!("Invalid segment");
            }
        }
        asm
    }
    
    //pop function for local, argument, this, that, temp, pointer, static
    pub fn pop(segment: &str, index: &str) -> String {
        let mut asm = String::with_capacity(100); // Preallocate with an estimated capacity
        
        match segment {
            "local" => {
                asm.push_str(&format!("@{}\nD=A\n@LCL\nD=M+D\n@R13\nM=D\n", index));
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n");
            }
            "argument" => {
                asm.push_str(&format!("@{}\nD=A\n@ARG\nD=M+D\n@R13\nM=D\n", index));
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n");
            }
            "this" => {
                asm.push_str(&format!("@{}\nD=A\n@THIS\nD=M+D\n@R13\nM=D\n", index));
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n");
            }
            "that" => {
                asm.push_str(&format!("@{}\nD=A\n@THAT\nD=M+D\n@R13\nM=D\n", index));
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n");
            }
            "temp" => {
                asm.push_str(&format!("@{}\nD=A\n@5\nD=A+D\n@R13\nM=D\n", index));
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n");
            }
            "pointer" => {
                asm.push_str(&format!("@{}\nD=A\n@3\nD=A+D\n@R13\nM=D\n", index));
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@R13\nA=M\nM=D\n");
            }
            "static" => {
                asm.push_str("@SP\nM=M-1\nA=M\nD=M\n");
                asm.push_str(&format!("@{}.{}\nM=D\n", "test", index));
            }
            _ => {
                println!("Invalid segment");
            }
        }
        asm
    }


    pub fn label(label: &str) -> String {
        format!("//label {}\n({})\n", label, label)
    }

    pub fn goto(label: &str) -> String {
        format!("//goto {}\n@{}\n0;JMP\n", label, label)
    }

    pub fn if_goto(label: &str) -> String {
        format!(
            "//if_goto {}\n
            @SP\n\
            AM=M-1\n\
            D=M\n\
            @{}\n\
            D;JNE\n",
            label,
            label
        )
    }

    pub fn call(function_name: &str,n_args:i16) -> String {
        let ret_addr_label = unique_label(&format!("{}$retAddr", function_name));
        let mut asm = String::new();
        asm.push_str(&format!("//call {} {}\n", function_name, n_args));
        asm.push_str(&push("constant", &ret_addr_label));
        asm.push_str(&push("saveCaller", "LCL"));
        asm.push_str(&push("saveCaller", "ARG"));
        asm.push_str(&push("saveCaller", "THIS"));
        asm.push_str(&push("saveCaller", "THAT"));
        //ARG = SP - nArgs - 5
        asm.push_str(&format!("@SP\nD=M\n@5\nD=D-A\n@{}\nD=D-A\n@ARG\nM=D\n", n_args));
        //LCL = SP
        asm.push_str("@SP\nD=M\n@LCL\nM=D\n");
        //goto functionName
        asm.push_str(&goto(function_name));
        //return address label
        asm.push_str(&label(&ret_addr_label));
        asm
    }

    pub fn function(function_name: &str, n_locals: i16) -> String {
        let mut asm = String::new();
        asm.push_str(&format!("//function {} {}\n", function_name, n_locals));
        asm.push_str(&label(function_name));
        for _ in 0..n_locals {
            asm.push_str(&push("constant", "0"));
        }
        asm
    }

    pub fn return_() -> String {
        let mut asm = String::new();  
        asm.push_str("// return\n");    
        // FRAME = LCL
        asm.push_str("@LCL\nD=M\n@R13\nM=D\n"); 
        // RET = *(FRAME-5)
        asm.push_str("@5\nA=D-A\nD=M\n@R14\nM=D\n");    
        // *ARG = pop()
        asm.push_str("@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\n");    
        // SP = ARG + 1
        asm.push_str("@ARG\nD=M+1\n@SP\nM=D\n");    
        // THAT = *(FRAME-1)
        asm.push_str("@R13\nAM=M-1\nD=M\n@THAT\nM=D\n");    
        // THIS = *(FRAME-2)
        asm.push_str("@R13\nAM=M-1\nD=M\n@THIS\nM=D\n");
        // ARG = *(FRAME-3)
        asm.push_str("@R13\nAM=M-1\nD=M\n@ARG\nM=D\n");
        // LCL = *(FRAME-4)
        asm.push_str("@R13\nAM=M-1\nD=M\n@LCL\nM=D\n");
        // goto RET
        asm.push_str("@R14\nA=M\n0;JMP\n");
        asm
    }


    //parse line
    pub fn parse_line(line: &str) -> String {
        let mut split_line = line.split_whitespace();
        let command = split_line.next().unwrap();
        let mut parsed = String::new();
        match command {
            "add" => {
                parsed.push_str(add());
            }
            "sub" => {
                parsed.push_str(sub());
            }
            "push" => {
                let segment = split_line.next().unwrap();
                let index = split_line.next().unwrap();
                parsed.push_str(push(segment, index).as_str());
            }
            "pop" => {
                let segment = split_line.next().unwrap();
                let index = split_line.next().unwrap();
                parsed.push_str(pop(segment, index).as_str());
            }
            "neg" => {
                parsed.push_str(neg());
            }
            "not" => {
                parsed.push_str(not());
            }
            "and" => {
                parsed.push_str(and());
            }
            "or" => {
                parsed.push_str(or());
            }
            "eq" => {
                parsed.push_str(eq().as_str());
            }
            "gt" => {
                parsed.push_str(gt().as_str());
            }
            "lt" => {
                parsed.push_str(lt().as_str());
            }
            "label" => {
                parsed.push_str(label(split_line.next().unwrap()).as_str());
            }
            "goto" => {
                let label = split_line.next().unwrap();
                parsed.push_str(goto(label).as_str());
            }
            "if-goto" => {
                let label = split_line.next().unwrap();
                parsed.push_str(if_goto(label).as_str());
            }
            "call" => {
                let function_name = split_line.next().unwrap();
                let n_args = split_line.next().unwrap().parse::<i16>().unwrap();
                parsed.push_str(call(function_name, n_args).as_str());
            }
            "function" => {
                let function_name = split_line.next().unwrap();
                let n_locals = split_line.next().unwrap().parse::<i16>().unwrap();
                parsed.push_str(function(function_name, n_locals).as_str());
            }
            "return" => {
                parsed.push_str(return_().as_str());
            }
            _ => {
                println!("Invalid command");
            }
        }
        parsed
    }

    //parse file
    pub fn parse_file(file_content: String) -> String {
        let mut parsed = String::new();
        for line in file_content.lines() {
            parsed.push_str(parse_line(line).as_str());
            parsed.push_str("\n");
        }
        parsed.pop();
        parsed
    }
    
}