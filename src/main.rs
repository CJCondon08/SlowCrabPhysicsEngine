use std::time::Instant;
use minifb::{Key, Window, WindowOptions};
mod object;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let width:usize = 1000;
    let height:usize = 700;

    let background_color: u32 = from_u8_rgb(70, 70, 70);
    let mut window: Window = Window::new("Slow Crab", width, height, WindowOptions::default()).unwrap();
    
    let mut object_list: Vec<object::Object> = Vec::new();
    let square_size: usize = 150;
    let mut delta_timer:f32 = 0.01;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let loop_time: Instant = Instant::now();

        for i in 0..object_list.len() {
            object_list[i].gravity(delta_timer);
            object_list[i].drag(&mut window, delta_timer);
            let mut temp = object_list[i];
            temp.is_colliding(&mut object_list, i);
            object_list[i] = temp;
            drop(temp);
            object_list[i].boundries(&mut window);
        }

        if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
            let mut square = object::Object::new_rigid();
            //square.y = 0;
            square.x = 20 + ((square_size+10) * object_list.len()) as i16;
            object_list.push(square);
        }

        // Clear the windowobject_list[i].
        let buffer = &mut vec![background_color; width * (height + 10)];
        
        //Draw objects
        for z in 0..object_list.len(){
            for y in object_list[z].y ..(object_list[z].y + square_size as i16) {
                for x in object_list[z].x ..(object_list[z].x + square_size as i16) {
                    let index: usize = x as usize + (width * y as usize);
                    buffer[index] = from_u8_rgb(40*(z as u8), 80, 200);
                }
            }
        }

        // Update the window
        window.update_with_buffer(buffer, width, height).unwrap();

        delta_timer = loop_time.elapsed().as_secs_f32();
        drop(loop_time);

        //println!("{}", 1.0/delta_timer);
    }
}