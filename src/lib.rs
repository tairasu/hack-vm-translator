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
    pub fn add() -> String {
        //addition
        let mut asm = String::new();
        asm.push_str("@SP\n");
        asm.push_str("M=M-1\n");
        asm.push_str("A=M\n");
        asm.push_str("D=M\n");
        asm.push_str("@SP\n");
        asm.push_str("M=M-1\n");
        asm.push_str("A=M\n");
        asm.push_str("M=M+D\n");
        asm.push_str("@SP\n");
        asm.push_str("M=M+1\n");
        asm
    }
    
    pub fn sub() -> String {
        //subtraction
        let mut asm = String::new();
        asm.push_str("@SP\n");
        asm.push_str("M=M-1\n");
        asm.push_str("A=M\n");
        asm.push_str("D=M\n");
        asm.push_str("@SP\n");
        asm.push_str("M=M-1\n");
        asm.push_str("A=M\n");
        asm.push_str("M=M-D\n");
        asm.push_str("@SP\n");
        asm.push_str("M=M+1\n");
        asm
    }

    static mut LABEL_COUNT: usize = 0;

fn unique_label(prefix: &str) -> String {
    unsafe {
        let label = format!("{}{}", prefix, LABEL_COUNT);
        LABEL_COUNT += 1;
        label
    }
}

pub fn neg() -> String {
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("M=-M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}

pub fn not() -> String {
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("M=!M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}

pub fn and() -> String {
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("M=D&M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}

pub fn or() -> String {
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("M=D|M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}

pub fn eq() -> String {
    let label_true = unique_label("EQ_TRUE");
    let label_end = unique_label("EQ_END");
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M-D\n");
    asm.push_str(&format!("@{}\n", label_true));
    asm.push_str("D;JEQ\n");
    asm.push_str("@SP\n");
    asm.push_str("A=M\n");
    asm.push_str("M=0\n");
    asm.push_str(&format!("@{}\n", label_end));
    asm.push_str("0;JMP\n");
    asm.push_str(&format!("({})\n", label_true));
    asm.push_str("@SP\n");
    asm.push_str("A=M\n");
    asm.push_str("M=-1\n");
    asm.push_str(&format!("({})\n", label_end));
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}

pub fn gt() -> String {
    let label_true = unique_label("GT_TRUE");
    let label_end = unique_label("GT_END");
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M-D\n");
    asm.push_str(&format!("@{}\n", label_true));
    asm.push_str("D;JGT\n");
    asm.push_str("@SP\n");
    asm.push_str("A=M\n");
    asm.push_str("M=0\n");
    asm.push_str(&format!("@{}\n", label_end));
    asm.push_str("0;JMP\n");
    asm.push_str(&format!("({})\n", label_true));
    asm.push_str("@SP\n");
    asm.push_str("A=M\n");
    asm.push_str("M=-1\n");
    asm.push_str(&format!("({})\n", label_end));
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}

pub fn lt() -> String {
    let label_true = unique_label("LT_TRUE");
    let label_end = unique_label("LT_END");
    let mut asm = String::new();
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M\n");
    asm.push_str("@SP\n");
    asm.push_str("M=M-1\n");
    asm.push_str("A=M\n");
    asm.push_str("D=M-D\n");
    asm.push_str(&format!("@{}\n", label_true));
    asm.push_str("D;JLT\n");
    asm.push_str("@SP\n");
    asm.push_str("A=M\n");
    asm.push_str("M=0\n");
    asm.push_str(&format!("@{}\n", label_end));
    asm.push_str("0;JMP\n");
    asm.push_str(&format!("({})\n", label_true));
    asm.push_str("@SP\n");
    asm.push_str("A=M\n");
    asm.push_str("M=-1\n");
    asm.push_str(&format!("({})\n", label_end));
    asm.push_str("@SP\n");
    asm.push_str("M=M+1\n");
    asm
}
    
    //push function for constant, local, argument, this, that, temp, pointer, static
    pub fn push(segment: &str, index: &str) -> String {
        let mut asm = String::new();
        match segment {
            "constant" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "local" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@LCL\n");
                asm.push_str("A=M+D\n");
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "argument" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@ARG\n");
                asm.push_str("A=M+D\n");
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "this" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@THIS\n");
                asm.push_str("A=M+D\n");
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "that" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@THAT\n");
                asm.push_str("A=M+D\n");
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "temp" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@5\n");
                asm.push_str("A=A+D\n");
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "pointer" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@3\n");
                asm.push_str("A=A+D\n");
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            "static" => {
                asm.push_str(format!("@{}.{}\n", "test", index).as_str());
                asm.push_str("D=M\n");
                asm.push_str("@SP\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M+1\n");
            }
            _ => {
                println!("Invalid segment");
            }
        }
        asm
    }
    
    //pop function for local, argument, this, that, temp, pointer, static
    pub fn pop(segment: &str, index: &str) -> String {
        let mut asm = String::new();
        match segment {
            "local" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@LCL\n");
                asm.push_str("D=M+D\n");
                asm.push_str("@R13\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str("@R13\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
            }
            "argument" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@ARG\n");
                asm.push_str("D=M+D\n");
                asm.push_str("@R13\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str("@R13\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
            }
            "this" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@THIS\n");
                asm.push_str("D=M+D\n");
                asm.push_str("@R13\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str("@R13\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
            }
            "that" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@THAT\n");
                asm.push_str("D=M+D\n");
                asm.push_str("@R13\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str("@R13\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
            }
            "temp" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@5\n");
                asm.push_str("D=A+D\n");
                asm.push_str("@R13\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str("@R13\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
            }
            "pointer" => {
                asm.push_str(format!("@{}\n", index).as_str());
                asm.push_str("D=A\n");
                asm.push_str("@3\n");
                asm.push_str("D=A+D\n");
                asm.push_str("@R13\n");
                asm.push_str("M=D\n");
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str("@R13\n");
                asm.push_str("A=M\n");
                asm.push_str("M=D\n");
            }
            "static" => {
                asm.push_str("@SP\n");
                asm.push_str("M=M-1\n");
                asm.push_str("A=M\n");
                asm.push_str("D=M\n");
                asm.push_str(format!("@{}.{}\n", "test", index).as_str());
                asm.push_str("M=D\n");
            }
            _ => {
                println!("Invalid segment");
            }
        }
        asm
    }

    //parse line
    pub fn parse_line(line: &str) -> String {
        let mut split_line = line.split_whitespace();
        let command = split_line.next().unwrap();
        let mut parsed = String::new();
        match command {
            "add" => {
                parsed.push_str(add().as_str());
            }
            "sub" => {
                parsed.push_str(sub().as_str());
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
                parsed.push_str(neg().as_str());
            }
            "not" => {
                parsed.push_str(not().as_str());
            }
            "and" => {
                parsed.push_str(and().as_str());
            }
            "or" => {
                parsed.push_str(or().as_str());
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