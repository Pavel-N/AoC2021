
fn first_star(diagnostic: Vec<Vec<char>>) {
    let mut gamma_rate = String::new();
    let mut epsilo_rate = String::new();

    for column_index in 0..diagnostic[0].len() {
        let mut zeros = 0;
        let mut ones = 0;

        for row_index in 0..diagnostic.len() {        
            if diagnostic[row_index][column_index] == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }

        if zeros > ones {
            gamma_rate += "0";
            epsilo_rate += "1";
        } else {
            gamma_rate += "1";
            epsilo_rate += "0";
        }
    }


    print!("Gama rate in binary: {}", gamma_rate);
    let gamma_rate = isize::from_str_radix(&gamma_rate.as_str(), 2).unwrap();
    println!(" -> decimal: {}", gamma_rate);

    print!("Epsiolon rate in binary: {}", epsilo_rate);
    let epsilo_rate = isize::from_str_radix(&epsilo_rate.as_str(), 2).unwrap();
    println!(" -> decimal: {}", epsilo_rate);

    println!("Power consumption: {}", gamma_rate*&epsilo_rate);
}


fn second_star(diagnostic: Vec<Vec<char>>) {
    fn most_common(column_index: usize, d: &Vec<Vec<char>>) -> char {
        let zeros = d.iter().filter(|row| row[column_index] == '0').count();
        let ones = d.iter().filter(|row| row[column_index] == '1').count();
            
        if zeros > ones {
            return '0';
        } else if zeros == ones {
            return '1';
        } else {
            return '1';
        }
    }
    
    fn least_common(column_index: usize, d: &Vec<Vec<char>>) -> char {
        let zeros = d.iter().filter(|row| row[column_index] == '0').count();
        let ones = d.iter().filter(|row| row[column_index] == '1').count();
        
        if zeros > ones {
            return '1';
        } else if zeros == ones {
            return '0';
        } else {
            return '0';
        }
    }
    

    let mut oxygen_generator_rating: Vec<Vec<char>> = diagnostic.clone();
    let mut co2_scrubber_rating: Vec<Vec<char>> = diagnostic.clone();

    for column_index in 0..diagnostic[0].len() {
        let mc = most_common(column_index, &oxygen_generator_rating);
        let lc = least_common(column_index, &co2_scrubber_rating);
        
        if oxygen_generator_rating.len() > 1 {
            oxygen_generator_rating = oxygen_generator_rating.clone().iter().filter(|x| x[column_index] == mc).cloned().collect();
        }

        if co2_scrubber_rating.len() > 1 {
            co2_scrubber_rating = co2_scrubber_rating.clone().iter().filter(|x| x[column_index] == lc).cloned().collect();
        }
    }


    let oxygen_generator_rating: String = oxygen_generator_rating[0].clone().into_iter().collect();
    print!("Oxygen generator rating: {}", oxygen_generator_rating);
    let oxygen_generator_rating = isize::from_str_radix(&oxygen_generator_rating.as_str(), 2).unwrap();
    println!(" -> Decimal: {}", oxygen_generator_rating);

    let co2_scrubber_rating: String = co2_scrubber_rating[0].clone().into_iter().collect();
    print!("Oxygen generator rating: {}", co2_scrubber_rating);
    let co2_scrubber_rating = isize::from_str_radix(&co2_scrubber_rating.as_str(), 2).unwrap();
    println!(" -> Decimal: {}", co2_scrubber_rating);

    println!("Life support rating: {}", oxygen_generator_rating * &co2_scrubber_rating);
}

fn main() {
    let diagnostic: Vec<Vec<char>> = std::fs::read_to_string("diagnostic.txt")
        .unwrap()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    
    first_star(diagnostic.clone());
    println!();
    second_star(diagnostic.clone());
}
