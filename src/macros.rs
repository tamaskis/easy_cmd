/// Runs a system command and exits with code 1 if an error occurs.
///
/// # Arguments
///
/// * `$cmd` - The command to run in the current working directory. The command can be represented
///   as a single string or a collection of strings, or as any other type that implements the
///   [`crate::AsCommand`] trait.
/// * `$dir` - Path to the directory that the command specified by `$cmd` should be run in (can be a
///   `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Example
///
/// ```
/// use simple_command::run_command;
///
/// // Run a command in the current working directory.
/// run_command!("git status");
///
/// // The command can also be provided as a collection of strings.
/// run_command!(&["git", "status"]);
/// ```
#[macro_export]
macro_rules! run_command {
    ($cmd:expr) => {{
        // Note that we need to supply &str as a type parameter because even though it isn't used,
        // the compiler still needs to know the type, and it cannot infer it from the macro call.
        if let Err(e) = simple_command::run_command::<_, &str>($cmd, None) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }};
}

/// Runs a system command in a specific directory and exits with code 1 if an error occurs.
///
/// # Arguments
///
/// * `$cmd` - The command to run in the directory specified by `$dir`. The command can be
///   represented as a single string or a collection of strings, or as any other type that
///   implements the [`crate::AsCommand`] trait.
/// * `$dir` - Path to the directory that the command specified by `$cmd` should be run in (can be a
///   `&str`, `String`, `Path`, or `PathBuf`).
///
/// # Example
///
/// ```
/// use simple_command::run_command_in_dir;
/// use std::path::Path;
///
/// // Run a command in a specific directory.
/// run_command_in_dir!("git status", "src");
///
/// // The command can also be provided as a collection of strings, and the directory can also be
/// // specified as a Path.
/// run_command_in_dir!(&["git", "status"], Path::new("src"));
/// ```
#[macro_export]
macro_rules! run_command_in_dir {
    ($cmd:expr, $dir:expr) => {{
        if let Err(e) = simple_command::run_command($cmd, Some($dir)) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }};
}
