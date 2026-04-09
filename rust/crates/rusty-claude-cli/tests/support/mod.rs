use std::fs;
use std::path::Path;
use std::process::Command;

pub fn claw_command(current_dir: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_claw"));
    command.current_dir(current_dir);
    apply_isolated_cli_env(&mut command, current_dir);
    command
}

fn apply_isolated_cli_env(command: &mut Command, current_dir: &Path) {
    let env_root = current_dir.join(".claw-test-env");
    let home = env_root.join("home");
    let config_home = env_root.join("claw-config");
    let codex_home = env_root.join("codex-home");
    let claude_config_home = env_root.join("claude-config");

    ensure_dir(&home);
    ensure_dir(&config_home);
    ensure_dir(&codex_home);
    ensure_dir(&claude_config_home);

    command
        .env("HOME", home)
        .env("CLAW_CONFIG_HOME", config_home)
        .env("CODEX_HOME", codex_home)
        .env("CLAUDE_CONFIG_HOME", claude_config_home);
}

fn ensure_dir(path: &Path) {
    fs::create_dir_all(path).expect("test env directory should exist");
}
