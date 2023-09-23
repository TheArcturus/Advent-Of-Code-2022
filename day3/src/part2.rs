use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead};
use std::io ;

pub fn exec() -> std::io::Result<()> {

    // HashMap to handle the strategy 

    let mut hm : HashMap<char, u32> = HashMap::new();

    for i in 0..26 {
        let c = (b'a' + i) as char ;
        hm.insert(c, (i + 1).into())  ;
    }

    for i in 0..26 {
        let c = (b'A' + i) as char ;
        hm.insert(c, (i + 27).into()) ;
    }

    let mut sum_priorities : u32 = 0 ;

    // File reading

    let file = File::open("input.txt")? ;
    let reader = io::BufReader::new(file) ;

    let mut lines = reader.lines();

    // On lit les lignes 3 par 3
    while let Some(line1) = lines.next() {
        
        let line1 = line1? ;

        let line2 = lines.next() ;
        if line2.is_none() {
            println!("No more lines to read") ;
            break ;
        }
        let line2 = line2.unwrap()? ;

        let line3 = lines.next() ;
        if line3.is_none() {
            println!("No more lines to read") ;
            break ;
        }
        let line3 = line3.unwrap()? ;

        // Intersection

        let mut set1 : HashSet<char> = line1.chars().collect() ;
        let set2 : HashSet<char> = line2.chars().collect() ;
        let set3 : HashSet<char> = line3.chars().collect() ;
    
        set1.retain(|c| set2.contains(c) && set3.contains(c)) ;

        // Get and sum priorities

        for c in set1.iter() {
            sum_priorities += hm.get(c).unwrap() ;
        }
    
    }

    println!("Le résultat est : {sum_priorities}") ;

    Ok(())

}
