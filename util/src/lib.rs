use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::Split;

pub struct Input(String);

impl Input {
    pub fn from_lines<I, S>(lines: I) -> Input
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Input(
            lines
                .into_iter()
                .fold(String::new(), |complete, line| {
                    complete + line.as_ref() + "\n"
                })
                .trim_end()
                .to_string(),
        )
    }
}

impl Input {
    pub fn load(path: impl AsRef<Path>) -> std::io::Result<Input> {
        let mut input = String::new();
        File::open(path)?.read_to_string(&mut input)?;
        Ok(Input(input))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_lines(&self) -> Split<char> {
        self.0.split('\n')
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn test_as_str() {
        // given some input
        let input = Input("a string".to_string());

        // expect input as str to equal "a string"
        assert_eq!("a string", input.as_str())
    }

    #[test]
    fn test_as_lines() {
        // given some input
        let input = Input("a line\nanother line".to_string());

        // expect input as str to equal "a string"
        itertools::assert_equal(input.as_lines(), vec!["a line", "another line"]);
    }

    #[test]
    fn test_from_lines() {
        // given some lines
        let input = ["a line", "another line"];

        // when Input is created from the lines
        let input = Input::from_lines(input);

        // then as_str returns the lines concatenated
        assert_eq!("a line\nanother line", input.as_str());
    }
}
