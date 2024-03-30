use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // input string
    input: String,
}

const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}

impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        // let mut numboats="";
        // for boat in boats {
        //     numboats += boat.to_string();
        // }
        let mut contents = fs::read_to_string("src/board.txt")
                .expect("Should have been able to read the file");
        println!("{}",contents);
        // let mut firs_line=[" ";4];
        let mut firs_line=String::new();
        for boat in boats {
            firs_line.push(boat.to_string().parse().unwrap());
            firs_line.push(' ');
        }
        fs::write("src/board.txt", firs_line).expect("Unable to write file");
        let b=Board {
            boats: [1,2,3,4],
            data: [[0; BSIZE]; BSIZE],
        };
        let boar = [[" "; BSIZE]; BSIZE];
        for datum in b.data {


            }
        }
        b
        // fs::write("src/board.txt", line).expect("Unable to write file");
    }
    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    // pub fn from(s: String) -> (Board) {
    //     let mut contents = fs::read_to_string("src/test.txt")
    //         .expect("Should have been able to read the file");
    // }
    // /* aggiunge la nave alla board, restituendo la nuova board se
    // possibile */
    // /* bonus: provare a *non copiare* data quando si crea e restituisce
    // una nuova board con la barca, come si può fare? */
    // pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {}
    // /* converte la board in una stringa salvabile su file */
    // pub fn to_string(&self) -> String{}
}

fn main() {
    // let arg = Args::parse();
    // let numbers = arg.input.split(" ");
    // for number in numbers {
    //     println!("{}",number);
    // }
    let a:u8 = 4;
    let data= [[a; BSIZE]; BSIZE];
    // println!("{:?}",data);
    let board = Board::new(&[1,2,3,4]);
}
