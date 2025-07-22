use std::process::Command;

/// Trait to convert various types into a [`Command`].
pub trait AsCommand {
    /// Converts the implementing type into a [`Command`].
    ///
    /// # Returns
    ///
    /// A [`Command`] object that can be used to run a system command.
    ///
    /// # Example
    ///
    /// ```
    /// use easy_cmd::AsCommand;
    /// use std::process::Command;
    ///
    /// // Define the same command using four different types.
    /// let command_str: &str = "echo \"Hello, World!\"";
    /// let command_string: String = String::from("echo \"Hello, World!\"");
    /// let command_vec: Vec<&str> = vec!["echo", "Hello, World!"];
    /// let command_slice: &[&str] = &["echo", "Hello, World!"];
    ///
    /// // Convert them to Command objects. Note that the four Command objects will be identical.
    /// let command_from_str = command_str.as_command();
    /// let command_from_string = command_string.as_command();
    /// let command_from_vec = command_vec.as_command();
    /// let command_from_slice = command_slice.as_command();
    /// ```
    fn as_command(&self) -> Command;
}

impl AsCommand for [&str] {
    fn as_command(&self) -> Command {
        let mut command = Command::new(self[0]);
        command.args(&self[1..]);
        command
    }
}

impl AsCommand for [String] {
    fn as_command(&self) -> Command {
        let parts: Vec<&str> = self.iter().map(|s| s.as_str()).collect();
        parts.as_command()
    }
}

impl AsCommand for &[&str] {
    fn as_command(&self) -> Command {
        self[..].as_command()
    }
}

impl AsCommand for &[String] {
    fn as_command(&self) -> Command {
        self[..].as_command()
    }
}

impl<const N: usize> AsCommand for [&str; N] {
    fn as_command(&self) -> Command {
        self.as_slice().as_command()
    }
}

impl<const N: usize> AsCommand for [String; N] {
    fn as_command(&self) -> Command {
        self.as_slice().as_command()
    }
}

impl AsCommand for Vec<&str> {
    fn as_command(&self) -> Command {
        self.as_slice().as_command()
    }
}

impl AsCommand for Vec<String> {
    fn as_command(&self) -> Command {
        self.as_slice().as_command()
    }
}

impl AsCommand for str {
    fn as_command(&self) -> Command {
        let parts = shell_words::split(self).expect("Failed to parse command string");
        parts.as_command()
    }
}

impl AsCommand for &str {
    fn as_command(&self) -> Command {
        let parts = shell_words::split(self).expect("Failed to parse command string");
        parts.as_command()
    }
}

impl AsCommand for String {
    fn as_command(&self) -> Command {
        self.as_str().as_command()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to check if the command matches the expected program and arguments.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to check.
    /// * `expected_program` - The expected program name.
    /// * `expected_args` - The expected arguments for the command.
    fn check_command(cmd: &Command, expected_program: &str, expected_args: &[&str]) {
        assert_eq!(cmd.get_program().to_str().unwrap(), expected_program);
        let args: Vec<&str> = cmd.get_args().map(|a| a.to_str().unwrap()).collect();
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_as_command_from_str_slice_reference() {
        let cmd: &[&str] = &["echo", "Hello, world!"];
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_string_slice_reference() {
        let cmd: &[&str] = &["echo", "Hello, world!"];
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_str_array() {
        let cmd: [&str; 2] = ["echo", "Hello, world!"];
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_string_array() {
        let cmd: [String; 2] = [String::from("echo"), String::from("Hello, world!")];
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_str_vector() {
        let cmd: Vec<&str> = vec!["echo", "Hello, world!"];
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_string_vector() {
        let cmd: Vec<String> = vec![String::from("echo"), String::from("Hello, world!")];
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_str() {
        let cmd: &str = "echo \"Hello, world!\"";
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }

    #[test]
    fn test_as_command_from_string() {
        let cmd: String = "echo \"Hello, world!\"".into();
        let cmd = cmd.as_command();
        check_command(&cmd, "echo", &["Hello, world!"]);
    }
}
