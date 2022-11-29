use std::fmt::Display;

pub struct Indentation {
    space_per_level: usize,
    level: usize,
    spaces: String,
}

impl Indentation {
    pub fn new(space_per_level: usize, initial_level: usize) -> Indentation {
        Indentation {
            space_per_level,
            level: initial_level,
            spaces: " ".repeat(space_per_level * initial_level),
        }
    }

    pub fn indent(&mut self) {
        self.level += 1;
        self.spaces = " ".repeat(self.space_per_level * self.level);
    }

    pub fn outdent(&mut self) {
        assert_ne!(self.level, 0);
        self.level -= 1;
        self.spaces = " ".repeat(self.space_per_level * self.level);
    }
}

impl Display for Indentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.spaces)
    }
}
