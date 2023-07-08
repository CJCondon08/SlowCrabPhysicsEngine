use minifb::Window;

pub struct Object {
    pub x: usize,
    pub y: usize,
    pub size: usize,
    pub mass: f32,
    pub acceleration: f32,
    pub rigid: bool,
    pub fall_time: f32
}

impl Object {
    pub fn new_rigid() -> Object {
        Object{x: 0, y: 0, size: 150, mass: 10.0, acceleration: 0.0, rigid: true, fall_time: 0.0}
    }

    pub fn new_non_rigid() -> Object {
        Object{x: 0, y: 0, size: 150, mass: 0.0, acceleration: 0.0, rigid: false, fall_time: 0.0}
    }

    pub fn gravity(&mut self, delta_timer: f32) -> usize{

        //change to collision detection
        if self.y >= 540 || !self.rigid{
            self.acceleration = 0.0;
            self.fall_time = 0.0;
            return self.y;
        }
        self.fall_time += delta_timer;

        let g: f32 = 9.8*3.0;
        self.acceleration = g*self.fall_time;
        self.y += self.acceleration as usize;
            
        self.y
    }

    pub fn drag(&mut self, window: &mut Window) -> (usize, usize){

        let mouse_x = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as usize;
        let mouse_y = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as usize;

        if !window.get_mouse_down(minifb::MouseButton::Left) 
        || mouse_y <= self.y || mouse_y >= self.y + self.size 
        || mouse_x <= self.x || mouse_x >= self.x + self.size {
            return (self.x, self.y);
        }

        drop(mouse_x);
        drop(mouse_y);

        let mut is_rigid:bool = false;

        if self.rigid {
            self.rigid = false;
            is_rigid = true;
            self.acceleration = 0.0;
            self.fall_time = 0.0;
        }

        self.x = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as usize - self.size/2;
        self.y = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as usize - self.size/2;

        if is_rigid {
            self.rigid = true;
        }

        return (self.x, self.y);
    }
}