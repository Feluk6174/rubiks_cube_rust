use std::io;
use rand::Rng; 

pub struct Cube {
    edges: [[u8; 4]; 6],
    corners: [[u8; 4]; 6], 
    colors: [char; 6]
}

pub trait Movement<T> {
    fn move_side(&mut self, movement:T);
}

impl Cube {
    pub fn new() -> Self {
        Self {
            edges: [[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4], [5, 5, 5, 5]],
            corners: [[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4], [5, 5, 5, 5]],
            colors: ['w', 'y', 'b', 'o', 'r', 'g']
        }
    }

    pub fn from_state(mut edges_state:u128, mut corners_state:u128) -> Self {
        let mut edges: [[u8; 4]; 6] = [[0; 4]; 6];
        let mut corners: [[u8; 4]; 6] = [[0; 4]; 6];

        for i in 0..6 {
            for j in 0..4 {
                //println!("edje {} {}: {}", 5-i, 3-j, (edges_state & 7) as u8);
                edges[5-i][3-j] = (edges_state & 7) as u8;
                corners[5-i][3-j] = (corners_state & 7) as u8;
                edges_state >>= 3;
                corners_state >>= 3;
            }
        }

        Self {
            edges: edges,
            corners: corners,
            colors: ['w', 'y', 'b', 'o', 'r', 'g']
        }
    }

    pub fn scrambeled(moves:usize) -> Self {
        let mut cube = Self {
            edges: [[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4], [5, 5, 5, 5]],
            corners: [[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4], [5, 5, 5, 5]],
            colors: ['w', 'y', 'b', 'o', 'r', 'g']
        };
        cube.scramble(moves);
        cube
    }

    pub fn load_state(&mut self, mut edges_state:u128, mut corners_state:u128) {
        for i in 0..6 {
            for j in 0..4 {
                //println!("edje {} {}: {}", 5-i, 3-j, (edges_state & 7) as u8);
                self.edges[5-i][3-j] = (edges_state & 7) as u8;
                self.corners[5-i][3-j] = (corners_state & 7) as u8;
                edges_state >>= 3;
                corners_state >>= 3;
            }
        }
    }

    pub fn scramble(&mut self, num:usize) {
        let mut rng = rand::thread_rng(); 
        let mut moves:[&str; 40] = [""; 40];
        let possible = ["f", "f2", "f'", "b", "b2", "b'", "u", "u2", "u'", "d", "d2", "d'", "r", "r2", "r'", "l", "l2", "l'"];
        for i in 0..num { 
            let num: u16 = rng.gen_range(0..18); 
            self.move_side(num);
            moves[i] = possible[num as usize];
        } 
        println!("{:?}", moves)
    }

    fn get_char(&self, side:usize, pices:&[[u8; 4]; 6], index:usize) -> char {
        //return ((side + 48) as u8) as char;
        let index:usize = pices[side][index] as usize;
        self.colors[index]
    }

    pub fn print(&self) {
        println!("{}", self)
    }

    fn turn_face(&mut self, side:usize) {
        self.edges[side] = [self.edges[side][3], self.edges[side][0], self.edges[side][1], self.edges[side][2]];
        self.corners[side] = [self.corners[side][3], self.corners[side][0], self.corners[side][1], self.corners[side][2]];

    }

    fn turn_side(&mut self, side:usize, sides:[usize; 4], side_vals:[[usize; 3]; 4]) {
        self.turn_face(side);
        //Guardes 2 cares en valors temporals per a fer el canvi
        let temp0_corners:[u8; 4] = self.corners[sides[0]];
        let temp0_edges:[u8; 4] = self.edges[sides[0]];
        let temp1_corners:[u8; 4] = self.corners[sides[2]];
        let temp1_edges:[u8; 4] = self.edges[sides[2]];


        self.corners[sides[0]][side_vals[0][0]] = self.corners[sides[3]][side_vals[3][0]];
        self.corners[sides[2]][side_vals[2][0]] = self.corners[sides[1]][side_vals[1][0]];
        self.corners[sides[1]][side_vals[1][0]] = temp0_corners[side_vals[0][0]];
        self.corners[sides[3]][side_vals[3][0]] = temp1_corners[side_vals[2][0]];
        
        self.edges[sides[0]][side_vals[0][1]] = self.edges[sides[3]][side_vals[3][1]];
        self.edges[sides[2]][side_vals[2][1]] = self.edges[sides[1]][side_vals[1][1]];
        self.edges[sides[1]][side_vals[1][1]] = temp0_edges[side_vals[0][1]];
        self.edges[sides[3]][side_vals[3][1]] = temp1_edges[side_vals[2][1]];

        self.corners[sides[0]][side_vals[0][2]] = self.corners[sides[3]][side_vals[3][2]];
        self.corners[sides[2]][side_vals[2][2]] = self.corners[sides[1]][side_vals[1][2]];
        self.corners[sides[1]][side_vals[1][2]] = temp0_corners[side_vals[0][2]];
        self.corners[sides[3]][side_vals[3][2]] = temp1_corners[side_vals[2][2]];
    }

    pub fn f(&mut self) {
        //colors: ['w', 'y', 'b', 'o', 'r', 'g']
		//d = [{"color": "w", "nums": [3, 2, 2]}, {"color": "r", "nums": [0, 3, 3]}, {"color": "y", "nums": [1, 0, 0]}, {"color": "o", "nums": [2, 1, 1]}, ]

        self.turn_side(5, [0, 4, 1, 3], [[3, 2, 2], [0, 3, 3], [1, 0, 0], [2, 1, 1]])
    }

    pub fn b(&mut self) {
        //colors: ['w', 'y', 'b', 'o', 'r', 'g']
		//d[{"color": "w", "nums": [0, 0, 1]}, {"color": "o", "nums": [0, 3, 3]}, {"color": "y", "nums": [3, 2, 2]}, {"color": "r", "nums": [2, 1, 1]}, ]

        self.turn_side(2, [0, 3, 1, 4], [[1, 0, 0], [0, 3, 3], [3, 2, 2], [2, 1, 1]])
    }

    pub fn u(&mut self) {
        //colors: ['w', 'y', 'b', 'o', 'r', 'g']
		//[{"color": "g", "nums": [0, 0, 1]}, {"color": "o", "nums": [0, 0, 1]}, {"color": "b", "nums": [0, 0, 1]}, {"color": "r", "nums": [0, 0, 1]}, ]
        self.turn_side(0, [5, 3, 2, 4], [[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]])
    }

    pub fn d(&mut self) {
        //colors: ['w', 'y', 'b', 'o', 'r', 'g']
		//[{"color": "g", "nums": [3, 2, 2]}, {"color": "r", "nums": [3, 2, 2]}, {"color": "b", "nums": [3, 2, 2]}, {"color": "o", "nums": [3, 2, 2]}, ]
        self.turn_side(1, [2, 3, 5, 4], [[3, 2, 2], [3, 2, 2], [3, 2, 2], [3, 2, 2]])
    }

    pub fn r(&mut self) {
        //colors: ['w', 'y', 'b', 'o', 'r', 'g']
		//[{"color": "w", "nums": [2, 1, 1]}, {"color": "b", "nums": [0, 3, 3]}, {"color": "y", "nums": [2, 1, 1]}, {"color": "g", "nums": [2, 1, 1]}, ]

        self.turn_side(4, [0, 2, 1, 5], [[2, 1, 1], [0, 3, 3], [2, 1, 1], [2, 1, 1]])
    }

    pub fn l(&mut self) {
        //colors: ['w', 'y', 'b', 'o', 'r', 'g']
		//[{"color": "w", "nums": [0, 3, 3]}, {"color": "g", "nums": [0, 3, 3]}, {"color": "y", "nums": [0, 3, 3]}, {"color": "b", "nums": [2, 1, 1]}, ]

        self.turn_side(3, [0, 5, 1, 2], [[0, 3, 3], [0, 3, 3], [0, 3, 3], [2, 1, 1]])
    }

    pub fn is_solved(&self) -> bool {

    }

    pub fn _is_solved(& self) -> bool {
        self.edges == [[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4], [5, 5, 5, 5]] &&
        self.corners == [[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4], [5, 5, 5, 5]] 
    }

    pub fn get_state(&self) -> (u128, u128){
        let mut edges_state:u128 = 0;
        let mut corners_state:u128 = 0;
        
        for side in self.edges {
            for edge in side {
                edges_state <<= 3;
                edges_state |= edge as u128;
            }
        }

        for side in self.corners {
            for edge in side {
                corners_state <<= 3;
                corners_state |= edge as u128;
            }
        }

        (edges_state, corners_state)
    }

    pub fn play(&mut self) -> io::Result<()>{
        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            stdin.read_line(&mut buffer)?;
            match &buffer[0..buffer.len() - 1] {
                "e" => println!("{} {}", self.get_state().0, self.get_state().1),
                "s" => self.scramble(40),
                "q" => break,
                m => self.move_side(m),
            }
            println!("{}", self);
            buffer = String::new();
        }
        Ok(())
    }

}

impl Movement<u16> for Cube {
    fn move_side(&mut self, movement:u16) {
        match movement {
            0 => {self.f()},
            1 => {self.f(); self.f()}
            2 => {self.f(); self.f(); self.f()},
            3 => {self.b()},
            4 => {self.b(); self.b()}
            5 => {self.b(); self.b(); self.b()},
            6 => {self.u()},
            7 => {self.u(); self.u()}
            8 => {self.u(); self.u(); self.u()},
            9 => {self.d()},
            10 => {self.d(); self.d()}
            11 => {self.d(); self.d(); self.d()},
            12 => {self.r()},
            13 => {self.r(); self.r()}
            14 => {self.r(); self.r(); self.r()},
            15 => {self.l()},
            16 => {self.l(); self.l()}
            17 => {self.l(); self.l(); self.l()},
            _ => println!("move not found"),
        }
    }
}

impl Movement<&str> for Cube {
    fn move_side(&mut self, movement:&str) {
        match movement {
            "f" => {self.f()},
            "f2" => {self.f(); self.f()}
            "f'" => {self.f(); self.f(); self.f()},
            "b" => {self.b()},
            "b2" => {self.b(); self.b()}
            "b'" => {self.b(); self.b(); self.b()},
            "u" => {self.u()},
            "u2" => {self.u(); self.u()}
            "u'" => {self.u(); self.u(); self.u()},
            "d" => {self.d()},
            "d2" => {self.d(); self.d()}
            "d'" => {self.d(); self.d(); self.d()},
            "r" => {self.r()},
            "r2" => {self.r(); self.r()}
            "r'" => {self.r(); self.r(); self.r()},
            "l" => {self.l()},
            "l2" => {self.l(); self.l()}
            "l'" => {self.l(); self.l(); self.l()},
            _ => println!("move not found")
        }
    }
}


impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "      {} {} {}
      {} b {}
      {} {} {}
{} {} {} {} {} {} {} {} {} {} {} {}
{} o {} {} w {} {} r {} {} y {}
{} {} {} {} {} {} {} {} {} {} {} {}
      {} {} {}
      {} g {}
      {} {} {}
", 
        self.get_char(2, &self.corners, 2), 
        self.get_char(2, &self.edges, 2), 
        self.get_char(2, &self.corners, 3),
        self.get_char(2, &self.edges, 1),
        self.get_char(2, &self.edges, 3),
        self.get_char(2, &self.corners, 1),
        self.get_char(2, &self.edges, 0), 
        self.get_char(2, &self.corners, 0),
        self.get_char(3, &self.corners, 3),
        self.get_char(3, &self.edges, 3),
        self.get_char(3, &self.corners, 0),
        self.get_char(0, &self.corners, 0),
        self.get_char(0, &self.edges, 0),
        self.get_char(0, &self.corners, 1),
        self.get_char(4, &self.corners, 1),
        self.get_char(4, &self.edges, 1),
        self.get_char(4, &self.corners, 2),
        self.get_char(1, &self.corners, 2),
        self.get_char(1, &self.edges, 2),
        self.get_char(1, &self.corners, 3),
        self.get_char(3, &self.edges, 2),
        self.get_char(3, &self.edges, 0),
        self.get_char(0, &self.edges, 3),
        self.get_char(0, &self.edges, 1),
        self.get_char(4, &self.edges, 0),
        self.get_char(4, &self.edges, 2),
        self.get_char(1, &self.edges, 1),
        self.get_char(1, &self.edges, 3),
        self.get_char(3, &self.corners, 2),
        self.get_char(3, &self.edges, 1),
        self.get_char(3, &self.corners, 1),
        self.get_char(0, &self.corners, 3),
        self.get_char(0, &self.edges, 2),
        self.get_char(0, &self.corners, 2),
        self.get_char(4, &self.corners, 0),
        self.get_char(4, &self.edges, 3),
        self.get_char(4, &self.corners, 3),
        self.get_char(1, &self.corners, 1),
        self.get_char(1, &self.edges, 0),
        self.get_char(1, &self.corners, 0),
        self.get_char(5, &self.corners, 0), 
        self.get_char(5, &self.edges, 0), 
        self.get_char(5, &self.corners, 1),
        self.get_char(5, &self.edges, 3), 
        self.get_char(5, &self.edges, 1),
        self.get_char(5, &self.corners, 3), 
        self.get_char(5, &self.edges, 2), 
        self.get_char(5, &self.corners, 2)
        )
    }
}

fn main() {
    let mut cube:Cube = Cube::new();
    cube.print();
    println!("{}", cube.is_solved())

}
