use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;
use crate::shared_utils;

/// Cross-platform helper to create ExitStatus from exit code
/// This is a workaround since ExitStatus::from_raw is platform-specific
pub fn create_exit_status(exit_code: i32) -> std::process::ExitStatus {
    // On Unix systems, we can use the exit code directly
    // On Windows, we'll need to handle this differently
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(exit_code)
    }
    
    #[cfg(windows)]
    {
        use std::os::windows::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(exit_code.try_into().unwrap_or(0))
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        // Fallback for other platforms - create a mock exit status
        // This is not ideal but allows compilation on other platforms
        let mut status = std::process::ExitStatus::default();
        // Note: This won't have the correct exit code, but it allows compilation
        status
    }
}

/// Cross-platform function to run shell scripts
/// Uses bash on Unix systems and tries to find an appropriate shell on Windows
pub fn run_shell_script(filename: &str) -> Result<std::process::Output, String> {
    // Try to find an appropriate shell
    let shell_cmd = if cfg!(unix) {
        "bash"
    } else if cfg!(windows) {
        // On Windows, try to find bash in common locations
        if Command::new("bash").arg("--version").output().is_ok() {
            "bash"
        } else if Command::new("wsl").arg("bash").arg("--version").output().is_ok() {
            "wsl"
        } else if Command::new("git").arg("--version").output().is_ok() {
            // Git Bash is commonly available on Windows
            "git"
        } else {
            return Err("No suitable shell found on Windows. Please install Git Bash, WSL, or another Unix-like shell.".to_string());
        }
    } else {
        "bash" // Default fallback
    };
    
    // Create the command based on the shell type
    let mut child_cmd = if shell_cmd == "wsl" {
        let mut cmd = Command::new("wsl");
        // For WSL, use the same approach as integration tests - convert Windows path to WSL path
        let current_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
        let wsl_working_dir = format!("/mnt/{}/examples", 
            current_dir.replace(":", "").replace("\\", "/"));
        // Change to the examples directory first, then run the original script
        cmd.args(&["bash", "-c", &format!("export LC_ALL=C && cd {} && bash {}", wsl_working_dir, filename)]);
        cmd
    } else if shell_cmd == "git" {
        let mut cmd = Command::new("git");
        cmd.args(&["bash", "-c", &format!("export LC_ALL=C && cd examples && bash {}", filename)]);
        cmd
    } else {
        let mut cmd = Command::new(shell_cmd);
        cmd.args(&["-c", &format!("cd examples && bash {}", filename)]);
        cmd
    };
    
    let mut child = match child_cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
        Ok(c) => c,
        Err(e) => { 
            return Err(format!("Failed to spawn shell: {}", e)); 
        }
    };
    
    let start = std::time::Instant::now();
    let output = loop {
        match child.try_wait() {
            Ok(Some(_)) => break child.wait_with_output().unwrap(),
            Ok(None) => {
                if start.elapsed() > Duration::from_millis(1000) { 
                    let _ = child.kill(); 
                    break child.wait_with_output().unwrap(); 
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(_) => break child.wait_with_output().unwrap(),
        }
    };
    
    Ok(output)
}
