#[derive(Debug)]
pub enum OutputOption {
    Links,
    All,
    OnlyCode,
}

#[derive(Debug)]
pub struct Config {
    option: OutputOption,
    numbers: u8,
    colorize: bool,
}

impl Config {
    pub fn new(output_option: OutputOption, numbers: u8, colorize: bool) -> Self {
        return Config {
            option: output_option,
            numbers,
            colorize
        }
    }

    pub fn option(&self) -> &OutputOption {
        return &self.option;
    }

    pub fn numbers(&self) -> u8 {
        return self.numbers;
    }

    pub fn colorize(&self) -> bool {
        return self.colorize;
    }
}
