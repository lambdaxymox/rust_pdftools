
pub trait AsShellCommand {
    fn as_shell_command(&self) -> String;
}

pub trait AsShellArg {
    fn as_shell_arg(&self) -> String;
}
