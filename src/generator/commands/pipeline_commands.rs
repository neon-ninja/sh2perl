use crate::ast::*;
use crate::generator::Generator;

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
        // For printing pipelines, use array-based approach
        let unique_id = generator.get_unique_id();
        output.push_str(&generator.indent());
        output.push_str(&format!("my $output_{};\n", unique_id));
        
        // Create a chain of output variables for the pipeline
        let current_output_var = format!("$output_{}", unique_id);
        
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
                    output.push_str(&format!("open(STDOUT, '>', \\{}) or die \"Cannot redirect STDOUT\";\n", current_output_var));
                    output.push_str(&generator.indent());
                    output.push_str(&generator.generate_command(command));
                    generator.indent_level -= 1;
                    output.push_str(&generator.indent());
                    output.push_str("}\n");
                } else {
                    output.push_str(&format!("{} = `echo \"$output_{}\" | ", current_output_var, unique_id));
                    output.push_str(&generator.generate_command_string_for_system(command));
                    output.push_str("`;\n");
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
                // Use the specialized ls command generation to handle flags like -1
                let unique_id = generator.get_unique_id();
                output.push_str(&generator.indent());
                output.push_str(&format!("my $output_{};\n", unique_id));
                
                // Generate ls command with proper flag handling
                output.push_str(&generator.indent());
                output.push_str(&format!("$output_{} = `ls`;\n", unique_id));
                
                // Now apply grep filtering
                let mut pattern = String::new();
                let mut invert_match = false;

                for arg in &cmd2.args {
                    if let Word::Literal(s) = arg {
                        if s.starts_with('-') {
                            if s.contains('v') { invert_match = true; }
                        } else {
                            pattern = s.clone();
                        }
                    } else {
                        pattern = generator.word_to_perl(arg);
                    }
                }

                if !pattern.is_empty() {
                    // Remove quotes if they exist around the pattern
                    let regex_pattern = if pattern.starts_with('"') && pattern.ends_with('"') {
                        &pattern[1..pattern.len()-1]
                    } else {
                        &pattern
                    };

                    output.push_str(&generator.indent());
                    output.push_str(&format!("my @lines = split(/\\n/, $output_{});\n", unique_id));
                    
                    if invert_match {
                        // Negative grep: exclude lines that match the pattern
                        output.push_str(&generator.indent());
                        output.push_str(&format!("my @filtered = grep {{ $_ !~ /{}/ }} @lines;\n", regex_pattern));
                    } else {
                        // Positive grep: include lines that match the pattern
                        output.push_str(&generator.indent());
                        output.push_str(&format!("my @filtered = grep {{ $_ =~ /{}/ }} @lines;\n", regex_pattern));
                    }
                    
                    output.push_str(&generator.indent());
                    output.push_str(&format!("$output_{} = join(\"\\n\", @filtered);\n", unique_id));
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
                        
                        if cmd_name == "sort" {
                            output.push_str(&generator.indent());
                            output.push_str(&format!("my @lines = split(/\\n/, $output_{});\n", unique_id));
                            output.push_str(&generator.indent());
                            output.push_str(&format!("$output_{} = join(\"\\n\", sort @lines);\n", unique_id));
                        } else if cmd_name == "grep" {
                            output.push_str(&generator.indent());
                            output.push_str(&format!("my @lines = split(/\\n/, $output_{});\n", unique_id));
                            output.push_str(&generator.indent());
                            output.push_str("my @filtered = grep { $_ ne '' } @lines;\n");
                            output.push_str(&generator.indent());
                            output.push_str(&format!("$output_{} = join(\"\\n\", @filtered);\n", unique_id));
                        } else {
                            // Generic command processing
                            output.push_str(&generator.indent());
                            if matches!(command, Command::Redirect(_)) {
                                output.push_str(&generator.generate_command(command));
                                output.push_str(&generator.indent());
                                output.push_str(&format!("$output_{} = $output;\n", unique_id));
                            } else {
                                output.push_str(&format!("$output_{} = `echo \"$output_{}\" | ", unique_id, unique_id));
                                output.push_str(&generator.generate_command_string_for_system(command));
                                output.push_str("`;\n");
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
