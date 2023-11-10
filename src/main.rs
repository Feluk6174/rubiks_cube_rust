mod cube;
mod solve;
use cube::{Cube, Movement};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn solve_part(min:u128, max:u128, state:&(u128, u128), depth:&u32, tx:Sender<bool>) {
    let mut cube:Cube = Cube::from_state(state.0, state.1);
    for i in min..max {
        let moves = solve::gen_moves(i, *depth);
        let solved = solve::execute_moves(&mut cube, moves, state.0, state.1);
        if solved {
            println!("Solved {} {}", i, depth);
            let _ = tx.send(true);
        }
    }
    let _ = tx.send(false);
}

fn solve_by_parts(depth:u32, state:(u128, u128)) -> bool {
    let num = solve::calc_iter(depth) / 4;
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    for i in 0..4 {
        let min = num*i;
        let max = num*(i+1);
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            solve_part(min, max, &state, &depth, tx);
        }))
    }
    for thread in threads {
        let _ = thread.join();
        let res = rx.recv().unwrap();
        if res {
            return true
        }
    }
    false
}

fn main() {
    let cube = Cube::scrambeled(20);
    for depth in 0..20 {
        println!("depth: {}", depth);
        if solve_by_parts(depth, cube.get_state()) {
            break
        }
    }
}
