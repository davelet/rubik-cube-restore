use rand::{rngs::ThreadRng, Rng};

use super::cube::{
    color::Color,
    face::{FaceOrientation, TwistDirection},
    Cube,
};

pub struct CubeScrambler<'a> {
    cube: &'a mut Cube,
}

impl<'a> CubeScrambler<'a> {
    pub fn new(cube: &'a mut Cube) -> Self {
        CubeScrambler { cube }
    }

    pub fn scramble(&mut self, face: FaceOrientation, direction: TwistDirection) {
        // 保存当前面的状态
        let mut current_face = [[Color::White; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                current_face[i][j] = self.cube.get_block_color(face.ordinal(), i, j);
            }
        }

        // 旋转当前面
        for i in 0..3 {
            for j in 0..3 {
                if direction == TwistDirection::Clockwise {
                    self.cube
                        .set_block_color(face.ordinal(), i, j, current_face[2 - j][i]);
                } else {
                    self.cube
                        .set_block_color(face.ordinal(), i, j, current_face[j][2 - i]);
                }
            }
        }

        // 旋转相邻的边
        self.rotate_adjacent_edges(face, direction);
    }

    fn rotate_adjacent_edges(&mut self, face: FaceOrientation, direction: TwistDirection) {
        let mut temp = [Color::White; 3];

        match face {
            FaceOrientation::Up(_) => {
                // 保存前面的边
                for i in 0..3 {
                    temp[i] = self
                        .cube
                        .get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 0, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 前 -> 右 -> 后 -> 左 -> 前
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 0, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Right(Color::Red).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 0, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 0, i),
                        );
                        self.cube
                            .set_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 0, i, temp[i]);
                    }
                } else {
                    // 前 <- 右 <- 后 <- 左 <- 前
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 0, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Left(Color::Orange).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 0, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 0, i),
                        );
                        self.cube
                            .set_block_color(FaceOrientation::Right(Color::Red).ordinal(), 0, i, temp[i]);
                    }
                }
            }
            FaceOrientation::Down(_) => {
                // 保存前面的边
                for i in 0..3 {
                    temp[i] = self
                        .cube
                        .get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 2, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 前 <- 左 <- 后 <- 右 <- 前
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Left(Color::Orange).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 2, i),
                        );
                        self.cube
                            .set_block_color(FaceOrientation::Right(Color::Red).ordinal(), 2, i, temp[i]);
                    }
                } else {
                    // 前 -> 左 -> 后 -> 右 -> 前
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 2, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Right(Color::Red).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2, i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2, i),
                        );
                        self.cube
                            .set_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2, i, temp[i]);
                    }
                }
            }
            FaceOrientation::Front(_) => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self
                        .cube
                        .get_block_color(FaceOrientation::Up(Color::White).ordinal(), 2, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 -> 右 -> 下 -> 左 -> 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2 - i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Left(Color::Orange).ordinal(),
                            2 - i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), 0, 2 - i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            0,
                            2 - i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), i, 0),
                        );
                        self.cube
                            .set_block_color(FaceOrientation::Right(Color::Red).ordinal(), i, 0, temp[i]);
                    }
                } else {
                    // 上 <- 右 <- 下 <- 左 <- 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            2,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Right(Color::Red).ordinal(),
                            i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), 0, 2 - i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            0,
                            2 - i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2 - i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Left(Color::Orange).ordinal(),
                            2 - i,
                            2,
                            temp[i],
                        );
                    }
                }
            }
            FaceOrientation::Back(_) => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self
                        .cube
                        .get_block_color(FaceOrientation::Up(Color::White).ordinal(), 0, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 <- 右 <- 下 <- 左 <- 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Right(Color::Red).ordinal(),
                            i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), 2, 2 - i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            2,
                            2 - i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2 - i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Left(Color::Orange).ordinal(),
                            2 - i,
                            0,
                            temp[i],
                        );
                    }
                } else {
                    // 上 -> 右 -> 下 -> 左 -> 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            0,
                            i,
                            self.cube
                                .get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2 - i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Left(Color::Orange).ordinal(),
                            2 - i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), 2, 2 - i),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            2,
                            2 - i,
                            self.cube
                                .get_block_color(FaceOrientation::Right(Color::Red).ordinal(), i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Right(Color::Red).ordinal(),
                            i,
                            2,
                            temp[i],
                        );
                    }
                }
            }
            FaceOrientation::Left(_) => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self
                        .cube
                        .get_block_color(FaceOrientation::Up(Color::White).ordinal(), i, 0);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 -> 前 -> 下 -> 后 -> 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2 - i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            2 - i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            i,
                            0,
                            temp[i],
                        );
                    }
                } else {
                    // 上 <- 前 <- 下 <- 后 <- 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2 - i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            2 - i,
                            2,
                            temp[i],
                        );
                    }
                }
            }
            FaceOrientation::Right(_) => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self
                        .cube
                        .get_block_color(FaceOrientation::Up(Color::White).ordinal(), i, 2);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 <- 前 <- 下 <- 后 <- 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2 - i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            2 - i,
                            0,
                            temp[i],
                        );
                    }
                } else {
                    // 上 -> 前 -> 下 -> 后 -> 上
                    for i in 0..3 {
                        self.cube.set_block_color(
                            FaceOrientation::Up(Color::White).ordinal(),
                            i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2 - i, 0),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Back(Color::Green).ordinal(),
                            2 - i,
                            0,
                            self.cube
                                .get_block_color(FaceOrientation::Down(Color::Yellow).ordinal(), i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Down(Color::Yellow).ordinal(),
                            i,
                            2,
                            self.cube
                                .get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), i, 2),
                        );
                        self.cube.set_block_color(
                            FaceOrientation::Front(Color::Blue).ordinal(),
                            i,
                            2,
                            temp[i],
                        );
                    }
                }
            }
        }
    }
}

pub struct CubeShuffler<'a> {
    scrambler: CubeScrambler<'a>,
    rng: Option<ThreadRng>,
}

impl<'a> CubeShuffler<'a> {
    pub fn new(cube: &'a mut Cube) -> Self {
        CubeShuffler {
            scrambler: CubeScrambler::new(cube),
            rng: Some(rand::thread_rng()),
        }
    }

    pub fn shuffle(&mut self, times: u32) {
        for _ in 0..times {
            // 随机选择一个面
            let face = self.random_face();
            // 随机选择旋转方向
            let twist_direction = self.random_twist_direction();

            self.scrambler.scramble(face, twist_direction);
        }

    }

    fn random_face(&mut self) -> FaceOrientation {
        let random_index = self.rng.as_mut().unwrap().gen_range(0..6);

        FaceOrientation::from_u8(random_index as u8)
    }

    fn random_twist_direction(&mut self) -> TwistDirection {
        let clockwise = self.rng.as_mut().unwrap().gen_bool(0.5);
        if clockwise {
            TwistDirection::Clockwise
        } else {
            TwistDirection::CounterClockwise
        }
    }
}
