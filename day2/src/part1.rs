use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_won(elf : &str, player : &str) -> i32 {

    if elf == "A" && player == "X" { return 3 ; }
    if elf == "B" && player == "Y" { return 3 ; }
    if elf == "C" && player == "Z" { return 3 ; }

    if elf == "A" && player == "Y" { return 6 ; }
    else if elf == "B" && player == "Z" { return 6 ; }
    else if elf == "C" && player == "X" { return 6 ; }

    0

}

pub fn exec() -> std::io::Result<()> {

    // HashMap to handle the strategy (in particular the score)

    let hm = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)]
    ) ;

    let mut sum_scores = 0 ;

    // File reading

    let file = File::open("input.txt")? ;
    let reader = BufReader::new(file) ;

    for line in reader.lines() {

        let line = line? ;

        let mut tmp = line.split(' ') ;
        let elf = tmp.next().unwrap() ;
        let player = tmp.next().unwrap() ;

        let result = is_won(elf, player) ;

        sum_scores += hm.get(player).unwrap() + result ;

    }

    println!("Le résultat est : {sum_scores}") ;

    Ok(())

}
