use std::fmt;
use std::fmt::Formatter;
use std::ptr::write;

#[derive(Clone)]
enum Cell{
    Dead,
    Alive
}

struct Board{
    generation: u32,
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl fmt::Display for Board{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Generation {} \n\n", self.generation).expect("Error reading generation");
        for i in 0..self.cells.len(){
            let cell = self.cells.get(i).unwrap();
            match cell {
                Cell::Alive => write!(f, "⬜ "),
                Cell::Dead => write!(f, "⬛ ")
            }.expect("Error reading cell");
            if (i % self.width as usize) == (self.width as usize - 1){
                write!(f, "\n");

            }
        }
        write!(f, "{}", "=".repeat(80))

    }
}

impl Board{
    fn new(w: u32, h: u32) -> Board{
        Board{
            generation: 1,
            width: w,
            height: h,
            cells: vec![Cell::Dead; (w * h) as usize]
        }
    }

    fn display(&self){
        println!("{}", self)
    }

    fn get_index(&self, x: u32, y:u32) -> usize{
        if x >= self.width && y >= self.height{
            panic!("Invalid coordinates")
        }
        (y * self.width + x) as usize
    }

    fn get_cell(&self, n: usize) -> &Cell{
        let cell = self.cells.get(n).unwrap();
        return cell
    }

    fn set_cell(&mut self, x: u32, y:u32, new_val:Cell){
        let index = self.get_index(x, y);
        self.cells[index] = new_val;
    }

    fn number_of_neighbors(&self, x: u32, y:u32) -> u32{
        let mut count = 0;

        if x >= self.width || y >= self.height{
            panic!("Out of range")
        }

        for i in 0..3 {
            for j in 0..3{
                if i==1 && j ==1 { continue; }
                if (x + i == 0 || y + j == 0 || x + i > self.width || y + j > self.height){continue}
                let current_x = x + i - 1;
                let current_y = y + j - 1;
                let index = self.get_index(current_x, current_y);
                let cell = self.get_cell(index);
                match cell {
                    Cell::Alive => count += 1,
                    _ => ()
                }
            }
        }
        return count
    }

    fn elapse(&mut self){
        self.generation += 1;

        let mut next_generation : Vec<Cell> = Vec::new();
        for y in 0..self.height{
            for x in 0..self.width{
                let index = self.get_index(x, y);
                let cell = self.get_cell(index);
                let neighbor = self.number_of_neighbors(x, y);

                match cell {
                    Cell::Alive => {
                        if neighbor < 2 {
                            next_generation.push(Cell::Dead);
                        } else if neighbor == 2 || neighbor == 3 {
                            next_generation.push(Cell::Alive)
                        } else {
                            next_generation.push(Cell::Dead)
                        }

                    },
                    Cell::Dead => {
                        if neighbor == 3 {
                            next_generation.push(Cell::Alive)
                        } else {
                            next_generation.push(Cell::Dead)
                        }
                    }
                }
            }
        }
        self.cells = next_generation;
    }
}

fn main() {
    let mut board1 = Board::new(10, 10);
    board1.set_cell(3,0, Cell::Alive);
    board1.set_cell(3,1, Cell::Alive);
    board1.set_cell(3,2, Cell::Alive);
    board1.set_cell(8,9, Cell::Alive);
    board1.set_cell(9,8, Cell::Alive);
    board1.set_cell(8,9, Cell::Alive);
    board1.set_cell(9,9, Cell::Alive);
    for i in 0..10 {
        board1.display();
        board1.elapse();

    }

}
