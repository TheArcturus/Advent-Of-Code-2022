use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn exec() -> std::io::Result<()> {

    // Stacks and hashmap to get each stacks by his number

    let stack1 : VecDeque<char> = VecDeque::new() ;
    let stack2 : VecDeque<char> = VecDeque::new() ;
    let stack3 : VecDeque<char> = VecDeque::new() ;
    let stack4 : VecDeque<char> = VecDeque::new() ;
    let stack5 : VecDeque<char> = VecDeque::new() ;
    let stack6 : VecDeque<char> = VecDeque::new() ;
    let stack7 : VecDeque<char> = VecDeque::new() ;
    let stack8 : VecDeque<char> = VecDeque::new() ;
    let stack9 : VecDeque<char> = VecDeque::new() ;

    let mut hm = HashMap::from([
        (1, stack1),
        (2, stack2),
        (3, stack3),
        (4, stack4),
        (5, stack5),
        (6, stack6),
        (7, stack7),
        (8, stack8),
        (9, stack9)
    ]) ;

    // Base representation of input
    let input = vec![        
        vec!['B', 'V', 'W', 'T', 'Q', 'N', 'H', 'D'],
        vec!['B', 'W', 'D'],
        vec!['C', 'J', 'W', 'Q', 'S', 'T'],
        vec!['P', 'T', 'Z', 'N', 'R', 'J', 'F'],
        vec!['T', 'S', 'M', 'J', 'V', 'P', 'G'],
        vec!['N', 'T', 'F', 'W', 'B'],
        vec!['N', 'V', 'H', 'F', 'Q', 'D', 'L', 'B'],
        vec!['R', 'F', 'P', 'H'],
        vec!['H', 'P', 'N', 'L', 'B', 'M', 'S', 'Z']
    ] ;

    for (i, array) in input.into_iter().enumerate() {
        for element in array.iter().rev() {
            let stack = hm.get_mut(&(i + 1)).unwrap() ;
            stack.push_back(*element);
        }
    }

    // Result
    let mut code = String::new() ;

    // File reading

    let file = File::open("input.txt")? ;
    let reader = BufReader::new(file) ;

    for line in reader.lines() {

        let line = line? ;

        // Split to get the integers

        let parts: Vec<&str> = line.split(' ').collect() ;
        let crate_number : i32 = parts[1].parse().unwrap() ;
        let src : usize = parts[3].parse().unwrap() ;
        let dst : usize = parts[5].parse().unwrap() ;

        // Move

        for i in 0..crate_number {
            let value = hm.get_mut(&src).unwrap().pop_back().unwrap() ;
            hm.get_mut(&dst).unwrap().push_back(value) ;
        }

    }

    // Reconstitution du code
    
    for i in 1..=9 {

        let stack = hm.get_mut(&i).unwrap() ;
        
        if !stack.is_empty() {
            let last = stack.pop_back().unwrap() ;
            code.push(last) ;
        }

    }

    println!("Le résultat est : {code}") ;

    Ok(())

}