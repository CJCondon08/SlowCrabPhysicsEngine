use minifb::{Key, Window};

#[derive (Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16
}

#[derive (Clone)]
pub struct Object {
    //vertecies stored from starting top left moving clockwise
    pub vertex: Vec<Point>,
    //tracks top-left corrner when theta == 0.0
    pub prev: Point,
    pub size: i16,
    pub theta: f32,
    mass: f32,
    acceleration: (f32, f32),
    rigid: bool
}

impl Object {
    pub fn new_rigid() -> Object {
        Object{vertex: vec![Point{x: 200, y: 200}, Point{x: 350, y: 200}, Point{x: 350, y: 350}, Point{x: 200, y: 350}], prev: Point{x: 200, y: 200}, size: 150, theta: 0.0, mass: 5.0, acceleration: (0.0, 0.0), rigid: true}
    }

    pub fn acceleration_controll(&mut self, delta_timer: f32) {

        if self.rigid == false {
            return;
        }
        
        //gravity
        let mut friction:f32 = 5.0;

        if self.vertex[0].y >= 700 - self.size {
            friction *= self.mass*0.6;
            self.acceleration.1 *= -0.5;
        }

        let g: f32 = -9.8*3.0;
        self.acceleration.1 -= g*delta_timer;
        self.vertex[0].y += self.acceleration.1.floor() as i16;

        //friction

        if self.acceleration.0 > 0.0 {
            friction *= -1.0;

            if self.acceleration.0 <= 0.05 {
                self.acceleration.0 = 0.0;
            }
        } else if self.acceleration.0 <= 0.0{
            //friction = 5.0;
            if self.acceleration.0 >= -0.05 {
                self.acceleration.0 = 0.0;
                
            }
        }

        if self.acceleration.0 == 0.0 {
            return;
        }

        self.acceleration.0 += friction*delta_timer;
        self.vertex[0].x -= self.acceleration.0.floor() as i16;

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

        if self.get_point(true, false).y >= object.get_point(false, false).y &&
        self.get_point(false, false).y < object.get_point(false, false).y - (object.size/3){
            self.acceleration.1 *= 0.4;
            let pen_depth = self.get_point(true, false).y - object.get_point(false, false).y; 
            
            if pen_depth > 0 {
                self.vertex[0].y -= pen_depth + 1;
            }
        
        } else if self.prev.y > self.vertex[0].y && self.get_point(false, false).y <= object.get_point(true, false).y {
            self.vertex[0].y += object.get_point(true, false).y - self.get_point(false, false).y + 1;

        } else if self.prev.x > self.vertex[0].x && self.get_point(true, true).x >= object.get_point(false, true).x && 
        self.get_point(false, true).x < object.get_point(false, true).x{

            self.vertex[0].x -= self.get_point(true, true).x - object.get_point(false, true).x;
        
        } else if self.prev.x > self.vertex[0].x && self.get_point(false, true).x <= object.get_point(true, true).x &&
        self.get_point(false, true).x > object.get_point(false, true).x{
            
            self.vertex[0].x += object.get_point(true, true).x - self.get_point(false, true).x;
        }
    }

    /*fn is_supported(&mut self, object: &mut Object) {
        if self.x + (self.size/2) < object.x || self.x + (self.size/2 ) > object.x + object.size {=
            if self.y+self.size == object.y { 
                self.fall(false);
            }    
        } 
    }*/

    pub fn is_colliding(&mut self, objects_list: &mut [Object], index: usize) {
        if objects_list.len() < 2 {
            return;
        }

        for i in 0..objects_list.len() {
            let mut object = objects_list[i].clone(); 
            if i != index {
        
                if self.get_point(true, false).y >= object.get_point(false, false).y &&
                self.get_point(false, false).y <= object.get_point(true, false).y &&
                self.get_point(true, true).x >= object.get_point(false, true).x &&
                self.get_point(false, true).x <= object.get_point(true, true).x{
                    
                    self.prevent_overlap(&mut object);
                    self.collision_effects(&mut object);
                    objects_list[i] = object.clone();
                }
            } 
        }
    }

    fn get_point(&mut self, max: bool, x: bool) -> Point{

        let mut result = self.vertex[0];
        
        if x{
            for i in 1..4 {
                if (result.x < self.vertex[i].x && max) || (result.x > self.vertex[i].x && !max) {
                    result = self.vertex[i]
                }
            }
            return result;
        }    

        for i in 1..4 {
            if(result.y < self.vertex[i].y && max) || (result.y > self.vertex[i].y && !max) {
                result = self.vertex[i];
            } 
        }

        return result;

    }

    pub fn update_points(&mut self){
        self.vertex[1].x = self.vertex[0].x + self.size;
        self.vertex[1].y = self.vertex[0].y;
        
        self.vertex[1] = self.get_rotation(self.vertex[1]);

        self.vertex[2].x = self.vertex[0].x + self.size;
        self.vertex[2].y = self.vertex[0].y + self.size;

        self.vertex[2] = self.get_rotation(self.vertex[2]);

        self.vertex[3].x = self.vertex[0].x;
        self.vertex[3].y = self.vertex[0].y + self.size;

        self.vertex[3] = self.get_rotation(self.vertex[3]);

        self.vertex[1] = self.get_rotation(self.vertex[1]);

    }

    fn get_rotation(&mut self, corrner: Point) -> Point {
        let mut result = Point{x: 0, y: 0};

        result.x = (corrner.x * f32::cos(self.theta).round() as i16) - (corrner.y * f32::sin(self.theta).round() as i16);
        result.y = (corrner.x * f32::sin(self.theta).round() as i16) + (corrner.y * f32::cos(self.theta).round() as i16);
        return result;
    }

    pub fn boundries(&mut self, window: &mut Window){
        
        if self.get_point(false, true).x <= 0 {
            self.vertex[0].x += 1 - self.get_point(false, true).x;
            self.acceleration.0 *= -0.2;

        } else if self.get_point(true, true).x > window.get_size().0 as i16{
            //+35 so it wont stick to the wall, needs more diagnosing
            self.vertex[0].x -= self.get_point(true, true).x - window.get_size().0 as i16 + 35;
            //self.vertex[0].x -= 15;
            self.acceleration.0 *= -0.2;
        }

        if self.get_point(false, false).y <= 5 {
            self.vertex[0].y += 6 - self.get_point(false, false).y;
            self.acceleration.1 *= -0.2;

        } else if self.get_point(true, false).y > window.get_size().1 as i16 {
            self.vertex[0].y -= self.get_point(true, false).y - window.get_size().1 as i16;
            self.acceleration.1 *= 0.4;
        }
    }

    pub fn drag(&mut self, window: &mut Window, delta_timer: f32, index: i16) -> i16{

        let mouse_x: i16 = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as i16;
        let mouse_y: i16 = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as i16;

        if !window.get_mouse_down(minifb::MouseButton::Left) 
        || mouse_y <= self.get_point(false, false).y || mouse_y >= self.get_point(true, false).y
        || mouse_x <= self.get_point(false, true).x || mouse_x >= self.get_point(true, true).x {
            return -1;
        }

        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            self.rotate(true);
        }

        drop(mouse_x);
        drop(mouse_y);
        //self.prev.x = self.x;
        //self.prev.y = self.y;

        let mut is_rigid:bool = false;

        if self.rigid {
            self.rigid = false;
            is_rigid = true;
            self.acceleration.1 = 0.0;
            self.acceleration.0 = 0.0;
        }

        self.vertex[0].x = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().0 as i16 - self.size/2;
        self.vertex[0].y = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().1 as i16 - self.size/2;

        if is_rigid {
            self.rigid = true;
        }

        self.acceleration.0 = self.motion(delta_timer, self.prev.x, self.vertex[0].x);
        self.acceleration.1 = self.motion(delta_timer, self.vertex[0].y, self.prev.y);
        return index;
    }

    fn rotate(&mut self, is_drag: bool){
        if self.theta >= 89.0 {
            self.theta = 0.0;
        }else if is_drag{
            self.theta += 45.0;
        }

        return;
    }
}