use std::{path::{PathBuf, Path}, fs};
use std::io::Result;

pub struct TestCase {
    path: PathBuf,
    input: String,
    output: (Option<String>, Option<String>),
}

impl TestCase {
    pub fn load(path: &Path) -> Result<Self> {
        let mut output = (None, None);
        let ans_path = PathBuf::from(&path).with_extension("ans");
        if ans_path.exists() {
            let ans_content = fs::read_to_string(ans_path)?;
            let mut lines = ans_content.lines();
            output = (
                lines.next().map(String::from),
                lines.next().map(String::from),
            );
        }

        Ok(TestCase {
            path: path.into(),
            input: fs::read_to_string(path)?,
            output,
        })
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap_or_default().to_str().map(str::to_string).unwrap_or_default()
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn output(&self, part: u32) -> Option<&str> {
        match part {
            1 => self.output.0.as_deref(),
            2 => self.output.1.as_deref(),
            _ => None,
        }
    }

    pub fn write_output(&self, part_1: &str, part_2: &str) -> Result<()> {
        let ans_path = PathBuf::from(&self.path).with_extension("ans");
        fs::write(ans_path, format!("{}\n{}\n", part_1, part_2))
    }
}

pub fn load_test_cases(day: u32) -> Result<Vec<TestCase>> {
    let dir = std::fs::read_dir(format!("input/{:02}", day))?;
    let mut paths = Vec::new();
    for file in dir {
        let path = file?.path();
        if !path.extension().is_some_and(|e| e == "ans") {
            paths.push(path);
        }
    }

    paths.sort();
    paths.iter().map(|p| TestCase::load(p)).collect()
}
