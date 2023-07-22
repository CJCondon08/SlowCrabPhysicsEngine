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
        Object{x: 0, y: 0, prev: (0, 0), size: 150, mass: 5.0, acceleration: (0.0, 0.0), rigid: true}
    }

    pub fn new_non_rigid() -> Object {
        Object{x: 0, y: 0, prev: (0, 0), size: 150, mass: 0.0, acceleration: (0.0, 0.0), rigid: false}
    }

    pub fn acceleration_controll(&mut self, delta_timer: f32) {

        if self.rigid == false {
            return;
        }
        
        //gravity

        if self.y >= 700 - self.size {
            self.acceleration.1 *= -0.5;
        }

        let g: f32 = -9.8*3.0;
        self.acceleration.1 -= g*delta_timer;  
        self.y += self.acceleration.1.floor() as i16;

        //friction

        let mut friction:f32 = 5.0;

        if self.acceleration.0 > 0.0 {
            friction = -5.0;

            if self.acceleration.0 <= 0.1 {
                self.acceleration.0 = 0.0;
            }
        } else if self.acceleration.0 <= 0.0{
            friction = 5.0;
            if self.acceleration.0 >= -0.1 {
                self.acceleration.0 = 0.0;
                
            }
        }

        if self.acceleration.0 == 0.0 {
            return;
        }

        if self.y >= 700 - self.size {
            friction *= self.mass*0.6;
        }

        self.acceleration.0 += friction*delta_timer;
        self.x -= self.acceleration.0.floor() as i16;

    }

    fn motion(&mut self, delta_timer: f32, greater_pos:i16, lesser_pos:i16) -> f32 {
        let mut force: f32;

        force = (greater_pos - lesser_pos) as f32;
        force *= delta_timer*130.0;
        return (force/self.mass)*1.75;
        
    }

    pub fn collision_effects(&mut self, object2: &mut Object){

        let mut pre_momentum = self.acceleration.0*self.mass + object2.acceleration.0*object2.mass;
        let v2_final = self.acceleration.0 - object2.acceleration.0;
        pre_momentum -= v2_final*object2.mass;
        self.acceleration.0 = pre_momentum/(self.mass+object2.mass);
        object2.acceleration.0 = self.acceleration.0 + v2_final;

        drop(pre_momentum);
        drop(v2_final);

        let mut pre_momentum = self.acceleration.1*self.mass + object2.acceleration.1*object2.mass;
        let v2_final = self.acceleration.1 - object2.acceleration.1;
        pre_momentum -= v2_final*object2.mass;
        self.acceleration.1 = pre_momentum/(self.mass+object2.mass);
        object2.acceleration.1 = self.acceleration.1 + v2_final;

    }

    pub fn prevent_overlap(&mut self, object: &mut Object){

        if self.y+self.size >= object.y 
            && object.y >= 700 - object.size && 
            (self.x + self.size >= object.x || self.x <= object.x + object.size) && 
            self.y < object.y - (object.size/3){
                self.acceleration.1 *= 0.4;
                self.y = (700 - object.size) - self.size; 
            
        }
    }

    pub fn is_colliding(&mut self, objects_list: &mut [Object], index: usize) {
        if objects_list.len() < 2 {
            return;
        }

        for i in 0..objects_list.len() {
            let mut object = objects_list[i]; 
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

                    self.prevent_overlap(&mut object);
                    self.collision_effects(&mut object);
                    objects_list[i] = object;
                }
            } 
        }
    }
    
    pub fn boundries(&mut self, window: &mut Window){
        
        if self.x <= 0 {
            self.x = 0;
            self.acceleration.0 *= -0.2;

        } else if self.x >= self.size + window.get_size().0 as i16 - 300{
            self.x = self.size + window.get_size().0 as i16 - 300;
            self.acceleration.0 *= -0.2;
        }

        if self.y <= 5 {
            self.y = 6;
            self.acceleration.1 = 0.0;

        } else if self.y >= window.get_size().1 as i16 - self.size{
            self.y = window.get_size().1 as i16 - self.size;
            self.acceleration.1 *= 0.4;
        }
    }

    pub fn drag(&mut self, window: &mut Window, delta_timer: f32, index: i16) -> i16{

        let mouse_x: i16 = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as i16;
        let mouse_y: i16 = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as i16;

        if !window.get_mouse_down(minifb::MouseButton::Left) 
        || mouse_y <= self.y || mouse_y >= self.y + self.size 
        || mouse_x <= self.x || mouse_x >= self.x + self.size {
            return -1;
        }

        if window.get_mouse_down(minifb::MouseButton::Right) {
            self.rotate(true);
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
        return index;
    }

    fn rotate(&mut self, is_drag: bool){
        return;
    }
}