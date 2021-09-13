use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

const GRID_WIDTH: u16 = 64;
const GRID_HEIGHT: u16 = 32;

pub struct Graphics {
    pub ctx: Sdl,               // SDL context from sdl2::init()
    pub canvas: Canvas<Window>, // canvas
    pub pixel_length: u16,      // height and width of a single pixel square
}

impl Graphics {
    pub fn new() -> Result<Self, String> {
        let ctx = sdl2::init()?;
        let video_subsys = ctx.video()?;

        let window = video_subsys
            .window("REMU", 640, 320)
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        Ok(Self {
            ctx,
            canvas,
            pixel_length: 10,
        })
    }

    pub fn draw(&mut self, gfx: &[bool]) -> Result<(), String> {
        let side_length = self.pixel_length;
        let canvas = &mut self.canvas;

        canvas.clear();

        for h in 0..GRID_HEIGHT {
            for w in 0..GRID_WIDTH {
                let x_pos = side_length * w;
                let y_pos = side_length * h;

                let index = h * GRID_WIDTH + w;
                let color = if gfx[index as usize] {
                    Color::RGB(0xFF, 0xFF, 0xFF)
                } else {
                    Color::RGB(0, 0, 0)
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(
                    x_pos.into(),
                    y_pos.into(),
                    side_length.into(),
                    side_length.into(),
                ))?;
            }
        }

        canvas.present();

        Ok(())
    }
}
