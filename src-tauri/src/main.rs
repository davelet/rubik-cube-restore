// use rubiks::{
//     cube::Cube,
//     shuffler::CubeShuffler,
//     solver::{execute, SolveTarget},
//     utils::print_cube,
// };

// mod rubiks;

// fn main() {
//     let mut cube = Cube::new();
//     let mut shuffler = CubeShuffler::new(&mut cube);
//     shuffler.shuffle(20);
//     print_cube(&cube);
//     let res = execute(&mut cube, SolveTarget::MiddleEdge);
//     println!("after {:?}", res);
// }
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    rubik_cube_restore_lib::run()
}
