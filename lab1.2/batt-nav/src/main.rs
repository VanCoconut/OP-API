use std::fs;
use std::str;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::{Empty, Write};

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

        let mut contents = OpenOptions::new()
            .append(true)
            .open("src/board.txt")
            .expect("cannot open file");

        // Write to a file
        // let mut contents = fs::read_to_string("src/board.txt")
        //     .expect("Should have been able to read the file");
        let mut string = str::from_utf8(&boats).unwrap().replace(","," ");
        string.push_str("\n");
        println!("{}",string);

        contents.write( string.as_bytes()).expect("Unable to write file");
        let b=Board {
            boats: [boats[0], boats[1], boats[2], boats[3]],
            data: [[0; BSIZE]; BSIZE],
        };
        contents.write( b.to_string().as_bytes()).expect("Unable to write file");
        b
    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String{
        let mut res= String::new();
        for riga in self.data{
            for elemento in riga{
                if elemento == 0 {
                    res.push_str("0 ");
                }
                else if elemento==1{
                    res.push_str("1 ");
                }
            }
            res.push_str("\n");
        }
        res
    }

    pub fn add_boat(mut self, boat: Boat, pos: (usize, usize)) -> () {
        match boat {
            Boat::Vertical(size) => {
                for i in 0..size{
                    self.data[pos.0+i][pos.1]= 1;
                }
            }
            Boat::Horizontal(size) => {
                for i in 0..size{
                    self.data[pos.0][pos.1+i]= 1;
                }
            }
        }

        let mut contents = fs::read_to_string("src/board.txt")
            .expect("Should have been able to read the file");

        let parts = contents.split("\n");
        let mut num_boats=String::new();
        for e in parts {
            num_boats.push_str(e);
            num_boats.push_str("\n");
            println!("{}", num_boats);
            break;
        }

        let mut contents = OpenOptions::new()
            .write(true)
            .open("src/board.txt")
            .expect("cannot open file");
        contents.write(num_boats.as_bytes()).expect("Unable to write file");
        contents.write(self.to_string().as_bytes()).expect("Unable to write file");

    }
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


fn main() {
    let arg = Args::parse();
    // let numbers = arg.input.split(" ");
    // for number in numbers {
    //     println!("{}",number);
    // }

    let mut board = Board { boats: [0,0,0,0], data: [[0; BSIZE];BSIZE] };
    let mut numbers= Vec::new();
    let mut argomento="6V";
    let mut pos =(10, 10);
    if arg.input.contains("new"){
        numbers = arg.input.split(" ").collect();
        board = Board::new(numbers[1].as_bytes());
    }

    if arg.input.contains("add"){
        numbers = arg.input.split(" ").collect();
        argomento= numbers[1];
        let mut pos_temp= Vec::new();
        pos_temp= numbers[2].split(",").collect();
        pos= (pos_temp[0].parse().unwrap(), pos_temp[1].parse().unwrap());

        let a= argomento.chars().next().unwrap();
        let b= a.to_digit(10).map(|digit| digit as usize).unwrap();
        let mut boat:Boat;
        boat=Boat::Horizontal(b);
        if argomento.contains("V") {
            boat=Boat::Vertical(b);
        }

        let add = board.add_boat(boat, pos);
    }

}
