use crate::rubiks::{
    cube::{
        color::Color,
        face::{FaceOrientation, TwistDirection},
        Cube,
    },
    shuffler::CubeScrambler,
    utils::print_cube,
};

pub fn rotate_and_record(
    cube: &mut Cube,
    face: FaceOrientation,
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
        FaceOrientation::Front(_) => {
            if clockwize_direction {
                'f'
            } else {
                'F'
            }
        }
        FaceOrientation::Back(_) => {
            if clockwize_direction {
                'b'
            } else {
                'B'
            }
        }
        FaceOrientation::Left(_) => {
            if clockwize_direction {
                'l'
            } else {
                'L'
            }
        }
        FaceOrientation::Right(_) => {
            if clockwize_direction {
                'r'
            } else {
                'R'
            }
        }
        FaceOrientation::Up(_) => {
            if clockwize_direction {
                'u'
            } else {
                'U'
            }
        }
        FaceOrientation::Down(_) => {
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

pub fn get_up_center(face: FaceOrientation) -> (usize, usize) {
    match face {
        FaceOrientation::Front(_) => (2, 1),
        FaceOrientation::Right(_) => (1, 2),
        FaceOrientation::Back(_) => (0, 1),
        FaceOrientation::Left(_) => (1, 0),
        _ => panic!("Invalid face orientation for get_up_center: {:?}", face),
    }
}

pub fn get_right_side(face: FaceOrientation) -> FaceOrientation {
    match face {
        FaceOrientation::Front(_) => FaceOrientation::Right(Color::Red),
        FaceOrientation::Right(_) => FaceOrientation::Back(Color::Green),
        FaceOrientation::Back(_) => FaceOrientation::Left(Color::Orange),
        FaceOrientation::Left(_) => FaceOrientation::Front(Color::Blue),
        _ => panic!("Invalid face orientation for get_right_side: {:?}", face),
    }
}

pub fn get_left_side(face: FaceOrientation) -> FaceOrientation {
    match face {
        FaceOrientation::Front(_) => FaceOrientation::Left(Color::Orange),
        FaceOrientation::Left(_) => FaceOrientation::Back(Color::Green),
        FaceOrientation::Back(_) => FaceOrientation::Right(Color::Red),
        FaceOrientation::Right(_) => FaceOrientation::Front(Color::Blue),
        _ => panic!("Invalid face orientation for get_left_side: {:?}", face),
    }
}
