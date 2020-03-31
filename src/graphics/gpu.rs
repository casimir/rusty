use glium::glutin::{self, event_loop::EventLoop};
use glium::{implement_vertex, uniform, Display, Program, Surface};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub struct PositionConverter {
    half_width: f32,
    half_height: f32,
}

impl PositionConverter {
    pub fn new(width: usize, height: usize) -> PositionConverter {
        PositionConverter {
            half_width: width as f32 / 2.0,
            half_height: height as f32 / 2.0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> [f32; 2] {
        [(x as f32 - self.half_width), (y as f32 - self.half_height)]
    }
}

pub const VERTEX_SHADER: &str = r#"
    #version 140

    in vec2 position;
    in vec3 color;
    out vec3 rgb;

    uniform mat4 matrix;

    void main() {
        rgb = color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER: &str = r#"
    #version 140

    in vec3 rgb;
    out vec4 color;

    void main() {
        color = vec4(rgb, 1.0);
    }
"#;

#[derive(Debug)]
pub enum GpuContextError {
    DisplayCreation(glium::backend::glutin::DisplayCreationError),
    ProgramCreation(glium::ProgramCreationError),
}

impl From<glium::backend::glutin::DisplayCreationError> for GpuContextError {
    fn from(val: glium::backend::glutin::DisplayCreationError) -> Self {
        Self::DisplayCreation(val)
    }
}

impl From<glium::ProgramCreationError> for GpuContextError {
    fn from(val: glium::ProgramCreationError) -> Self {
        Self::ProgramCreation(val)
    }
}

pub fn init_context(
    width: usize,
    height: usize,
    title: impl Into<String>,
) -> Result<(Display, Program, EventLoop<()>), GpuContextError> {
    let event_loop = EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width as f32, height as f32))
        .with_title(title);
    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop)?;
    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None)?;
    Ok((display, program, event_loop))
}

#[derive(Debug)]
pub enum GpuError {
    Creation(glium::vertex::BufferCreationError),
    Draw(glium::DrawError),
    SwapBuffers(glium::SwapBuffersError),
}

impl From<glium::vertex::BufferCreationError> for GpuError {
    fn from(val: glium::vertex::BufferCreationError) -> Self {
        Self::Creation(val)
    }
}

impl From<glium::DrawError> for GpuError {
    fn from(val: glium::DrawError) -> Self {
        Self::Draw(val)
    }
}

impl From<glium::SwapBuffersError> for GpuError {
    fn from(val: glium::SwapBuffersError) -> Self {
        Self::SwapBuffers(val)
    }
}

fn make_ortho(width: f32, height: f32) -> [[f32; 4]; 4] {
    [
        [2.0 / width, 0.0, 0.0, 0.0],
        [0.0, -2.0 / height, 0.0, 0.0],
        [0.0, 0.0, -1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn render_buffer(
    display: &Display,
    buffer: Vec<Vertex>,
    program: &Program,
    viewport: (f32, f32),
) -> Result<(), GpuError> {
    let uniforms = uniform! {
        matrix: make_ortho(viewport.0, viewport.1),
    };
    let vertex_buffer = glium::VertexBuffer::new(display, &buffer)?;
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(
        &vertex_buffer,
        &glium::index::NoIndices(glium::index::PrimitiveType::Points),
        program,
        &uniforms,
        &Default::default(),
    )?;
    target.finish()?;
    Ok(())
}
