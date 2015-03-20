pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub enum Pixel { Color, None }

struct Canvas<'a> {
    w: i32,
    h: i32,
    pixels: &'a [Pixel]
}

impl Canvas {
    fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        // TODO check limits
        self.pixels[x + y * self.w]
    }

    fn set_pixel(&self, x: i32, y:i32, p: Pixel) {
        // TODO check limits
        self.pixels[x + y * self.w] = p;
    }
}

trait Backend {
    fn init(&self, x: i32, y: i32);
    fn render(&self, c: &Canvas);
}

pub struct Renderer<'a> {
    backend: Backend,
    canvas: Canvas<'a>
}

impl Renderer {
    fn render(&self) {
        self.backend.render(self.canvas);
    }
}

pub fn newRenderer(b: Backend, x: i32, y: i32) -> Renderer {
    b.init(x, y);
    Renderer {
        backend: b,
        canvas: Canvas { x: x, y: y, pixels: [Pixel::None; x * y] }
    }
}
