
struct Submarine1 {
    horizontal_pos: u32,
    depth: u32,
}

impl Submarine1 {
    pub fn new() -> Self {
        Self {
            horizontal_pos: 0,
            depth: 0,
        }
    }

    pub fn forward(&mut self, x: u32) {
        self.horizontal_pos += x;
    }

    pub fn down(&mut self, x: u32) {
        self.depth += x;
    }

    pub fn up(&mut self, x: u32) {
        self.depth -= x;
    }

    pub fn product(&self) -> u32 {
        self.horizontal_pos * self.depth
    }
}


fn first_star(instructions: Vec<String>) {
    let mut sub = Submarine1::new();

    for ins in instructions {
        match ins.split_at(ins.find(' ').unwrap()) {
            (command, x) => match (
                command, 
                x.to_string().trim().parse::<u32>().unwrap()
            ) {
                ("forward", x) => sub.forward(x),
                ("down", x) => sub.down(x),
                ("up", x) => sub.up(x),
                (_, _) => panic!("Unknown command!")
            }
        }

    }

    println!("Distance product: {}", sub.product());
}


struct Submarine2 {
    horizontal_pos: u32,
    depth: u32,
    aim: u32
}

impl Submarine2 {
    pub fn new() -> Self {
        Self {
            horizontal_pos: 0,
            depth: 0,
            aim: 0
        }
    }

    pub fn forward(&mut self, x: u32) {
        self.horizontal_pos += x;
        self.depth += self.aim * x;
    }

    pub fn down(&mut self, x: u32) {
        self.aim += x;
    }

    pub fn up(&mut self, x: u32) {
        self.aim -= x;
    }

    pub fn product(&self) -> u32 {
        self.horizontal_pos * self.depth
    }
}


fn second_star(instructions: Vec<String>) {
    let mut sub = Submarine2::new();

    for ins in instructions {
        match ins.split_at(ins.find(' ').unwrap()) {
            (command, x) => match (
                command,
                x.to_string().trim().parse::<u32>().unwrap()
            ) {
                ("forward", x) => sub.forward(x),
                ("down", x) => sub.down(x),
                ("up", x) => sub.up(x),
                (_, _) => panic!("Unknown command!")
            }
        }

    }

    println!("Distance product: {}", sub.product());
}


fn main() {
    let instructions: Vec<String> = std::fs::read_to_string("instructions.txt")
        .unwrap()
        .split('\n')
        .map(|i| i.to_string())
        .collect();

    first_star(instructions.clone());
    second_star(instructions.clone());
}
