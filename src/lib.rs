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
        "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M+D\n@SP\nM=M+1\n"
    }
    
    pub fn sub() -> &'static str{
        "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1\n"
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
    "@SP\nM=M-1\nA=M\nM=-M\n@SP\nM=M+1\n"
}

pub fn not() -> &'static str{
    "@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1\n"
}

pub fn and() -> &'static str{
    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D&M\n@SP\nM=M+1\n"
}

pub fn or() -> &'static str{
    "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D|M\n@SP\nM=M+1\n"
}

pub fn eq() -> String {
    let label_true = unique_label("EQ_TRUE");
    let label_end = unique_label("EQ_END");
    format!(
        "@SP\n\
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
        "@SP\n\
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
        "@SP\n\
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
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "local" => {
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "argument" => {
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "this" => {
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "that" => {
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "temp" => {
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@5\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "pointer" => {
                asm.push_str(&format!("@{}\n", index));
                asm.push_str("D=A\n@3\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
            }
            "static" => {
                asm.push_str(&format!("@{}.{}\n", "test", index));
                asm.push_str("D=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
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