macro_rules! pushrep {
    ($out: expr, $char: expr, $count: expr) => {
        for _ in 0..$count {
            $out.push($char);
        }
    };
}

pub mod grid_print {
    use std::io::Write;
    pub use termcolor::Color;
    use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};

    #[derive(Clone, Copy)]
    pub struct ColoredChar {
        char: char,
        color: Option<Color>,
    }

    impl ColoredChar {
        pub fn new(char: char) -> Self {
            Self { char, color: None }
        }

        pub fn color(mut self, color: Option<Color>) -> Self {
            self.color = color;
            self
        }

        pub fn apply_default_color(&mut self, color: Option<Color>) {
            if self.color.is_none() {
                self.color = color;
            }
        }
    }

    pub struct ColoredString {
        chars: Vec<ColoredChar>,
    }

    impl ColoredString {
        pub fn new() -> Self {
            Self { chars: vec![] }
        }

        pub fn from(string: &str) -> Self {
            Self {
                chars: string.chars().map(ColoredChar::new).collect(),
            }
        }

        pub fn set_color(mut self, color: Color) -> Self {
            for char in self.chars.iter_mut() {
                char.color = Some(color);
            }
            self
        }

        pub fn from_c(string: &str, color: Option<Color>) -> Self {
            Self {
                chars: string
                    .chars()
                    .map(|c| ColoredChar::new(c).color(color))
                    .collect(),
            }
        }

        pub fn push_char(&mut self, char: char) {
            self.chars.push(ColoredChar::new(char));
        }

        pub fn push_char_c(&mut self, char: char, color: Option<Color>) {
            self.chars.push(ColoredChar::new(char).color(color));
        }

        pub fn push_char_rep(&mut self, char: char, count: usize) {
            pushrep!(self.chars, ColoredChar::new(char), count);
        }

        pub fn push_char_rep_c(&mut self, char: char, color: Option<Color>, count: usize) {
            pushrep!(self.chars, ColoredChar::new(char).color(color), count);
        }

        pub fn push_str(&mut self, string: &str) {
            for char in string.chars() {
                self.push_char(char);
            }
        }

        pub fn push_str_c(&mut self, string: &str, color: Option<Color>) {
            for char in string.chars() {
                self.push_char_c(char, color);
            }
        }

        pub fn chain_str(mut self, string: &str) -> Self {
            self.push_str(string);
            self
        }

        pub fn chain_str_c(mut self, string: &str, color: Option<Color>) -> Self {
            self.push_str_c(string, color);
            self
        }

        pub fn push_colored_string(&mut self, string: &ColoredString) {
            self.chars.extend_from_slice(&string.chars);
        }

        pub fn apply_default_color(&mut self, color: Option<Color>) {
            for char in &mut self.chars {
                char.apply_default_color(color);
            }
        }

        pub fn print(&mut self, bufwtr: &mut BufferWriter, buffer: &mut termcolor::Buffer) {
            for char in &self.chars {
                let mut spec = ColorSpec::new();
                spec.set_fg(char.color);
                buffer.set_color(&spec).unwrap();
                buffer.write_all(char.char.to_string().as_bytes()).unwrap();
            }
            bufwtr.print(&buffer).unwrap();
            buffer.reset().unwrap();
            buffer.flush().unwrap();
        }
    }

    pub struct Grid {
        x_labels: Vec<ColoredString>,
        y_labels: Vec<ColoredString>,
        static_column_width: bool,
        draw_x_labels: bool,
        draw_y_labels: bool,
        grid: Vec<Vec<ColoredString>>,
        line_color: Option<Color>,
        x_label_color: Option<Color>,
        y_label_color: Option<Color>,
        cell_color: Option<Color>,
    }

    impl Grid {
        pub fn new() -> Self {
            Self {
                x_labels: vec![],
                y_labels: vec![],
                static_column_width: false,
                draw_x_labels: true,
                draw_y_labels: true,
                grid: vec![],
                line_color: None,
                x_label_color: None,
                y_label_color: None,
                cell_color: None,
            }
        }

        pub fn set_line_color(mut self, color: Color) -> Self {
            self.line_color = Some(color);
            self
        }

        pub fn set_x_label_color(mut self, color: Color) -> Self {
            self.x_label_color = Some(color);
            self
        }

        pub fn set_y_label_color(mut self, color: Color) -> Self {
            self.y_label_color = Some(color);
            self
        }

        pub fn set_cell_color(mut self, color: Color) -> Self {
            self.cell_color = Some(color);
            self
        }

        pub fn set_static_column_width(mut self, static_column_width: bool) -> Self {
            self.static_column_width = static_column_width;
            self
        }

        pub fn set_draw_x_labels(mut self, draw_x_labels: bool) -> Self {
            self.draw_x_labels = draw_x_labels;
            self
        }

        pub fn set_draw_y_labels(mut self, draw_y_labels: bool) -> Self {
            self.draw_y_labels = draw_y_labels;
            self
        }

        pub fn set_x_labels(mut self, mut labels: Vec<ColoredString>) -> Self {
            for s in &mut labels {
                s.apply_default_color(self.x_label_color)
            }
            self.x_labels = labels;
            self
        }

        pub fn set_y_labels(mut self, mut labels: Vec<ColoredString>) -> Self {
            for s in &mut labels {
                s.apply_default_color(self.y_label_color)
            }
            self.y_labels = labels;
            self
        }

        pub fn set_grid(mut self, mut grid: Vec<Vec<ColoredString>>) -> Self {
            for row in &mut grid {
                for s in row {
                    s.apply_default_color(self.cell_color);
                }
            }
            self.grid = grid;
            self
        }

        pub fn print(&self) {
            let mut bufwtr = BufferWriter::stdout(ColorChoice::Always);
            let mut buffer = bufwtr.buffer();
            let mut out = ColoredString::new();

            //   Calculate column widths.
            let mut label_width = 0;
            let mut largest_width = 0;
            let mut column_widths = vec![];
            for i in 0..self.grid.len() {
                if column_widths.len() <= i {
                    column_widths.push(0);
                }
                for j in 0..self.grid[i].len() {
                    if self.grid[i][j].chars.len() > column_widths[i] {
                        column_widths[i] = self.grid[i][j].chars.len();
                    }
                }
                if self.draw_x_labels && self.x_labels.len() > i {
                    if self.x_labels[i].chars.len() > column_widths[i] {
                        column_widths[i] = self.x_labels[i].chars.len();
                    }
                }
            }
            for n in &column_widths {
                if *n > largest_width {
                    largest_width = *n;
                }
            }
            for n in &self.y_labels {
                if n.chars.len() > label_width {
                    label_width = n.chars.len();
                }
            }

            // Draw the top border.
            if self.draw_x_labels {
                if self.draw_y_labels {
                    out.push_char_rep(' ', label_width + 1);
                }
                out.push_char_c('│', self.line_color);
                for i in 0..self.grid.len() {
                    let width;
                    if self.static_column_width {
                        width = largest_width;
                    } else {
                        width = column_widths[i];
                    }
                    let diff = width + 2 - self.x_labels[i].chars.len();
                    let pad_a = diff / 2;
                    let pad_b;
                    if diff % 2 == 0 {
                        pad_b = pad_a;
                    } else {
                        pad_b = pad_a + 1;
                    }
                    out.push_char_rep(' ', pad_a);
                    out.push_colored_string(&self.x_labels[i]);
                    out.push_char_rep(' ', pad_b);
                    out.push_char_c('│', self.line_color);
                }
                out.push_char('\n');
                if self.draw_y_labels {
                    out.push_char_rep_c('─', self.line_color, label_width + 1);
                    out.push_char_c('╆', self.line_color);
                } else {
                    out.push_char_c('┢', self.line_color);
                }
                for i in 0..self.grid.len() {
                    let width;
                    if self.static_column_width {
                        width = largest_width;
                    } else {
                        width = column_widths[i];
                    }
                    out.push_char_rep_c('━', self.line_color, width + 2);
                    if i == self.grid.len() - 1 {
                        out.push_char_c('┪', self.line_color);
                    } else {
                        out.push_char_c('┿', self.line_color);
                    }
                }
                out.push_char('\n');
            } else {
                if self.draw_y_labels {
                    out.push_char_rep_c('─', self.line_color, label_width + 1);
                    out.push_char_c('┲', self.line_color);
                } else {
                    out.push_char_c('┏', self.line_color);
                }
                for i in 0..self.grid.len() {
                    let width;
                    if self.static_column_width {
                        width = largest_width;
                    } else {
                        width = column_widths[i];
                    }
                    out.push_char_rep_c('━', self.line_color, width + 2);
                    if i == self.grid.len() - 1 {
                        out.push_char_c('┓', self.line_color);
                    } else {
                        out.push_char_c('┯', self.line_color);
                    }
                }
                out.push_char('\n');
            }

            // Draw the rows.
            for y in 0..self.grid[0].len() {
                if self.draw_y_labels {
                    let pad_a = label_width - self.y_labels[y].chars.len();
                    out.push_char_rep(' ', pad_a);
                    out.push_colored_string(&self.y_labels[y]);
                    out.push_char(' ');
                }
                out.push_char_c('┃', self.line_color);
                for x in 0..self.grid.len() {
                    let width;
                    if self.static_column_width {
                        width = largest_width;
                    } else {
                        width = column_widths[x];
                    }
                    let diff = width + 2 - self.grid[x][y].chars.len();
                    let pad_a = diff / 2;
                    let pad_b;
                    if diff % 2 == 0 {
                        pad_b = pad_a;
                    } else {
                        pad_b = pad_a + 1;
                    }
                    out.push_char_rep(' ', pad_a);
                    out.push_colored_string(&self.grid[x][y]);
                    out.push_char_rep(' ', pad_b);
                    if x < self.grid.len() - 1 {
                        out.push_char_c('│', self.line_color);
                    }
                }
                out.push_char_c('┃', self.line_color);
                out.push_char('\n');
                if y < self.grid[0].len() - 1 {
                    if self.draw_y_labels {
                        out.push_char_rep_c('─', self.line_color, label_width + 1);
                        out.push_char_c('╂', self.line_color);
                    } else {
                        out.push_char_c('┠', self.line_color);
                    }
                    for x in 0..self.grid.len() {
                        let width;
                        if self.static_column_width {
                            width = largest_width;
                        } else {
                            width = column_widths[x];
                        }
                        out.push_char_rep_c('─', self.line_color, width + 2);
                        if x == self.grid.len() - 1 {
                            out.push_char_c('┨', self.line_color);
                        } else {
                            out.push_char_c('┼', self.line_color);
                        }
                    }
                    out.push_char('\n');
                }
            }

            // Draw the bottom border.
            if self.draw_y_labels {
                out.push_char_rep_c('─', self.line_color, label_width + 1);
                out.push_char_c('┺', self.line_color);
            } else {
                out.push_char_c('┗', self.line_color);
            }
            for i in 0..self.grid.len() {
                let width;
                if self.static_column_width {
                    width = largest_width;
                } else {
                    width = column_widths[i];
                }
                out.push_char_rep_c('━', self.line_color, width + 2);
                if i == self.grid.len() - 1 {
                    out.push_char_c('┛', self.line_color);
                } else {
                    out.push_char_c('┷', self.line_color);
                }
            }
            out.print(&mut bufwtr, &mut buffer);
        }
    }
}
