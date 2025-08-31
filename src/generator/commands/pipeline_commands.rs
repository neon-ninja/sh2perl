use crate::ast::*;
use crate::generator::Generator;
use crate::generator::commands::builtins::{is_builtin, generate_generic_builtin, get_specialized_module};

/// Helper function to generate Perl code for a command using the builtins registry
fn generate_command_using_builtins(
    generator: &mut Generator, 
    command: &Command, 
    input_var: &str, 
    output_var: &str
) -> String {
    if let Command::Simple(cmd) = command {
        let cmd_name = match &cmd.name {
            Word::Literal(s) => s,
            _ => "unknown_command"
        };
        
        if is_builtin(cmd_name) {
            if let Some(module) = get_specialized_module(cmd_name) {
                match module {
                    "pipeline_commands" => {
                        // Handle commands that have specialized logic in this module
                        generate_specialized_pipeline_command(generator, cmd, cmd_name, input_var, output_var)
                    },
                    _ => {
                        // Route to other specialized modules (this would need to be implemented)
                        // For now, fall back to generic handling
                        let args: Vec<String> = cmd.args.iter()
                            .map(|arg| generator.word_to_perl(arg))
                            .collect();
                        generate_generic_builtin(cmd_name, &args, input_var, output_var)
                    }
                }
            } else {
                // Use generic builtin handling
                let args: Vec<String> = cmd.args.iter()
                    .map(|arg| generator.word_to_perl(arg))
                    .collect();
                generate_generic_builtin(cmd_name, &args, input_var, output_var)
            }
        } else {
            // Non-builtin command - use system call
            format!("{} = `echo \"${}\" | {}`;\n", 
                output_var, input_var, 
                generator.generate_command_string_for_system(command))
        }
    } else {
        // Non-simple command - use system call
        format!("{} = `echo \"${}\" | {}`;\n", 
            output_var, input_var, 
            generator.generate_command_string_for_system(command))
    }
}

/// Generate specialized pipeline command logic (moved from the main function)
fn generate_specialized_pipeline_command(
    generator: &mut Generator,
    cmd: &SimpleCommand,
    cmd_name: &str,
    input_var: &str,
    output_var: &str
) -> String {
    let mut output = String::new();
    
    match cmd_name {
        "grep" => {
            let pattern = cmd.args.iter()
                .filter(|arg| !matches!(arg, Word::Literal(s) if s.starts_with('-')))
                .next()
                .map(|arg| generator.word_to_perl(arg))
                .unwrap_or_else(|| ".*".to_string());
            
            // Remove quotes if they exist around the pattern
            let regex_pattern = if pattern.starts_with('"') && pattern.ends_with('"') {
                &pattern[1..pattern.len()-1]
            } else {
                &pattern
            };
            
            let grep_unique_id = generator.get_unique_id();
            output.push_str(&format!("my @lines_{} = split(/\\n/, ${});\n", grep_unique_id, input_var));
            output.push_str(&format!("my @filtered_{} = grep {{ $_ =~ /{}/ }} @lines_{};\n", grep_unique_id, regex_pattern, grep_unique_id));
            output.push_str(&format!("{} = join(\"\\n\", @filtered_{});\n", output_var, grep_unique_id));
        },
        "wc" => {
            let is_line_count = cmd.args.iter().any(|arg| matches!(arg, Word::Literal(s) if s == "-l"));
            let wc_unique_id = generator.get_unique_id();
            if is_line_count {
                output.push_str(&format!("my @lines_{} = split(/\\n/, ${});\n", wc_unique_id, input_var));
                output.push_str(&format!("{} = scalar(@lines_{});\n", output_var, wc_unique_id));
            } else {
                // Default to character count
                output.push_str(&format!("{} = length(${});\n", output_var, input_var));
            }
        },
        "sort" => {
            let is_numeric = cmd.args.iter().any(|arg| matches!(arg, Word::Literal(s) if s == "-n"));
            let is_reverse = cmd.args.iter().any(|arg| matches!(arg, Word::Literal(s) if s == "-r"));
            
            let sort_unique_id = generator.get_unique_id();
            output.push_str(&format!("my @lines_{} = split(/\\n/, ${});\n", sort_unique_id, input_var));
            if is_numeric {
                if is_reverse {
                    output.push_str(&format!("@lines_{} = sort {{ (split(/\\s+/, $a))[0] <=> (split(/\\s+/, $b))[0] }} @lines_{};\n", sort_unique_id, sort_unique_id));
                } else {
                    output.push_str(&format!("@lines_{} = sort {{ (split(/\\s+/, $a))[0] <=> (split(/\\s+/, $b))[0] }} @lines_{};\n", sort_unique_id, sort_unique_id));
                }
            } else {
                if is_reverse {
                    output.push_str(&format!("@lines_{} = sort {{ $b cmp $a }} @lines_{};\n", sort_unique_id, sort_unique_id));
                } else {
                    output.push_str(&format!("@lines_{} = sort @lines_{};\n", sort_unique_id, sort_unique_id));
                }
            }
            output.push_str(&format!("{} = join(\"\\n\", @lines_{});\n", output_var, sort_unique_id));
        },
        "uniq" => {
            let is_count = cmd.args.iter().any(|arg| matches!(arg, Word::Literal(s) if s == "-c"));
            
            let uniq_unique_id = generator.get_unique_id();
            output.push_str(&format!("my @lines_{} = split(/\\n/, ${});\n", uniq_unique_id, input_var));
            if is_count {
                output.push_str(&format!("my %count_{};\n", uniq_unique_id));
                output.push_str(&format!("$count_{}{{$_}}++ for @lines_{};\n", uniq_unique_id, uniq_unique_id));
                output.push_str(&format!("{} = join(\"\\n\", map {{ \"$count_{}{{$_}} $_\" }} keys %count_{});\n", output_var, uniq_unique_id, uniq_unique_id));
            } else {
                output.push_str(&format!("my %seen_{};\n", uniq_unique_id));
                output.push_str(&format!("my @unique_{} = grep {{ !$seen_{}{{$_}}++ }} @lines_{};\n", uniq_unique_id, uniq_unique_id, uniq_unique_id));
                output.push_str(&format!("{} = join(\"\\n\", @unique_{});\n", output_var, uniq_unique_id));
            }
        },
        "tr" => {
            // Handle tr command for character translation
            let mut delete_chars = String::new();
            let mut is_delete = false;
            for arg in &cmd.args {
                if let Word::Literal(s) = arg {
                    if s == "-d" {
                        is_delete = true;
                    } else if !s.starts_with('-') {
                        delete_chars = s.clone();
                    }
                }
            }
            
            if is_delete && !delete_chars.is_empty() {
                // Remove quotes if they exist around the pattern
                let chars_to_delete = if delete_chars.starts_with('"') && delete_chars.ends_with('"') {
                    &delete_chars[1..delete_chars.len()-1]
                } else {
                    &delete_chars
                };
                
                output.push_str(&format!("{} = ${};\n", output_var, input_var));
                output.push_str(&format!("{} =~ tr/{}/ /d;\n", output_var, chars_to_delete));
            } else {
                output.push_str(&format!("{} = ${};\n", output_var, input_var));
            }
        },
        "xargs" => {
            // Handle xargs command
            let mut xargs_cmd = String::new();
            let mut xargs_args = Vec::new();
            
            for arg in &cmd.args {
                if let Word::Literal(s) = arg {
                    if s == "grep" {
                        xargs_cmd = s.clone();
                    } else if s.starts_with('-') {
                        xargs_args.push(s.clone());
                    } else {
                        xargs_args.push(s.clone());
                    }
                } else {
                    // Handle non-literal arguments (like StringInterpolation)
                    xargs_args.push(generator.word_to_perl(arg));
                }
            }
            
            if !xargs_cmd.is_empty() {
                let args_str = xargs_args.join(" ");
                output.push_str(&format!("{} = `echo \"${}\" | xargs {} {}`;\n", output_var, input_var, xargs_cmd, args_str));
            } else {
                output.push_str(&format!("{} = ${};\n", output_var, input_var));
            }
        },
        _ => {
            // Fallback for other specialized commands
            let args: Vec<String> = cmd.args.iter()
                .map(|arg| generator.word_to_perl(arg))
                .collect();
            output.push_str(&generate_generic_builtin(cmd_name, &args, input_var, output_var));
        }
    }
    
    output
}

/// Generate a simple pipe pipeline (no logical operators)
pub fn generate_pipeline_impl(generator: &mut Generator, pipeline: &Pipeline) -> String {
    // This is now a pure pipe pipeline since logical operators are handled separately
    generate_simple_pipe_pipeline(generator, pipeline, true)
}

/// Generate a simple pipe pipeline with print option
pub fn generate_pipeline_with_print_option(generator: &mut Generator, pipeline: &Pipeline, should_print: bool) -> String {
    let mut output = String::new();
    
    if pipeline.commands.len() == 1 {
        // Single command, no pipeline needed
        output.push_str(&generator.generate_command(&pipeline.commands[0]));
    } else {
        // Multiple commands, implement proper Perl pipeline
        output.push_str(&generate_simple_pipe_pipeline(generator, pipeline, should_print));
    }
    
    output
}

/// Generate a simple pipe pipeline (commands connected with |)
fn generate_simple_pipe_pipeline(generator: &mut Generator, pipeline: &Pipeline, should_print: bool) -> String {
    let mut output = String::new();
    
    // Wrap in scoping block for proper variable isolation
    output.push_str("{\n");
    generator.indent_level += 1;
    
    if should_print {
        // For printing pipelines, use proper command chaining
        let unique_id = generator.get_unique_id();
        output.push_str(&generator.indent());
        output.push_str(&format!("my $output_{};\n", unique_id));
        
        for (i, command) in pipeline.commands.iter().enumerate() {
            if i > 0 {
                output.push_str("\n");
            }
            
            if i == 0 {
                // First command - generate output
                output.push_str(&generator.indent());
                if matches!(command, Command::Redirect(_)) {
                    output.push_str(&generator.generate_command(command));
                    output.push_str(&generator.indent());
                    output.push_str(&format!("$output_{} = $output;\n", unique_id));
                } else {
                    output.push_str(&format!("$output_{} = `", unique_id));
                    output.push_str(&generator.generate_command_string_for_system(command));
                    output.push_str("`;\n");
                }
            } else {
                // Handle subsequent commands - they should use the previous command's output
                output.push_str(&generator.indent());
                if matches!(command, Command::Redirect(_)) {
                    output.push_str("{\n");
                    generator.indent_level += 1;
                    output.push_str(&generator.indent());
                    output.push_str("local *STDOUT;\n");
                    output.push_str(&generator.indent());
                    output.push_str(&format!("open(STDOUT, '>', \\{}) or die \"Cannot redirect STDOUT\";\n", format!("$output_{}", unique_id)));
                    output.push_str(&generator.indent());
                    output.push_str(&generator.generate_command(command));
                    generator.indent_level -= 1;
                    output.push_str(&generator.indent());
                    output.push_str("}\n");
                } else {
                    // Use proper command chaining without echo
                    if let Command::Simple(cmd) = command {
                        let cmd_name = match &cmd.name {
                            Word::Literal(s) => s,
                            _ => "unknown_command"
                        };
                        
                        // Use the builtins registry to handle command generation
                        let command_output = generate_command_using_builtins(generator, command, &format!("output_{}", unique_id), &format!("output_{}", unique_id));
                        
                        // Split the output into lines and apply indentation
                        for line in command_output.lines() {
                            if !line.trim().is_empty() {
                                output.push_str(&generator.indent());
                                output.push_str(line);
                                if !line.ends_with('\n') {
                                    output.push_str("\n");
                                }
                            }
                        }
                    } else {
                        // Non-simple command
                        output.push_str(&generator.indent());
                        output.push_str(&format!("$output_{} = `echo \"$output_{}\" | ", unique_id, unique_id));
                        output.push_str(&generator.generate_command_string_for_system(command));
                        output.push_str("`;\n");
                    }
                }
            }
        }
        
        // Output the final result
        if should_print {
            output.push_str(&generator.indent());
            output.push_str(&format!("print $output_{};\n", unique_id));
            output.push_str(&generator.indent());
            output.push_str("print \"\\n\";\n");
        }
    } else {
        // For command substitution, use streaming approach
        if let (Command::Simple(cmd1), Command::Simple(cmd2)) = (&pipeline.commands[0], &pipeline.commands[1]) {
            let cmd1_name = match &cmd1.name {
                Word::Literal(s) => s,
                _ => "unknown_command"
            };
            let cmd2_name = match &cmd2.name {
                Word::Literal(s) => s,
                _ => "unknown_command"
            };

            if cmd1_name == "ls" && cmd2_name == "grep" {
                // Use the builtins registry for ls+grep combination
                let unique_id = generator.get_unique_id();
                output.push_str(&generator.indent());
                output.push_str(&format!("my $output_{};\n", unique_id));
                
                // Generate ls command using builtins
                let ls_output = generate_command_using_builtins(generator, &pipeline.commands[0], "", &format!("output_{}", unique_id));
                for line in ls_output.lines() {
                    if !line.trim().is_empty() {
                        output.push_str(&generator.indent());
                        output.push_str(line);
                        if !line.ends_with('\n') {
                            output.push_str("\n");
                        }
                    }
                }
                
                // Now apply grep filtering using builtins
                let grep_output = generate_command_using_builtins(generator, &pipeline.commands[1], &format!("output_{}", unique_id), &format!("output_{}", unique_id));
                for line in grep_output.lines() {
                    if !line.trim().is_empty() {
                        output.push_str(&generator.indent());
                        output.push_str(line);
                        if !line.ends_with('\n') {
                            output.push_str("\n");
                        }
                    }
                }
                
                output.push_str(&generator.indent());
                output.push_str(&format!("$output_{};\n", unique_id));
            } else {
                // Generic 2-command pipeline
                let unique_id = generator.get_unique_id();
                output.push_str(&generator.indent());
                output.push_str(&format!("my $output_{};\n", unique_id));
                
                // Handle the first command
                output.push_str(&generator.indent());
                if matches!(&pipeline.commands[0], Command::Redirect(_)) {
                    output.push_str(&generator.generate_command(&pipeline.commands[0]));
                    output.push_str(&generator.indent());
                    output.push_str(&format!("$output_{} = $output;\n", unique_id));
                } else {
                    output.push_str(&format!("$output_{} = `", unique_id));
                    output.push_str(&generator.generate_command_string_for_system(&pipeline.commands[0]));
                    output.push_str("`;\n");
                }
                
                // Process remaining commands in the pipeline
                for command in &pipeline.commands[1..] {
                    if let Command::Simple(cmd) = command {
                        let cmd_name = match &cmd.name {
                            Word::Literal(s) => s,
                            _ => "unknown_command"
                        };
                        
                        // Use the builtins registry for all commands
                        let command_output = generate_command_using_builtins(generator, command, &format!("output_{}", unique_id), &format!("output_{}", unique_id));
                        
                        // Split the output into lines and apply indentation
                        for line in command_output.lines() {
                            if !line.trim().is_empty() {
                                output.push_str(&generator.indent());
                                output.push_str(line);
                                if !line.ends_with('\n') {
                                    output.push_str("\n");
                                }
                            }
                        }
                    }
                }
                
                output.push_str(&generator.indent());
                output.push_str(&format!("$output_{};\n", unique_id));
            }
        }
    }
    
    // Close the scoping block
    generator.indent_level -= 1;
    output.push_str(&generator.indent());
    output.push_str("}\n");
    
    output
}
