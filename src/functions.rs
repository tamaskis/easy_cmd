use crate::traits::AsCommand;
use std::path::Path;
use std::process::Command;

/// Runs a system command.
///
/// # Arguments
///
/// * `cmd` - The command to run. The command can be represented as a single string or a collection
///   of strings, or as any other type that implements the [`crate::AsCommand`] trait.
/// * `dir` - Path to the directory that the command specified by `cmd` should be run in (can be a
///   `&str`, `String`, `Path`, or `PathBuf`). If `None`, the command is run in the current working
///   directory.
///
/// # Returns
///
/// A result where:
///
/// * `Ok` contains `()` if the command executes successfully (exit status zero).
/// * `Err` contains an error message if the command fails to run or exits with a non-zero status.
///
/// # Example
///
/// ```
/// use simple_command::run_command;
/// use std::path::Path;
///
/// // Run `git status` inside the current working directory (the `simple_command` repo).
/// //  --> Note: we have to disambiguate the `None` type to `None::<&str>` since the compiler
/// //      cannot infer the type of the path.
/// run_command("git status", None::<&str>).unwrap();
///
/// // Alternatively, we could run the same command using a collection of strings.
/// //  --> Note: like before, we have to disambiguate the `None` type to `None::<&str>` since the
/// //      compile cannot infer the type of the path.
/// run_command(&["git", "status"], None::<&str>).unwrap();
///
/// // If we want to run `ls -la` in the `src` directory, we can do it like this:
/// run_command("ls -la", Some("src")).unwrap();
///
/// // Alternatively, we could run the same command using a a collection of strings for the command
/// // and a `Path` for the directory:
/// run_command(&vec!["ls", "-la"], Some(Path::new("src"))).unwrap();
/// ```
pub fn run_command<C: AsCommand + ?Sized, P: AsRef<Path>>(
    cmd: &C,
    dir: Option<P>,
) -> Result<(), String> {
    // Create the Command object with the first element as the command name.
    let mut command: Command = cmd.as_command();

    // Set the current directory if provided.
    if let Some(dir_path) = dir {
        command.current_dir(dir_path);
    }

    // Execute the command and capture the exit status.
    let status = command
        .status()
        .map_err(|e| format!("Failed to run command {command:?}: {e}"))?;

    // Check if the command exited successfully.
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command {command:?} failed with status: {status:?}"
        ))
    }
}
