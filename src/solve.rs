use crate::cube::Cube;
use crate::cube::Movement;

pub fn gen_moves(mut num: u128, depth: u32) -> [u16; 20] {
    let size:usize = depth as usize + 1;
    let mut moves:[u16; 20] = [32; 20];
    moves [0] = (num % 18) as u16;
    num /= 18;
    for i in 1..size-1 {
        moves[i] = (num % 15) as u16;
        num /= 15;
    }
    //moves[size-1] = (num % 16) as u16;
    moves
}


pub fn calc_iter(depth: u32) -> u128 {
    let mut num:u128 = 18;
    if depth == 0 {return 18}
    for _ in 0..depth-1 {
        num *= 15
    }
    num
}

pub fn execute_moves(cube:&mut Cube, moves:[u16; 20], edges_state:u128, corners_state:u128) -> bool {
    cube.load_state(edges_state, corners_state);
    //println!("{}", cube);
    let mut mov: u16 = moves[0];
    let mut last_move:u16 = 0;
    for i in 0..20 {
        if moves[i] == 32 {break;}
        if i != 0 {
            mov = calc_next(last_move, moves[i]);
        }
        last_move = mov;
        cube.move_side(mov);
    }
    cube.is_solved()
}

fn calc_next(last_mov: u16, current_mov:u16) -> u16 {
    (((last_mov / 3) * 3  + 3) % 18 + current_mov) % 18
}

pub fn solve(edges_state:u128, corners_state:u128) -> (bool, u128, u32)  {
    let mut cube = Cube::new();
    for depth in 1..20 {
        println!("depth: {}", depth);
        for move_int in 0..calc_iter(depth) {
            if execute_moves(&mut cube, gen_moves(move_int, depth), edges_state, corners_state) {
                return (true, move_int, depth)
            }
        }
    }
    (false, 0, 0)
}
fn main() {
    let mut cube = Cube::new();
    cube.scramble(7);
    println!("{}", cube);
    let state = cube.get_state();
    println!("{:?}", solve(state.0, state.1));

    // 12 6  14 8
    // 6  12 8  14
    //println!("6 {} {} {}", calc_next(6, 3), calc_next(12, 11), calc_next(8, 5));

    //println!("{:?}", execute_moves(&mut cube, [6, 3, 11, 5, 32, 32, 32,32,32,32,32,32,32,32,32,32,32,32,32,32], state.0, state.1));

    //let mut cube = Cube::new();
    //println!("{} {}", cube.is_solved(), Cube::from_state(cube.get_state().0, cube.get_state().1).is_solved())
}