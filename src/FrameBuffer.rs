
struct FrameBuffer {
    width: i32,
    height: i32,
    buffer: vec
}

impl FrameBuffer {
    pub fn new() -> FrameBuffer {
        FrameBuffer {width: 512, height: 512, buffer: () }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn draw_pixel() {

    }
}