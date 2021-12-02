
fn first_star(depths: Vec<u32>) {
    let mut increments = 0u32;
    for i in 1..depths.len() {
        if depths[i-1] < depths[i] {
            increments += 1;
        }
    }

    println!("(1) Depths increments: {}", increments);
}


fn second_star(depths: Vec<u32>) {
    let mut increments = 0u32;
    for i in 3..depths.len() {
        let a = depths[i-3] + depths[i-2] + depths[i-1];
        let b = depths[i-2] + depths[i-1] + depths[i];
        
        if b > a {
            increments += 1;
        }
    }

    println!("(2) Depths increments: {}", increments);
}


fn main() {
    let depths: Vec<u32> = std::fs::read_to_string("depths.txt")
        .unwrap()
        .split('\n')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    first_star(depths.clone());
    second_star(depths.clone());
}
