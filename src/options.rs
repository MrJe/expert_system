#[derive(Default)]
pub struct Options {
    pub graph: bool,
    pub interactive: bool,
    pub file: bool,
    pub comment: bool,
    pub log: bool,
}

impl Options {
    pub fn new() -> Self {
        Options {
            graph: false,
            interactive: false,
            file: false,
            comment: false,
            log: false,
        }
    }

    pub fn load(&mut self, options: &str) {
        for c in options.chars() {
            match c {
                'g' => self.graph = true,
                'i' => self.interactive = true,
                'f' => self.file = true,
                'c' => self.comment = true,
                'l' => self.log = true,
                _ => continue,
            }
        }
    }
}
