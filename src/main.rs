use std::{io::stdin, usize};

// lexer tokens
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
    None,
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
            Token::None => Token::None,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Add,
    Sub,
    Input,
    Print,
    Loop(Vec<Instruction>),
    None,
}

fn main() {
    // instruction pointer for moving on the tape
    let mut ip: u8 = 0;
    let mut tape: [i8; 256000] = [0; 256000];

    let mut code: String = String::new();

    println!("Insert brainfuck code:");
    // ask for input
    stdin()
        .read_line(&mut code)
        .expect("Did not enter a correct string");

    let mut tokens: Vec<Token> = Vec::new();

    // transform plain brainfuck code to an array of lexems
    lexer(code, &mut tokens);

    // get an array of instructions from lexems
    let instructions: Vec<Instruction> = parser(tokens);

    // executes instructions of brainfuck lexems in rust
    run(instructions, &mut tape, &mut ip);
}

// function to translate instructions in real operations on tape
fn run(instructions: Vec<Instruction>, tape: &mut [i8; 256000], ip: &mut u8) {
    for instruction in instructions {
        match instruction {
            Instruction::MoveRight => move_right(ip),
            Instruction::MoveLeft => move_left(ip),
            Instruction::Add => add(&mut tape[*ip as usize]),
            Instruction::Sub => sub(&mut tape[*ip as usize]),
            Instruction::Print => print_tape_cell(&mut tape[*ip as usize]),
            Instruction::Input => user_input(&mut tape[*ip as usize]),
            Instruction::Loop(inner_instructions) => {
                while tape[*ip as usize] > 0 {
                    run(inner_instructions.clone(), tape, ip);
                }
            }
            _ => (),
        }
    }
}

// parse lexems to instructions
fn parser(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut starting_loop_index: i8 = -1;
    let mut ending_loop_index: i8 = -1;
    let mut sub_loops_counter: u8 = 0;

    let mut instructions: Vec<Instruction> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        let instruction: Instruction = match token {
            Token::MoveRight => {
                // this conditions is needed to avoid creating instructions
                // when parsing loops
                if sub_loops_counter == 0 {
                    Instruction::MoveRight
                } else {
                    Instruction::None
                }
            }
            Token::MoveLeft => {
                if sub_loops_counter == 0 {
                    Instruction::MoveLeft
                } else {
                    Instruction::None
                }
            }
            Token::Add => {
                if sub_loops_counter == 0 {
                    Instruction::Add
                } else {
                    Instruction::None
                }
            }
            Token::Sub => {
                if sub_loops_counter == 0 {
                    Instruction::Sub
                } else {
                    Instruction::None
                }
            }
            Token::Print => {
                if sub_loops_counter == 0 {
                    Instruction::Print
                } else {
                    Instruction::None
                }
            }
            Token::Input => {
                if sub_loops_counter == 0 {
                    Instruction::Input
                } else {
                    Instruction::None
                }
            }
            Token::StartLoop => {
                if sub_loops_counter == 0 {
                    starting_loop_index = index as i8;
                }

                // this variable is used to avoid creating a stack
                // and understand when this loop ends
                sub_loops_counter += 1;

                Instruction::None
            }
            Token::EndLoop => {
                let mut result: Instruction = Instruction::None;

                // at the end of the loop so now 
                // we can get the right index to 
                // create an array of instructions
                // and generate a Loop instruction
                if sub_loops_counter == 1 {
                    ending_loop_index = index as i8;

                    // create a sub-array and transform
                    // startLoop and endLoop in only one
                    // loop instruction
                    let loop_tokens: Vec<Token> = tokens
                        [((starting_loop_index as usize) + 1)..(ending_loop_index as usize)]
                        .to_vec();

                    result = Instruction::Loop(parser(loop_tokens));

                    // reset variables for next loops
                    starting_loop_index = -1;
                    ending_loop_index = -1;

                    sub_loops_counter = 0;
                } else {
                    // if other endLoops are encountered
                    // we can jump them to get the right
                    // endloop
                    sub_loops_counter -= 1;
                }

                result
            }
            Token::None => Instruction::None,
        };

        // save instructions only when needed
        if instruction != Instruction::None {
            instructions.push(instruction);
        }
    }

    instructions
}

// populates the tokens vector with lexems
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
            _ => Token::None,
        };

        tokens.push(token);
    }
}

// from now on there will be only functions associated
// with brainfuck functionalities

// asks for user inputs and write the inserted value 
// on the tape at the currently pointed cell
fn user_input(cell_value: &mut i8) {
    let mut input_value = String::new();
    stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    *cell_value = input_value.trim().parse().unwrap_or(0);
}

// print the current pointed tape cell
fn print_tape_cell(cell_value: &mut i8) {
    let unsigned = *cell_value as u8;
    let char = unsigned as char;

    if (*cell_value >= 0) {
        print!("{}", char);
    }
}

// moves the instruction pointer to the right
fn move_right(ip: &mut u8) {
    *ip += 1;
}

// moves the instruction pointer to the left
fn move_left(ip: &mut u8) {
    *ip -= 1;
}

// adds 1 unit to the current pointed tape cell
fn add(cell_value: &mut i8) {
    *cell_value += 1;
}

// removes 1 unit from the current pointed tape cell
fn sub(cell_value: &mut i8) {
    *cell_value -= 1;
}
