//#![allow(dead_code)]


mod bingo;
use bingo::{BingoGame, BingoHall};


fn first_star(mut data: Vec<String>) {
    // Load called numbers
    let called_numbers: Vec<u32> = data[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();


    data.remove(0);  // Remove called numbers
    data.remove(0);  // Remove empty line
    

    // Load bingos
    let mut bingos: Vec<BingoGame> = Vec::new();
    while data.len() > 0 {
        let mut x:Vec<Vec<u32>> = Vec::new();
        for i in 0..5 {
            x.push(
                data[i]
                    .split_ascii_whitespace()
                    .map(|n| n.to_string().parse().unwrap())
                    .collect()
            );
        }
    
        bingos.push(BingoGame::new(x));

        for _ in 0..5 {
            data.remove(0);
        }

        if data.len() > 0 {
            data.remove(0);
        }

    }

    // Create Bingo Hall
    let mut bingo_hall = BingoHall::new(called_numbers, bingos);

    // Play
    let mut res: Option<usize>;
    loop {
        bingo_hall.call_number();
        res = bingo_hall.eval();

        // BINGO!
        if res.is_some() {
            break;
        }
    }
    
    // View boards
    /* for bingo in bingo_hall.bingos.clone() {
        println!("{}", bingo);
    } */
    
    println!("Winner index: {}", res.unwrap());
    let sum = bingo_hall.bingos[res.unwrap()].sum_unmarked();
    println!("Unmarked sum: {}", sum);
    println!("Last number called: {}", bingo_hall.last_called);
    println!("Score: {}", sum * bingo_hall.last_called);
}


fn second_star(mut data: Vec<String>) {
    // Load called numbers
    let called_numbers: Vec<u32> = data[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();


    data.remove(0);  // Remove called numbers
    data.remove(0);  // Remove empty line
    

    // Load bingos
    let mut bingos: Vec<BingoGame> = Vec::new();
    while data.len() > 0 {
        let mut x:Vec<Vec<u32>> = Vec::new();
        for i in 0..5 {
            x.push(
                data[i]
                    .split_ascii_whitespace()
                    .map(|n| n.to_string().parse().unwrap())
                    .collect()
            );
        }
    
        bingos.push(BingoGame::new(x));

        for _ in 0..5 {
            data.remove(0);
        }

        if data.len() > 0 {
            data.remove(0);
        }
    }

    // Create Bingo Hall
    let mut bingo_hall = BingoHall::new(called_numbers, bingos);  
    
    // Play
    loop {
        bingo_hall.call_number();

        // Filter out won bingos except tghe last one
        loop {
            if let Some(index) = bingo_hall.eval() {
                if bingo_hall.bingos.len() == 1 {
                    break;
                } 

                bingo_hall.bingos.remove(index);
            } else {
                break;
            }
        }
        
        // When the last bingo wins, end the game
        if bingo_hall.bingos[0].is_bingo == true {
            break;
        }
    }

    let sum = bingo_hall.bingos[0].sum_unmarked();
    println!("Unmarked sum: {}", sum);
    println!("Last number called: {}", bingo_hall.last_called);
    println!("Score: {}", sum * bingo_hall.last_called);
}


fn main() {
    let data: Vec<String> = std::fs::read_to_string("bingo.txt").unwrap().lines().map(|s| s.to_string()).collect();

    first_star(data.clone());
    println!();
    second_star(data.clone());
}
