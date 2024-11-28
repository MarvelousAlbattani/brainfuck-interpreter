use std::{io::stdin, usize};

#[derive(Debug)]
enum Token {
    MoveRight,
    MoveLeft,
    Add,
    Sub,
    Input,
    Print,
    StartLoop,
    EndLoop,
    None
}


impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::MoveRight => Token::MoveRight,
            Token::MoveLeft => Token::MoveLeft,
            Token::Add => Token::Add,
            Token::Sub => Token::Sub,
            Token::Print => Token::Print,
            Token::Input => Token::Input,
            Token::StartLoop => Token::StartLoop,
            Token::EndLoop => Token::EndLoop,
            Token::None => Token::None
        }
    }
}

#[derive(PartialEq)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Add,
    Sub,
    Input,
    Print,
    Loop(Vec<Instruction>),
    None
}

fn main() {
    let mut ip: u32 = 0;
    let mut tape: [u32; 256000] = [0; 256000];

    let mut code: String = String::new();

    println!("Insert brainfuck code -> ");
    stdin()
        .read_line(&mut code)
        .expect("Did not enter a correct string");
    
    let mut tokens: Vec<Token> = Vec::new();

    lexer(code, &mut tokens);

    let instructions: Vec<Instruction> = parser(tokens);

    run(instructions);
}

fn run(instructions: Vec<Instruction>) {
    for instruction in instructions {
        match instruction {
            Instruction::MoveRight => println!("MoveRight"),
            Instruction::MoveLeft => println!("MoveLeft"),
            Instruction::Add => println!("Add"),
            Instruction::Sub => println!("Sub"),
            Instruction::Print => println!("Print"),
            Instruction::Input => println!("Input"),
            Instruction::Loop(instructions) => {
                println!("Loop")
            },
            _ => println!("Nothin")
        } 
    }
}

fn parser(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut starting_loop_index: i32 = -1;
    let mut ending_loop_index: i32 = -1;
    let mut sub_loops_counter: u8 = 0;

    let mut instructions: Vec<Instruction> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        let instruction: Instruction = match token {
            Token::MoveRight => if sub_loops_counter == 0 { Instruction::MoveRight } else { Instruction::None },
            Token::MoveLeft => if sub_loops_counter == 0 { Instruction::MoveLeft } else { Instruction::None },
            Token::Add => if sub_loops_counter == 0 { Instruction::Add } else { Instruction::None },
            Token::Sub => if sub_loops_counter == 0 { Instruction::Sub } else { Instruction::None },
            Token::Print => if sub_loops_counter == 0 { Instruction::Print } else { Instruction::None },
            Token::Input => if sub_loops_counter == 0 { Instruction::Input } else { Instruction::None },
            Token::StartLoop => {
                if sub_loops_counter == 0 {
                    starting_loop_index = index as i32;
                }
                    
                sub_loops_counter += 1;

                Instruction::None
            },
            Token::EndLoop => {
                let mut result: Instruction = Instruction::None;

                if sub_loops_counter == 1 {
                    ending_loop_index = index as i32;

                    let loop_tokens: Vec<Token> = tokens[
                        ((starting_loop_index as usize) + 1)..(ending_loop_index as usize)
                    ].to_vec();
                    println!("aa");
                    println!("{:?}", loop_tokens);

                    result = Instruction::Loop(
                        parser(loop_tokens)
                    );

                    starting_loop_index = -1;
                    ending_loop_index = -1;
                } else {
                    sub_loops_counter -= 1;
                }

                result
            },
            Token::None => Instruction::None
        };

        if instruction != Instruction::None {
            instructions.push(instruction);
        }
    }

    instructions
}

fn lexer(code: String, tokens: &mut Vec<Token>) {
    let exploded_code: Vec<char> = code.chars().collect();
    
    for command in exploded_code {
        let token: Token = match command {
            '>' => Token::MoveRight,
            '<' => Token::MoveLeft,
            '+' => Token::Add,
            '-' => Token::Sub,
            '.' => Token::Print,
            ',' => Token::Input,
            '[' => Token::StartLoop,
            ']' => Token::EndLoop,
            '\n' => break,
            _ => Token::None
        };

        tokens.push(token);
    }
}

/*
*
* Token::MoveRight => move_right(ip),
            Token::MoveLeft => move_left(ip),
            Token::Add => add(&mut tape[*ip as usize]),
            Token::Sub => sub(&mut tape[*ip as usize]),
            Token::Print => print_tape_cell(&mut tape[*ip as usize]),
            Token::Input => user_input(&mut tape[*ip as usize]),
            Token::StartLoop => {
                if sub_loops_counter == 0 {
                    starting_loop_index = index as u8; 
                }
                    
                sub_loops_counter += 1;
            },
            Token::EndLoop => {
                if sub_loops_counter == 0 {
                    ending_loop_index = index as u8;
                    
                    ()
                }

                sub_loops_counter -= 1;
            },
            Token::None => ()*/

fn user_input(cell_value: &mut u32) {
    let mut input_value = String::new();
    stdin().read_line(&mut input_value).expect("Failed to read line");

    *cell_value = input_value.trim().parse().unwrap_or(0);
}

fn print_tape_cell(cell_value: &mut u32) {
    print!("{}", char::from_u32(*cell_value).unwrap());
}

fn check_loop(cell_value: &mut u32, end_loop: &mut bool) {
    *end_loop = *cell_value == 0;
}

fn redo_loop(tokens: &[char], current_token_index: &mut usize) {
    // se tornando indietro trovo parentesi chiuse ] le conto in modo da saltare lo stesso numero
    // di parentesi aperte
    let mut sub_loops_counter: u32 = 0;

    while *current_token_index != 0 {
        *current_token_index -= 1;
        if sub_loops_counter == 0 && tokens[*current_token_index] == '[' {
            break;
        } else if sub_loops_counter > 0 && tokens[*current_token_index] == '[' {
            sub_loops_counter -= 1;
        } else if tokens[*current_token_index] == ']' {
            sub_loops_counter += 1;
        }
    }
}

fn move_right(ip: &mut u32) {
    *ip += 1;
}

fn move_left(ip: &mut u32) {
    *ip -= 1;
}

fn add(cell_value: &mut u32) {
    *cell_value += 1;
}

fn sub(cell_value: &mut u32) {
    if *cell_value - 1 < 0 {
        *cell_value = 0;
    } else {
        *cell_value -= 1;
    }
}

