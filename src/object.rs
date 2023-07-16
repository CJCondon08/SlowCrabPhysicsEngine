use minifb::Window;

#[derive (Copy, Clone)]
pub struct Object {
    pub x: i16,
    pub y: i16,
    prev: (i16, i16),
    size: i16,
    mass: f32,
    acceleration: (f32, f32),
    rigid: bool
}

impl Object {
    pub fn new_rigid() -> Object {
        Object{x: 0, y: 200, prev: (0, 200), size: 150, mass: 5.0, acceleration: (0.0, 0.0), rigid: true}
    }

    pub fn new_non_rigid() -> Object {
        Object{x: 0, y: 0, prev: (0, 0), size: 150, mass: 0.0, acceleration: (0.0, 0.0), rigid: false}
    }

    pub fn gravity(&mut self, delta_timer: f32) {
        //change to collision detection
        if self.y >= 540 || self.rigid == false{
            self.acceleration = (0.0, 0.0);
            return;
        }

        let g: f32 = -9.8*3.0;
        self.acceleration.1 -= g*delta_timer;  
        self.y += self.acceleration.1.floor() as i16;

        if self.acceleration.0 == 0.0 {
            return;
        }

        let air_resistance:f32 = 5.0;
        self.acceleration.0 += air_resistance*delta_timer;
        self.x -= self.acceleration.0 as i16;

    }

    fn motion(&mut self, delta_timer: f32, greater_pos:i16, lesser_pos:i16) -> f32 {
        let mut force: f32;

        force = (greater_pos - lesser_pos) as f32;
        force *= delta_timer*130.0;
        return (force/self.mass)*1.75;
        
    }

    pub fn collision_effects(&mut self, object2: &mut Object){
        let x_diff = self.x - object2.x;
        let y_diff = self.y - object2.y;
        let separation_dist = (self.size - object2.size) / 2;

        self.x += x_diff/100;
        object2.x -= x_diff/100;
        self.y += y_diff/100;
        object2.y -= y_diff/100;

        let mut pre_momentum = self.acceleration.0*self.mass + object2.acceleration.0*object2.mass;
        let v2_final = self.acceleration.0 - object2.acceleration.0;
        pre_momentum -= v2_final;
        self.acceleration.0 = pre_momentum/(self.mass+object2.mass);
        object2.acceleration.0 = self.acceleration.0 + v2_final;

        drop(pre_momentum);
        drop(v2_final);

        let mut pre_momentum = self.acceleration.1*self.mass + object2.acceleration.1*object2.mass;
        let v2_final = self.acceleration.1 - object2.acceleration.1;
        pre_momentum -= v2_final;
        self.acceleration.1 = pre_momentum/(self.mass+object2.mass);
        object2.acceleration.1 = self.acceleration.1 + v2_final;

    }

    pub fn is_colliding(&mut self, objects_list: &[Object], index: usize) {
        if objects_list.len() < 2 {
            return;
        }
    
        let other_objects = &objects_list[0..];
        let mut i = 0;
        for object in other_objects {

            if i != index {
                let self_left = self.x - self.size / 2;
                let self_right = self.x + self.size / 2;
                let self_top = self.y - self.size / 2;
                let self_bottom = self.y + self.size / 2;
        
                let other_left = object.x - object.size / 2;
                let other_right = object.x + object.size / 2;
                let other_top = object.y - object.size / 2;
                let other_bottom = object.y + object.size / 2;
        
                if self_left <= other_right && self_right >= other_left &&
                   self_top <= other_bottom && self_bottom >= other_top {
                    self.collision_effects(&mut object.clone());
                }
            }

            i += 1;

        }
    }
    
    pub fn boundries(&mut self, window: &mut Window){
        
        if self.x <= 5 {
            self.x = 10;
            self.acceleration.0 = 0.0;

        } else if self.x >= self.size + window.get_size().0 as i16 - 270{
            self.x = self.size + window.get_size().1 as i16 - 10;
            self.acceleration.0 = 0.0;
        }

        if self.y <= 5 {
            self.y = 10;
            self.acceleration.1 = 0.0;

        } else if self.y >= window.get_size().1 as i16 - self.size{
            self.y = window.get_size().1 as i16 - self.size;
            self.acceleration.1 = 0.0;
        }
    }
    pub fn drag(&mut self, window: &mut Window, delta_timer: f32) -> (i16, i16){

        let mouse_x: i16 = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as i16;
        let mouse_y: i16 = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as i16;

        if !window.get_mouse_down(minifb::MouseButton::Left) 
        || mouse_y <= self.y || mouse_y >= self.y + self.size 
        || mouse_x <= self.x || mouse_x >= self.x + self.size {
            return (self.x, self.y);
        }

        drop(mouse_x);
        drop(mouse_y);
        self.prev.0 = self.x;
        self.prev.1 = self.y;

        let mut is_rigid:bool = false;

        if self.rigid {
            self.rigid = false;
            is_rigid = true;
            self.acceleration.1 = 0.0;
            self.acceleration.0 = 0.0;
        }

        self.x = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as i16 - self.size/2;
        self.y = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as i16 - self.size/2;

        if is_rigid {
            self.rigid = true;
        }

        self.acceleration.0 = self.motion(delta_timer, self.prev.0, self.x);
        self.acceleration.1 = self.motion(delta_timer, self.y, self.prev.1);
        return (self.x, self.y);
    }
}