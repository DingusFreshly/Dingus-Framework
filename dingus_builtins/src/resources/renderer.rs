use minifb::{Window, WindowOptions, Key, clamp};
use crate::consts::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug)]
pub struct Renderer {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: false,
                ..WindowOptions::default()
            },
        ).unwrap();


        let buffer = vec![0; width * height];

        Self { window, buffer, width, height }
    }

    // todo: test optimized render methods
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if Self::is_in_bounds(x, SCREEN_WIDTH as i32) && Self::is_in_bounds(y, SCREEN_HEIGHT as i32) {
            self.buffer[y as usize * SCREEN_WIDTH + x as usize] = color;
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let left = Self::restrict_to_bounds(x, SCREEN_WIDTH as i32) as usize;
        let right = Self::restrict_to_bounds(x + w, SCREEN_WIDTH as i32) as usize;
        for i in y..(y + h) {
            let offset = i as usize * SCREEN_WIDTH;
            self.buffer[(offset + left)..(offset + right)].fill(color);
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: u32) {
        let rad2 = radius * radius;
        for y in 0..radius {
            let x = (rad2 - y * y).isqrt();
            let left = Self::restrict_to_bounds(cx - x, SCREEN_WIDTH as i32) as usize;
            let right = Self::restrict_to_bounds(cx + x, SCREEN_WIDTH as i32) as usize;
            let top = cy - y;
            let bottom = cy + y;
            if Self::is_in_bounds(top, SCREEN_HEIGHT as i32) {
                self.buffer[(top as usize * SCREEN_WIDTH + left)..(top as usize * SCREEN_WIDTH + right)].fill(color);
            }
            if Self::is_in_bounds(bottom, SCREEN_HEIGHT as i32) {
                self.buffer[(bottom as usize * SCREEN_WIDTH + left)..(bottom as usize * SCREEN_WIDTH + right)].fill(color);
            }
        }
    }

    fn restrict_to_bounds(x: i32, b: i32) -> i32 {
        if x < 0 { 0 }
        else if x >= b { b - 1 }
        else { x }
    }

    fn is_in_bounds(x: i32, b: i32) -> bool {
        x >= 0 && x < b
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}
