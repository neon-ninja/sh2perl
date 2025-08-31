use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BuiltinCommand {
    pub name: &'static str,
    pub description: &'static str,
}

impl BuiltinCommand {
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
        }
    }
}

pub fn get_builtin_commands() -> HashMap<&'static str, BuiltinCommand> {
    let mut commands = HashMap::new();
    
    // File and directory operations
    commands.insert("ls", BuiltinCommand::new("ls", "List directory contents"));
    commands.insert("cat", BuiltinCommand::new("cat", "Concatenate and display files"));
    commands.insert("find", BuiltinCommand::new("find", "Find files"));
    commands.insert("grep", BuiltinCommand::new("grep", "Search for patterns in text"));
    commands.insert("sed", BuiltinCommand::new("sed", "Stream editor"));
    commands.insert("awk", BuiltinCommand::new("awk", "Pattern scanning and processing"));
    commands.insert("sort", BuiltinCommand::new("sort", "Sort lines"));
    commands.insert("uniq", BuiltinCommand::new("uniq", "Remove duplicate lines"));
    commands.insert("wc", BuiltinCommand::new("wc", "Word, line, and byte count"));
    commands.insert("head", BuiltinCommand::new("head", "Display first lines"));
    commands.insert("tail", BuiltinCommand::new("tail", "Display last lines"));
    commands.insert("cut", BuiltinCommand::new("cut", "Cut sections from lines"));
    commands.insert("paste", BuiltinCommand::new("paste", "Merge lines from files"));
    commands.insert("comm", BuiltinCommand::new("comm", "Compare sorted files"));
    commands.insert("diff", BuiltinCommand::new("diff", "Compare files"));
    commands.insert("tr", BuiltinCommand::new("tr", "Translate or delete characters"));
    commands.insert("xargs", BuiltinCommand::new("xargs", "Execute command with arguments"));
    
    // File manipulation
    commands.insert("cp", BuiltinCommand::new("cp", "Copy files"));
    commands.insert("mv", BuiltinCommand::new("mv", "Move/rename files"));
    commands.insert("rm", BuiltinCommand::new("rm", "Remove files"));
    commands.insert("mkdir", BuiltinCommand::new("mkdir", "Create directories"));
    commands.insert("touch", BuiltinCommand::new("touch", "Create empty files"));
    
    // Text processing
    commands.insert("echo", BuiltinCommand::new("echo", "Display text"));
    commands.insert("printf", BuiltinCommand::new("printf", "Format and print data"));
    commands.insert("basename", BuiltinCommand::new("basename", "Extract filename"));
    commands.insert("dirname", BuiltinCommand::new("dirname", "Extract directory name"));
    
    // System utilities
    commands.insert("date", BuiltinCommand::new("date", "Display date and time"));
    commands.insert("time", BuiltinCommand::new("time", "Time command execution"));
    commands.insert("sleep", BuiltinCommand::new("sleep", "Delay execution"));
    commands.insert("which", BuiltinCommand::new("which", "Locate command"));
    commands.insert("yes", BuiltinCommand::new("yes", "Output string repeatedly"));
    
    // Compression and archiving
    commands.insert("gzip", BuiltinCommand::new("gzip", "Compress files"));
    commands.insert("zcat", BuiltinCommand::new("zcat", "Decompress and display"));
    
    // Network and downloads
    commands.insert("wget", BuiltinCommand::new("wget", "Download files"));
    commands.insert("curl", BuiltinCommand::new("curl", "Transfer data"));
    
    // Process management
    commands.insert("kill", BuiltinCommand::new("kill", "Terminate processes"));
    commands.insert("nohup", BuiltinCommand::new("nohup", "Run command immune to hangups"));
    commands.insert("nice", BuiltinCommand::new("nice", "Run command with modified priority"));
    
    // Checksums and verification
    commands.insert("sha256sum", BuiltinCommand::new("sha256sum", "Compute SHA256 checksums"));
    commands.insert("sha512sum", BuiltinCommand::new("sha512sum", "Compute SHA512 checksums"));
    commands.insert("strings", BuiltinCommand::new("strings", "Extract printable strings"));
    
    // I/O redirection
    commands.insert("tee", BuiltinCommand::new("tee", "Read from stdin, write to stdout and files"));
    
    //TODO: pkill and killall
    commands
}

pub fn is_builtin(command_name: &str) -> bool {
    get_builtin_commands().contains_key(command_name)
}

/// Get the module name that handles this builtin command's Perl generation
/// Returns None if the command should use generic handling
pub fn get_specialized_module(command_name: &str) -> Option<&'static str> {
    match command_name {
        "grep" | "wc" | "sort" | "uniq" | "tr" | "xargs" => Some("pipeline_commands"),
        "ls" => Some("ls_commands"),
        "echo" => Some("echo_commands"),
        "printf" => Some("printf_commands"),
        _ => None
    }
}

/// Generate generic Perl code for a builtin command that doesn't need special handling
/// This is a fallback for commands that don't have specialized modules
pub fn generate_generic_builtin(command_name: &str, args: &[String], input_var: &str, output_var: &str) -> String {
    let args_str = args.join(" ");
    format!("{} = `echo \"${}\" | {} {}`;\n", output_var, input_var, command_name, args_str)
}




