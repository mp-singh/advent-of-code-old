use std::fs;

fn main() {
    let stack = read_input();
    println!("{:?}", day7a(&stack));
    println!("{:?}", day7b(&stack));
}

pub fn day7a(stack: &[String]) -> i32 {
    let mut vm = VM::new();
    vm.load_instructions(stack);
    match vm.run() {
        Ok(v) => v,
        Err(e) => e,
    }
}

pub fn day7b(stack: &[String]) -> i32 {
    let mut vm = VM::new();
    vm.load_instructions(stack);
    vm.fix()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let val = s[4..]
            .parse::<i32>()
            .expect("unable to parse instruction val to i32");
        match &s[0..3] {
            "acc" => Instruction::Acc(val),
            "jmp" => Instruction::Jmp(val),
            "nop" => Instruction::Nop(val),
            _ => panic!("unknown instruction"),
        }
    }
}

#[derive(Debug, Default)]
pub struct VM {
    stack: Vec<Instruction>,
    pointer: usize,
    acc: i32,
    visited: Vec<bool>,
}

impl VM {
    pub fn new() -> Self {
        VM::default()
    }

    pub fn load_instructions(&mut self, stack: &[String]) {
        self.stack = stack
            .iter()
            .map(|s| Instruction::from_str(s.as_str()))
            .collect()
    }

    pub fn run(&mut self) -> Result<i32, i32> {
        self.visited = vec![false; self.stack.len()];
        loop {
            match self.get_next_instruction() {
                Ok(Some(next)) => self.execute(next),
                Ok(None) => {
                    break;
                }
                Err(_) => return Err(self.acc),
            }
        }
        Ok(self.acc)
    }

    fn get_next_instruction(&mut self) -> Result<Option<Instruction>, ()> {
        let s = match self.stack.get(self.pointer) {
            Some(s) => *s,
            None => return Ok(None),
        };

        if self.visited[self.pointer] {
            return Err(());
        }

        Ok(Some(s))
    }

    fn execute(&mut self, instruction: Instruction) {
        self.visited[self.pointer] = true;
        match instruction {
            Instruction::Acc(val) => {
                self.pointer += 1;
                self.acc += val
            }
            Instruction::Jmp(val) => self.pointer = (self.pointer as i32 + val) as usize,
            Instruction::Nop(_) => self.pointer += 1,
        }
    }

    fn fix(&mut self) -> i32 {
        for (i, instruction) in self.stack.iter().enumerate() {
            let mut new_stack = self.stack.clone();
            match instruction {
                Instruction::Acc(_) => continue,
                Instruction::Jmp(val) => {
                    new_stack[i] = Instruction::Nop(*val);
                }
                Instruction::Nop(val) => {
                    new_stack[i] = Instruction::Jmp(*val);
                }
            }
            let mut new_vm = VM::new();
            new_vm.stack = new_stack;
            if let Ok(val) = new_vm.run() {
                return val;
            }
        }
        0
    }
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("unable to read file")
        .lines()
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
