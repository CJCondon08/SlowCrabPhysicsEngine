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

    pub fn gravity(&mut self, delta_timer: f32) -> i16{
        //change to collision detection
        if self.y >= 540 || !self.rigid{
            self.acceleration.1 = 0.0;
            self.acceleration.0 = 0.0;
            return self.y;
        }

        let g: f32 = -9.8*3.0;
        self.acceleration.1 -= g*delta_timer;  
        self.y += self.acceleration.1.floor() as i16;
        
        let air_resistance:f32 = 5.0;
        self.acceleration.0 -= air_resistance*delta_timer;
        self.x -= self.acceleration.0 as i16;
        self.y
    }

    fn motion(&mut self, delta_timer: f32, greater_pos:i16, lesser_pos:i16) -> f32 {
        let mut force: f32;

        force = (greater_pos - lesser_pos) as f32;
        force *= delta_timer*130.0;
        self.acceleration.1 = force/self.mass;
        self.acceleration.1
    }

    pub fn collision_effects(/*&mut self, object2: &mut Object*/){
        //self.acceleration.0 += object2.acceleration.0*object2.mass;
        //self.acceleration.1 += object2.acceleration.1*object2.mass;
        //object2.acceleration.0 += self.acceleration.0*self.mass;
        //object2.acceleration.0 += self.acceleration.0*self.mass;
        println!("collision");
    }

    pub fn is_colliding(&mut self, objects_list: &Vec<Object>, index: usize) -> bool{
        let mut i = index;

        if objects_list.len() < 2 {
            return false;
        }

        while i < objects_list.len() -1 {
            if self.x < objects_list[i].x+objects_list[i].size{
                if self.x+self.size > objects_list[i].x {
                    Object::collision_effects();
                }
            }
            i += 1;
        }
        return true;
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