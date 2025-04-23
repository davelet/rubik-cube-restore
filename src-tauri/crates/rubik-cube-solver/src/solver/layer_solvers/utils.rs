use rubik_cube_core::cube::{
    face::{Face, TwistDirection},
    Cube,
};
use rubik_cube_shuffler::CubeScrambler;

pub fn rotate_and_record(
    cube: &mut Cube,
    face: Face,
    clockwize_direction: bool,
    steps: &mut Vec<char>,
) {
    let mut shuffler = CubeScrambler::new(cube);
    shuffler.scramble(
        face,
        if clockwize_direction {
            TwistDirection::Clockwise
        } else {
            TwistDirection::CounterClockwise
        },
    );

    let step = match face {
        Face::Front => {
            if clockwize_direction {
                'f'
            } else {
                'F'
            }
        }
        Face::Back => {
            if clockwize_direction {
                'b'
            } else {
                'B'
            }
        }
        Face::Left => {
            if clockwize_direction {
                'l'
            } else {
                'L'
            }
        }
        Face::Right => {
            if clockwize_direction {
                'r'
            } else {
                'R'
            }
        }
        Face::Up => {
            if clockwize_direction {
                'u'
            } else {
                'U'
            }
        }
        Face::Down => {
            if clockwize_direction {
                'd'
            } else {
                'D'
            }
        }
    };
    steps.push(step);

    // println!("\nrotate_and_record: {:?}", step);
    // print_cube(cube);
}

pub fn get_up_center(face: Face) -> (usize, usize) {
    match face {
        Face::Front => (2, 1),
        Face::Right => (1, 2),
        Face::Back => (0, 1),
        Face::Left => (1, 0),
        _ => panic!("Invalid face for get_up_center: {:?}", face),
    }
}

pub fn get_right_side(face: Face) -> Face {
    match face {
        Face::Front => Face::Right,
        Face::Right => Face::Back,
        Face::Back => Face::Left,
        Face::Left => Face::Front,
        _ => panic!("Invalid face orientation for get_right_side: {:?}", face),
    }
}

pub fn get_left_side(face: Face) -> Face {
    match face {
        Face::Front => Face::Left,
        Face::Left => Face::Back,
        Face::Back => Face::Right,
        Face::Right => Face::Front,
        _ => panic!("Invalid face orientation for get_left_side: {:?}", face),
    }
}
