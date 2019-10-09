pub struct Options {
    pub graph: bool,
    pub interactive: bool,
    pub reasoning: bool,
}

impl Options {
    pub fn new() -> Self {
        Options {
            graph: false,
            interactive: false,
            reasoning: false,
        }
    }

    pub fn load(&mut self, options: &str) {
        for c in options.chars() {
            match c {
                'g' => self.graph = true,
                'i' => self.interactive = true,
                'r' => self.reasoning = true,
                _   => continue,
            }
        }
    }
}
