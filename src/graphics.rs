use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;

pub struct Graphics {
    pub ctx: Sdl,                   // SDL context from sdl2::init()
    pub canvas: Canvas<Window>,     // canvas
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

        Ok(Self { ctx, canvas })
    }
}
