use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct CPU {
    x: i32,
    instructions: Vec<Instruction>,
}

impl CPU {

    fn new(instructions: Vec<Instruction>) -> Self {
        CPU {
            x: 1,
            instructions,
        }
    }

    fn run(&mut self) -> i32 {

        let mut cycle = 0 ;
        let mut cycle_remaining = 1 ;
        let mut sum = 0 ;
        let mut tmp = 0 ;
        let mut current_instruction = 0 ;

        while current_instruction < self.instructions.len() {
            
            let instruction = &self.instructions[current_instruction] ;
            cycle_remaining -= 1 ;
            cycle += 1 ;

            if cycle_remaining == 0 {

                current_instruction += 1 ;

                self.x += tmp ;

                match instruction {

                    Instruction::Noop => {
                        cycle_remaining = 1 ;
                        tmp = 0
                    },
                    Instruction::AddX(v) => {
                        cycle_remaining = 2 ;
                        tmp = *v ;
                    } 

                } 

            }
            
            if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
                println!("Value of X during cycle {}th : {}", cycle, self.x) ;
                println!("Signal strength during cycle {}th : {}\n", cycle, cycle as i32 * self.x) ;
                sum += cycle as i32 * self.x ;
            }

        }

        sum

    }

    fn synchronize_crt(&mut self) -> [[char;40]; 6] {

        let mut cycle = 0 ;
        let mut cycle_remaining = 1 ;
        let mut tmp = 0 ;
        let mut current_instruction = 0 ;
        let mut sprite = [['.'; 40]; 6] ;

        while current_instruction < self.instructions.len() {

            // Register calculator
            
            let instruction = &self.instructions[current_instruction] ;
            cycle_remaining -= 1 ;
            cycle += 1 ;

            if cycle_remaining == 0 {

                current_instruction += 1 ;

                self.x += tmp ;
                tmp = 0 ;

                match instruction {

                    Instruction::Noop => {
                        cycle_remaining = 1 ;
                        tmp = 0
                    },
                    Instruction::AddX(v) => {
                        cycle_remaining = 2 ;
                        tmp = *v ;
                    } 

                } 

            }

            // Draw sprite

            

        }

        sprite

    }

}

fn main() -> std::io::Result<()> {

    let mut instructions = Vec::new() ;

    // File reading

    let file = File::open("input.txt")? ;
    let reader = BufReader::new(file) ;

    for line in reader.lines() {

        let line = line? ;

        let mut input = line.split(' ') ;
        let instr = input.next().unwrap() ;

        // Ajout des instructions sur la pile
        if instr == "addx" {
            let incr = input.next().unwrap().parse().unwrap() ;
            instructions.push(Instruction::AddX(incr)) ;
        } else {
            instructions.push(Instruction::Noop) ;
        }

    }

    let mut cpu = CPU::new(instructions) ;

    // Partie 1

    let result = cpu.run() ;
    println!("Le résultat est : {result}") ;

    // Partie 2

    let result = cpu.synchronize_crt() ;

    for row in result.iter() {
        for pixel in row.iter() {
            print!("{}", pixel) ;
        }
        println!() ;
    }

    Ok(())

}