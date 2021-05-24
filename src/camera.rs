use super::*;

pub enum Direction {
    Forward,
    Backward,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Camera {
    pub velocity: f32,
    pub sensitivity: f32,
    pub camera_pos: glm::Vec3,
    pub camera_front: glm::Vec3,

    pub pitch: f32,
    pub yaw: f32,
}

#[allow(unused)]
impl Camera {
    pub fn new_centered(velocity: f32, sensitivity: f32) -> Camera {
        Camera {
            velocity,
            sensitivity,
            camera_pos: glm::vec3(0f32, 0., 0.),
            camera_front: glm::vec3(0f32, 0., -1.),
            pitch: 0.,
            yaw: -90.,
        }
    }

    pub fn new(
        velocity: f32,
        sensitivity: f32,
        camera_pos: glm::Vec3,
        pitch: f32,
        yaw: f32,
    ) -> Camera {
        Camera {
            velocity,
            sensitivity,
            camera_pos,
            camera_front: glm::vec3(0., 0., -1.),
            pitch,
            yaw,
        }
    }
}

#[allow(unused)]
impl Camera {
    pub fn turn(&mut self, delta: (f32, f32)) {
        self.yaw += self.sensitivity * delta.0;
        self.pitch -= self.sensitivity * delta.1;

        if self.pitch > 89. {
            self.pitch = 89.;
        }
        if self.pitch < -89. {
            self.pitch = -89.;
        }

        let mut direction = glm::vec3(0f32, 0., 0.);
        direction.x = degree_to_radian(self.yaw).cos() * degree_to_radian(self.pitch).cos();
        direction.y = degree_to_radian(self.pitch).sin();
        direction.z = degree_to_radian(self.yaw).sin() * degree_to_radian(self.pitch).cos();
        self.camera_front = glm::normalize(&direction);
    }

    pub fn move_front(&mut self) {
        self.move_to(Direction::Forward);
    }

    pub fn move_back(&mut self) {
        self.move_to(Direction::Backward);
    }

    pub fn move_down(&mut self) {
        self.move_to(Direction::Down);
    }

    pub fn move_up(&mut self) {
        self.move_to(Direction::Up);
    }

    pub fn move_right(&mut self) {
        self.move_to(Direction::Right);
    }

    pub fn move_left(&mut self) {
        self.move_to(Direction::Left);
    }

    fn move_to(&mut self, direction: Direction) {
        let mut y_at_start = self.camera_pos.y;
        match direction {
            Direction::Forward => self.camera_pos += self.velocity * self.camera_front,
            Direction::Backward => self.camera_pos -= self.velocity * self.camera_front,
            Direction::Down => y_at_start -= 0.1,
            Direction::Up => y_at_start += 0.1,
            Direction::Right => {
                self.camera_pos +=
                    glm::normalize(&glm::cross(&self.camera_front, &glm::vec3(0f32, 1., 0.)))
                        * self.velocity
            }
            Direction::Left => {
                self.camera_pos -=
                    glm::normalize(&glm::cross(&self.camera_front, &glm::vec3(0f32, 1., 0.)))
                        * self.velocity
            }
        }
        self.camera_pos.y = y_at_start;
    }
}
