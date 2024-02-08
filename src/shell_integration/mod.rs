use crate::Shell;

const BASH_PREEXEC: &str = include_str!("../assets/bash-preexec.sh");
const SHELL_HOOK: &str = include_str!("../assets/shell-hook");

pub async fn shell_integration(shell: Shell) -> Result<(), Box<dyn std::error::Error>> {
    let mut memo = String::new();

    if shell == Shell::Bash {
        memo += BASH_PREEXEC;
    }

    memo += SHELL_HOOK;
    println!("{}", memo);
    Ok(())
}
