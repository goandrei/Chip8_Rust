use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// original Chip8 width
const WIDTH: u32 = 64;
// original Chip8 height
const HEIGHT: u32 = 32;
const RESIZE: u32 = 10;

pub struct Display {
    canvas: Canvas<Window>,
    display: [u8; 32*64],
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Chip8", WIDTH * RESIZE, HEIGHT * RESIZE)
                                    .position_centered()
                                    .build()
                                    .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas: canvas,
            display: [0u8; 32*64],
        }
    }

    pub fn clear_screen(&mut self) -> Result<bool, String> {
        // clear the collision detection display
        for elem in self.display.iter_mut() {
            *elem = 0u8;
        }

        // clear the actual screen
        self.canvas.set_draw_color(Color::RGB(0u8, 0u8, 0u8));
        self.canvas.clear();
        self.canvas.present();

        Ok(true)
    }

    pub fn initialize(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn get_display(&self) -> [u8; 32 * 64] {
        self.display
    }

    pub fn set_display(&mut self, display: [u8; 32 * 64]) {
        self.display = display;
    }

    pub fn draw(&mut self, x: u32, y: u32, buff: u8) -> Result<bool, String> {

        let x = x % WIDTH;
        let y = y % HEIGHT;

        let mut collision = false;
        for col in 0..8u32 {
            let bit = (buff >> (7 - col)) & 0x01;
            let display_coord = self.coord_to_matrix(x + col, y) % 2048;
            println!("{}, {} => {}", x+col, y, display_coord);

            // if there is a pixel flip set the flag and continue since we also know we don't have to draw anything
            collision |= self.display[display_coord] == 1 && bit == 1;   
            self.display[display_coord] ^= bit;
            
            self.set_color(self.display[display_coord]);

            // compute the new position relative to the resized axes
            let x_position = (x + col) * RESIZE;
            let y_position = y * RESIZE;

            match self.canvas.fill_rect(Rect::new(x_position as i32, y_position as i32, RESIZE, RESIZE)) {
                Ok(_) => (),
                Err(err) => return Err(err),
            };
            println!();
        }

        self.canvas.present();

        Ok(collision)
    }

    fn coord_to_matrix(&self, x: u32, y: u32) -> usize {
        (x + y * WIDTH) as usize
    }

    fn set_color(&mut self, bit: u8) {
        if bit == 1 {
            // if bit = 1 -> set color to white
            self.canvas.set_draw_color(Color::RGB(255u8, 255u8, 255u8));
            print!("1");
        } else {
            // if bit = 0 -> set color to black => clear it
            self.canvas.set_draw_color(Color::RGB(0u8, 0u8, 0u8));
            print!("0");
        }
    }
}