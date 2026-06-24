use crate::enums::test_result::TestResult;
use crate::methods::reducer_escape::reducer_escape;
use crate::records::reducer::Reducer;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

impl Reducer {
    pub fn run(&mut self) -> TestResult {
        self.write_temp_script(false);

        let escaped_script_name = reducer_escape(self, &self.script_name);
        let mut cmd = self.command.clone();
        while let Some(pos) = cmd.find("{}") {
            cmd.replace_range(pos..pos + 2, &escaped_script_name);
        }

        let mut result = TestResult::NoBug;

        self.step += 1;
        println!("Step {:4}...", self.step);

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to run reducer command");

        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                let bytes_read = reader.read_line(&mut line).unwrap_or(0);
                if bytes_read == 0 {
                    break;
                }

                if line.contains(self.search_text.as_str()) {
                    result = TestResult::BugFound;
                    break;
                }
            }
        }

        let _ = child.wait();

        result
    }
}
