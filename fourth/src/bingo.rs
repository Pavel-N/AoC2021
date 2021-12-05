use std::fmt::{Debug, Display, Write};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BingoCell {
    value: u32,
    checked: bool
}

impl BingoCell {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            checked: false
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BingoGame {
    data: [[BingoCell; 5]; 5],
    pub is_bingo: bool
}

impl BingoGame {
    pub fn new(input: Vec<Vec<u32>>) -> Self {
        let mut data = [[BingoCell::new(0); 5]; 5];
        
        for row_index in 0..data.len() {
            for cell_index in 0..data[row_index].len() {
                data[row_index][cell_index] = BingoCell::new(input[row_index][cell_index])
            }
        }

        Self { 
            data,
            is_bingo: false
        }
    }

    pub fn eval(&mut self) {
        // Check rows
        for row in self.data {
            if row.iter().all(|x| x.checked == true) {
                self.is_bingo = true;
            }
        }

        // Check columns
        for i in 0..5 {
            let mut column: Vec<BingoCell> = Vec::new();

            for j in 0..5 {
                column.push(self.data[j][i]);
            }

            if column.iter().all(|c| c.checked == true) {
                self.is_bingo = true;
            }
        }
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;

        for row in self.data {
            for cell in row {
                if !cell.checked {
                    sum += cell.value;
                }
            }
        }

        sum
    }
}

impl Display for BingoGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data {
            for cell in row {
                if cell.checked {
                    f.write_fmt(format_args!("{} ", "XX"))?;
                } else {
                    f.write_fmt(format_args!("{:2} ", cell.value))?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}



pub struct BingoHall {
    called_numbers: Vec<u32>,
    pub last_called: u32,
    pub bingos: Vec<BingoGame>
}

impl BingoHall {
    pub fn new(called_numbers: Vec<u32>, bingos: Vec<BingoGame>) -> Self {
        Self {
            called_numbers,
            last_called: 999,
            bingos
        }
    }

    pub fn call_number(&mut self) {
        if self.called_numbers.len() > 0 {
            for bingo in self.bingos.iter_mut() {
                for row in bingo.data.iter_mut() {
                    for cell in row {
                        if cell.value == self.called_numbers[0] {
                            cell.checked = true;
                        }
                    }
                }
            }

            self.last_called = self.called_numbers[0];
            self.called_numbers.remove(0);
        }
    }

    pub fn eval(&mut self) -> Option<usize> {
        for (i, bingo) in self.bingos.iter_mut().enumerate() {
            bingo.eval();

            if bingo.is_bingo {
                return Some(i);
            }
        }

        None
    }
}
