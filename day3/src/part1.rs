use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

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
    let reader = BufReader::new(file) ;

    for line in reader.lines() {

        let line = line? ;

        // Split components of each ruckstack
        let (component1, component2) = line.split_at(line.len()/2) ;

        // Intersection

        let set1: HashSet<char> = component1.chars().collect() ;
        let set2: HashSet<char> = component2.chars().collect() ;
    
        let common: HashSet<_> = set1.intersection(&set2).collect() ;

        // Get and sum priorities

        for c in common.iter() {
            sum_priorities += hm.get(c).unwrap() ;
        }
    

    }

    println!("Le résultat est : {sum_priorities}") ;

    Ok(())

}
