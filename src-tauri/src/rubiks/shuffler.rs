use rand::Rng;

use super::cube::{
    color::Color,
    face::{CubeFace, FaceOrientation, TwistDirection},
    Cube,
};

pub struct CubeShuffler<'a> {
    cube: &'a mut Cube,
    rng: rand::rngs::ThreadRng,
}

impl<'a> CubeShuffler<'a> {
    pub fn new(cube: &'a mut Cube) -> Self {
        CubeShuffler {
            cube,
            rng: rand::thread_rng(),
        }
    }

    pub fn shuffle(&mut self, times: u32) -> &Cube {

        for _ in 0..times {
            // 随机选择一个面
            let face: CubeFace = CubeFace::new(self.random());
            // 随机选择旋转方向
            let clockwise = self.rng.gen_bool(0.5);
            let twist_direction = if clockwise {
                TwistDirection::Clockwise
            } else {
                TwistDirection::CounterClockwise
            };

            self.rotate_face(&face, twist_direction);
        }

        &self.cube
    }

    fn random(&mut self) -> FaceOrientation {
        let random_index = self.rng.gen_range(0..6);

        match random_index {
            0 => FaceOrientation::Up,
            1 => FaceOrientation::Down,
            2 => FaceOrientation::Front,
            3 => FaceOrientation::Back,
            4 => FaceOrientation::Left,
            5 => FaceOrientation::Right,
            _ => panic!(),
        }
    }

    pub fn rotate_face(&mut self, face: &CubeFace, direction: TwistDirection) {
        // 保存当前面的状态
        let mut current_face = [[Color::White; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                current_face[i][j] = self.cube.get_color(face.orientation as usize, i, j);
            }
        }

        // 旋转当前面
        for i in 0..3 {
            for j in 0..3 {
                if direction == TwistDirection::Clockwise {
                    self.cube.set_color(face.orientation as usize, i, j, current_face[2 - j][i]);
                } else {
                    self.cube.set_color(face.orientation as usize, i, j, current_face[j][2 - i]);
                }
            }
        }

        // 旋转相邻的边
        self.rotate_adjacent_edges(face, direction);
    }

    fn rotate_adjacent_edges(&mut self, face: &CubeFace, direction: TwistDirection) {
        let mut temp = [Color::White; 3];

        match face.orientation {
            FaceOrientation::Up => {
                // 保存前面的边
                for i in 0..3 {
                    temp[i] = self.cube.get_color(FaceOrientation::Front as usize, 0, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 前 -> 右 -> 后 -> 左 -> 前
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Front as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Right as usize, 0, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Right as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Back as usize, 0, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Back as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Left as usize, 0, i),
                        );
                        self.cube.set_color(FaceOrientation::Left as usize, 0, i, temp[i]);
                    }
                } else {
                    // 前 <- 右 <- 后 <- 左 <- 前
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Front as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Left as usize, 0, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Left as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Back as usize, 0, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Back as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Right as usize, 0, i),
                        );
                        self.cube.set_color(FaceOrientation::Right as usize, 0, i, temp[i]);
                    }
                }
            }
            FaceOrientation::Down => {
                // 保存前面的边
                for i in 0..3 {
                    temp[i] = self.cube.get_color(FaceOrientation::Front as usize, 2, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 前 <- 左 <- 后 <- 右 <- 前
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Front as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Left as usize, 2, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Left as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Back as usize, 2, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Back as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Right as usize, 2, i),
                        );
                        self.cube.set_color(FaceOrientation::Right as usize, 2, i, temp[i]);
                    }
                } else {
                    // 前 -> 左 -> 后 -> 右 -> 前
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Front as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Right as usize, 2, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Right as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Back as usize, 2, i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Back as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Left as usize, 2, i),
                        );
                        self.cube.set_color(FaceOrientation::Left as usize, 2, i, temp[i]);
                    }
                }
            }
            FaceOrientation::Front => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self.cube.get_color(FaceOrientation::Up as usize, 2, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 -> 右 -> 下 -> 左 -> 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Left as usize, 2 - i, 2),
                        );
                        self.cube.set_color(
                            FaceOrientation::Left as usize,
                            2 - i,
                            2,
                            self.cube.get_color(FaceOrientation::Down as usize, 0, 2 - i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            0,
                            2 - i,
                            self.cube.get_color(FaceOrientation::Right as usize, i, 0),
                        );
                        self.cube.set_color(FaceOrientation::Right as usize, i, 0, temp[i]);
                    }
                } else {
                    // 上 <- 右 <- 下 <- 左 <- 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            2,
                            i,
                            self.cube.get_color(FaceOrientation::Right as usize, i, 0),
                        );
                        self.cube.set_color(
                            FaceOrientation::Right as usize,
                            i,
                            0,
                            self.cube.get_color(FaceOrientation::Down as usize, 0, 2 - i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            0,
                            2 - i,
                            self.cube.get_color(FaceOrientation::Left as usize, 2 - i, 2),
                        );
                        self.cube.set_color(FaceOrientation::Left as usize, 2 - i, 2, temp[i]);
                    }
                }
            }
            FaceOrientation::Back => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self.cube.get_color(FaceOrientation::Up as usize, 0, i);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 <- 右 <- 下 <- 左 <- 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Right as usize, i, 2),
                        );
                        self.cube.set_color(
                            FaceOrientation::Right as usize,
                            i,
                            2,
                            self.cube.get_color(FaceOrientation::Down as usize, 2, 2 - i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            2,
                            2 - i,
                            self.cube.get_color(FaceOrientation::Left as usize, 2 - i, 0),
                        );
                        self.cube.set_color(FaceOrientation::Left as usize, 2 - i, 0, temp[i]);
                    }
                } else {
                    // 上 -> 右 -> 下 -> 左 -> 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            0,
                            i,
                            self.cube.get_color(FaceOrientation::Left as usize, 2 - i, 0),
                        );
                        self.cube.set_color(
                            FaceOrientation::Left as usize,
                            2 - i,
                            0,
                            self.cube.get_color(FaceOrientation::Down as usize, 2, 2 - i),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            2,
                            2 - i,
                            self.cube.get_color(FaceOrientation::Right as usize, i, 2),
                        );
                        self.cube.set_color(FaceOrientation::Right as usize, i, 2, temp[i]);
                    }
                }
            }
            FaceOrientation::Left => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self.cube.get_color(FaceOrientation::Up as usize, i, 0);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 -> 前 -> 下 -> 后 -> 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            i,
                            0,
                            self.cube.get_color(FaceOrientation::Back as usize, 2 - i, 2),
                        );
                        self.cube.set_color(
                            FaceOrientation::Back as usize,
                            2 - i,
                            2,
                            self.cube.get_color(FaceOrientation::Down as usize, i, 0),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            i,
                            0,
                            self.cube.get_color(FaceOrientation::Front as usize, i, 0),
                        );
                        self.cube.set_color(FaceOrientation::Front as usize, i, 0, temp[i]);
                    }
                } else {
                    // 上 <- 前 <- 下 <- 后 <- 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            i,
                            0,
                            self.cube.get_color(FaceOrientation::Front as usize, i, 0),
                        );
                        self.cube.set_color(
                            FaceOrientation::Front as usize,
                            i,
                            0,
                            self.cube.get_color(FaceOrientation::Down as usize, i, 0),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            i,
                            0,
                            self.cube.get_color(FaceOrientation::Back as usize, 2 - i, 2),
                        );
                        self.cube.set_color(FaceOrientation::Back as usize, 2 - i, 2, temp[i]);
                    }
                }
            }
            FaceOrientation::Right => {
                // 保存上面的边
                for i in 0..3 {
                    temp[i] = self.cube.get_color(FaceOrientation::Up as usize, i, 2);
                }

                if direction == TwistDirection::Clockwise {
                    // 上 <- 前 <- 下 <- 后 <- 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            i,
                            2,
                            self.cube.get_color(FaceOrientation::Front as usize, i, 2),
                        );
                        self.cube.set_color(
                            FaceOrientation::Front as usize,
                            i,
                            2,
                            self.cube.get_color(FaceOrientation::Down as usize, i, 2),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            i,
                            2,
                            self.cube.get_color(FaceOrientation::Back as usize, 2 - i, 0),
                        );
                        self.cube.set_color(FaceOrientation::Back as usize, 2 - i, 0, temp[i]);
                    }
                } else {
                    // 上 -> 前 -> 下 -> 后 -> 上
                    for i in 0..3 {
                        self.cube.set_color(
                            FaceOrientation::Up as usize,
                            i,
                            2,
                            self.cube.get_color(FaceOrientation::Back as usize, 2 - i, 0),
                        );
                        self.cube.set_color(
                            FaceOrientation::Back as usize,
                            2 - i,
                            0,
                            self.cube.get_color(FaceOrientation::Down as usize, i, 2),
                        );
                        self.cube.set_color(
                            FaceOrientation::Down as usize,
                            i,
                            2,
                            self.cube.get_color(FaceOrientation::Front as usize, i, 2),
                        );
                        self.cube.set_color(FaceOrientation::Front as usize, i, 2, temp[i]);
                    }
                }
            }
        }
    }
}
