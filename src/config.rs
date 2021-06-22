pub struct Config {
    fps: u32,
    w: u32,
    h: u32,
    title: String,
    actual_w: f64,
    canvas_l: f64,
    canvas_r: f64,
}

impl Config {
    pub fn new(fps: u32, w: u32, h: u32, title: String) -> Self {
        let actual_w = w as f64 * (9_f64 / 32_f64);
        let canvas_l = (w as f64 - actual_w) / 2_f64;
        let canvas_r = canvas_l + actual_w;

        Config {
            fps,
            w,
            h,
            title,
            actual_w,
            canvas_l,
            canvas_r,
        }
    }

    /// Get a reference to the config's fps.
    pub fn fps(&self) -> &u32 {
        &self.fps
    }

    /// Get a reference to the config's title.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get a reference to the config's h.
    pub fn h(&self) -> &u32 {
        &self.h
    }

    /// Get a reference to the config's w
    pub fn w(&self) -> &u32 {
        &self.w
    }

    /// Get a reference to the config's actual w.
    pub fn actual_w(&self) -> &f64 {
        &self.actual_w
    }

    /// Get a reference to the config's canvas l.
    pub fn canvas_l(&self) -> &f64 {
        &self.canvas_l
    }

    /// Get a reference to the config's canvas r.
    pub fn canvas_r(&self) -> &f64 {
        &self.canvas_r
    }
}
impl Default for Config {
    fn default() -> Self {
        Self::new(60, 1600, 900, String::from("Tetris"))
    }
}