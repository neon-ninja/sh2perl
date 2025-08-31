Running shell script: examples/003_pipeline.sh
Generated Perl code:
#!/usr/bin/env perl
use strict;
use warnings;
use File::Basename;

{
    my $output_1;
        my @ls_files;
    if (opendir(my $dh, '.')) {
        while (my $file = readdir($dh)) {
            next if $file eq '.' || $file eq '..';
            push @ls_files, $file;
        }
        closedir($dh);
    }
    $output_1 = join("\n", @ls_files);
    print $output_1;
    print "\n";

    my $grep_result_2;
my @grep_lines_2 = split(/\n/, $output_1);
my @grep_filtered_2 = grep /\.txt$/, @grep_lines_2;
$grep_result_2 = join("\n", @grep_filtered_2);
    $output_1 = $grep_result_2;
    print $output_1;
    print "\n";

    my @wc_lines_1 = split(/\n/, $output_1);
my $wc_line_count_1 = scalar(@wc_lines_1);
my $wc_result_1 = '';
$wc_result_1 .= "$wc_line_count_1 ";
$wc_result_1 =~ s/\s+$//;
$output_1 = $wc_result_1;
    print $output_1;
    print "\n";
}
{
    my $output_3;
    $output_3 = '';
if (open(my $fh, '<', 'file.txt')) {
while (my $line = <$fh>) {
$line =~ s/\r\n?/\n/g; # Normalize line endings
$output_3 .= $line;
}
close($fh);
} else {
warn "cat: file.txt: No such file or directory";
exit(1);
}

    print $output_3;
    print "\n";

    my @sort_lines_3 = split(/\n/, $output_3);
my @sort_sorted_3 = sort @sort_lines_3;
$output_3 = join("\n", @sort_sorted_3);
    print $output_3;
    print "\n";

    my @uniq_lines_3 = split(/\n/, $output_3);
my %uniq_counts_3;
foreach my $line (@uniq_lines_3) {
$uniq_counts_3{$line}++;
}
my @uniq_result_3;
foreach my $line (keys %uniq_counts_3) {
push @uniq_result_3, sprintf("%7d %s", $uniq_counts_3{$line}, $line);
}
$output_3 = join("\n", @uniq_result_3);
    print $output_3;
    print "\n";

    my @sort_lines_3 = split(/\n/, $output_3);
my @sort_sorted_3 = sort { 
    my $a_num = (split(/\s+/, $a))[0] || 0;
    my $b_num = (split(/\s+/, $b))[0] || 0;
    $a_num <=> $b_num || $a cmp $b;
} @sort_lines_3;
@sort_sorted_3 = reverse(@sort_sorted_3);
$output_3 = join("\n", @sort_sorted_3);
    print $output_3;
    print "\n";
}
{
    my $output_4;
        my @find_files;
    sub find_files {
        my ($dir, $pattern) = @_;
        if (opendir(my $dh, $dir)) {
            while (my $file = readdir($dh)) {
                next if $file eq '.' || $file eq '..';
                my $full_path = $dir eq '.' ? "./$file" : "$dir/$file";
                if (-d $full_path) {
                    find_files($full_path, $pattern);
                } elsif ($file =~ /^$pattern$/) {
                    push @find_files, $full_path;
                }
            }
            closedir($dh);
        }
    }
    find_files('.', '.*\.sh');
    $output_4 = join("\n", @find_files);

    print $output_4;
    print "\n";

    $output_4 = `xargs grep -l "function"`;
    print $output_4;
    print "\n";

    $output_4 = `tr -d "\\\\/"`;
    print $output_4;
    print "\n";
}


--- Running generated Perl code ---
-files | grep tr.rs
.cursor
.git
.highest_tests_and_lines.txt.swp
.txt
001
001..005
002
003
004
005
1
2
268
269
3
4
5
all_summary.txt
analyze_test_regressions.ps1
analyze_test_regressions.py
analyze_test_regressions.sh
analyze_test_regressions_fast.py
analyze_test_regressions_optimized.py
analyze_test_regressions_simple.py
analyze_test_regressions_ultra_optimized.py
ast_output.txt
backup_20250827_155315
backup_20250827_163237
backup_20250827_163351
backup_20250827_163455
backup_20250827_164440
backup_20250827_171835
backup_20250827_172105
backup_20250827_173705
backup_20250827_174510
backup_20250827_174955
backup_20250827_175748
backup_20250827_180101
bash_tests
build-and-run-wasm.ps1
build-wasm.sh
Cargo.lock
Cargo.toml
cc.err
cc.txt
clean_008.pl
clean_008_streaming.pl
clean_dead_code.sh
clean_dead_code_conservative.sh
clean_dead_code_fixed.sh
clean_dead_code_safe.sh
clean_dead_code_safe_v2.sh
clean_dead_code_simple.sh
command_cache.json
comment_debug.bat
comment_debug.ps1
comment_debug.sh
comparison.txt
config.txt
cursor_monitor.py
dead_code_analysis.tsv
debug_find.pl
debug_function_test.rs
debug_lexer.rs
debug_output.rs
debug_test.sh
diff1.txt
diff2.txt
errors.log
examples
EXAMPLES_REFACTORING_SUMMARY.md
f
f.bat
f.ps1
fail.bat
failed.txt
file.txt
file_
final_result.txt
first_n_tests_passed.txt
fix_note.txt
get_regressions.sh
git_commit_sizes.pl
grep_clean.pl
grep_final.pl
grep_output.pl
grep_output_fixed.pl
grep_output_new.pl
grep_test.pl
h2s.sh
hell -ExecutionPolicy Bypass -File analyze_test_regressions.ps1
highest_tests_and_lines.txt
history
history2summary.sh
jumk
junk
lexer_output.txt
LICENSE
monitor.py
outlex.txt
output.txt
output2.txt
output3.txt
output4.txt
output5.txt
output6.txt
output7.txt
output8.txt
output_advanced.txt
output_advanced10.txt
output_advanced2.txt
output_advanced3.txt
output_advanced4.txt
output_advanced5.txt
output_advanced6.txt
output_advanced7.txt
output_advanced8.txt
output_advanced9.txt
output_hard_to_lex.txt
output_hard_to_lex_final.txt
output_hard_to_lex_revised.txt
output_hard_to_lex_simple.txt
output_local_names.txt
output_local_names_fixed.txt
output_simple.txt
output_simple_fixed.txt
out_56.txt
out_56_fix2.txt
out_case.txt
out_case_fix.txt
out_case_fix2.txt
out_case_fix3.txt
out_case_fix4.txt
out_case_fix5.txt
out_case_fix6.txt
out_fail.txt
out_fail56.txt
out_fail56_2.txt
out_fail57.txt
out_fail57_again.txt
out_fix1.txt
out_fix2.txt
PARSER_REFACTORING_SUMMARY.md
parse_cc_err.py
passed.txt
pkg
project
r.bat
README-WASM.md
README.md
README_DEBUG_SCRIPTS.md
regressions.txt
remove_func.py
rename_files.sh
run-next-test.bat
SHARED_UTILS_REFACTORING.md
simple_ls_test.sh
src
summary.txt
t
t.tsv
t.txt
t2
target
temp.rs
temp_file
temp_file1
temp_file2
temp_file3
temp_line.txt
temp_output.txt
temp_output2.txt
temp_perl.pl
temp_test.exe
temp_test.pdb
temp_test.rs
temp_test.txt
temp_test_unix.txt
test-wasm-simple.html
test-wasm.html
test1
tests
testSummary.txt
test_008.pl
test_008_streaming.pl
test_arithmetic.rs
test_arithmetic.sh
test_arithmetic_eval.sh
test_arithmetic_simple.sh
test_array.sh
test_arrays_fixed.pl
test_arrays_fixed2.pl
test_arrays_fixed3.pl
test_arrays_fixed4.pl
test_arrays_fixed5.pl
test_array_slice.sh
test_array_syntax.sh
test_cartesian.exe
test_cartesian.pdb
test_cartesian.rs
test_cd_dot_dot.sh
test_comment_debug.sh
test_comment_issue.sh
test_comprehensive.sh
test_context_echo.rs
test_debug.sh
test_dollar_identifier.sh
test_echo_minus.sh
test_extended.sh
test_fix.pl
test_function.sh
test_function_debug.rs
test_function_simple.sh
test_gen.pl
test_indexed_arrays.txt
test_lexer.rs
test_lexer_debug.exe
test_lexer_debug.pdb
test_lexer_debug.rs
test_lexer_simple.rs
test_line.sh
test_line1.sh
test_line1_modified.sh
test_line_counting.rs
test_local_names_preserved.sh
test_ls_debug.sh
test_ls_glob.sh
test_ls_star.sh
test_ls_star_dot_sh.sh
test_ls_working.sh
test_minimal.rs
test_minimal.sh
test_minimal_comment.sh
test_minimal_new.sh
test_mixed_pipeline.sh
test_modern_perl_signatures.sh
test_modern_perl_signatures_advanced.sh
test_modern_perl_signatures_simple.sh
test_multiple_spaces.rs
test_newline_filter.sh
test_output.pl
test_output.txt
test_output2.pl
test_output3.pl
test_output4.pl
test_output5.pl
test_output6.pl
test_output7.pl
test_output_ansi_quoting.pdb
test_output_ansi_quoting_basic.pdb
test_output_ansi_quoting_escape.pdb
test_output_ansi_quoting_practical.pdb
test_output_ansi_quoting_unicode.pdb
test_output_args.pdb
test_output_brace_expansion_practical.pdb
test_output_cat_EOF.pdb
test_output_control_flow_function.pdb
test_output_control_flow_if.pdb
test_output_find_example.pdb
test_output_misc.pdb
test_output_now.txt
test_output_parameter_expansion.pdb
test_output_process_substitution_advanced.pdb
test_output_process_substitution_comm.pdb
test_output_shell_calling_perl.pdb
test_output_simple.pdb
test_output_simple_backup.pdb
test_output_subprocess.pdb
test_output_test_ls_star_dot_sh.pdb
test_output_test_quoted.pdb
test_param_expansion.sh
test_parser_debug.rs
test_perl_generation.sh
test_pipeline.pl
test_pipeline_fixed.pl
test_pipeline_fixed2.pl
test_plus_assign.sh
test_printf.rs
test_problematic_part.sh
test_process_sub_clean.pl
test_process_sub_final.pl
test_process_sub_fixed.pl
test_process_sub_fixed2.pl
test_process_sub_fixed3.pl
test_process_sub_fixed4.pl
test_process_sub_fixed5.pl
test_process_sub_fixed6.pl
test_process_sub_fixed7.pl
test_process_sub_perl_only.pl
test_read_r.rs
test_regex.rs
test_section.sh
test_separate_binaries.bat
test_separate_binaries.ps1
test_simple.exe
test_simple.pdb
test_simple.rs
test_simple.sh
test_simple_array.sh
test_simple_echo.rs
test_simple_ls.sh
test_simple_output.pl
test_single.sh
test_single_arithmetic.sh
test_spaces.sh
test_syntax_error.sh
test_utf8.sh
test_very_large_output.sh
test_while.sh
test_whitespace.sh
tmp
tmp_output.tmp
tmp_test_perl.pl
tokens.txt
txt
t_regressions.py
users.txt
wasm.ps1
www
__equiv_rust_args_bin.pdb
__equiv_rust_misc_bin.pdb
__equiv_rust_simple_bin.pdb
__equiv_rust_test_quoted_bin.pdb
__pycache__
__tmp_run.pl
__tmp_test_output.rs
.txt
all_summary.txt
ast_output.txt
cc.txt
comparison.txt
config.txt
diff1.txt
diff2.txt
failed.txt
file.txt
final_result.txt
first_n_tests_passed.txt
fix_note.txt
highest_tests_and_lines.txt
lexer_output.txt
outlex.txt
output.txt
output2.txt
output3.txt
output4.txt
output5.txt
output6.txt
output7.txt
output8.txt
output_advanced.txt
output_advanced10.txt
output_advanced2.txt
output_advanced3.txt
output_advanced4.txt
output_advanced5.txt
output_advanced6.txt
output_advanced7.txt
output_advanced8.txt
output_advanced9.txt
output_hard_to_lex.txt
output_hard_to_lex_final.txt
output_hard_to_lex_revised.txt
output_hard_to_lex_simple.txt
output_local_names.txt
output_local_names_fixed.txt
output_simple.txt
output_simple_fixed.txt
out_56.txt
out_56_fix2.txt
out_case.txt
out_case_fix.txt
out_case_fix2.txt
out_case_fix3.txt
out_case_fix4.txt
out_case_fix5.txt
out_case_fix6.txt
out_fail.txt
out_fail56.txt
out_fail56_2.txt
out_fail57.txt
out_fail57_again.txt
out_fix1.txt
out_fix2.txt
passed.txt
regressions.txt
summary.txt
t.txt
temp_line.txt
temp_output.txt
temp_output2.txt
temp_test.txt
temp_test_unix.txt
testSummary.txt
test_indexed_arrays.txt
test_output.txt
test_output_now.txt
tokens.txt
users.txt
73
apple
banana
apple
cherry
banana
apple

apple
apple
apple
banana
banana
cherry
      3 apple
      1 cherry
      2 banana
      3 apple
      2 banana
      1 cherry
./analyze_test_regressions.sh
./bash_tests/test_local_names_preserved.sh
./build-wasm.sh
./clean_dead_code.sh
./clean_dead_code_conservative.sh
./clean_dead_code_fixed.sh
./clean_dead_code_safe.sh
./clean_dead_code_safe_v2.sh
./clean_dead_code_simple.sh
./comment_debug.sh
./debug_test.sh
./examples/001_simple.sh
./examples/002_control_flow.sh
./examples/003_pipeline.sh
./examples/004_test_quoted.sh
./examples/005_args.sh
./examples/006_misc.sh
./examples/007_cat_EOF.sh
./examples/008_simple_backup.sh
./examples/009_arrays.sh
./examples/010_pattern_matching.sh
./examples/011_brace_expansion.sh
./examples/012_process_substitution.sh
./examples/013_parameter_expansion.sh
./examples/014_ansi_quoting.sh
./examples/015_grep_advanced.sh
./examples/016_grep_basic.sh
./examples/017_grep_context.sh
./examples/018_grep_params.sh
./examples/019_grep_regex.sh
./examples/020_ansi_quoting_basic.sh
./examples/021_ansi_quoting_escape.sh
./examples/022_ansi_quoting_unicode.sh
./examples/023_ansi_quoting_practical.sh
./examples/024_parameter_expansion_case.sh
./examples/025_parameter_expansion_advanced.sh
./examples/026_parameter_expansion_more.sh
./examples/027_parameter_expansion_defaults.sh
./examples/028_arrays_indexed.sh
./examples/029_arrays_associative.sh
./examples/030_control_flow_if.sh
./examples/031_control_flow_loops.sh
./examples/032_control_flow_function.sh
./examples/033_brace_expansion_basic.sh
./examples/034_brace_expansion_advanced.sh
./examples/035_brace_expansion_practical.sh
./examples/036_pattern_matching_basic.sh
./examples/037_pattern_matching_extglob.sh
./examples/038_pattern_matching_nocase.sh
./examples/039_process_substitution_here.sh
./examples/040_process_substitution_comm.sh
./examples/041_process_substitution_mapfile.sh
./examples/042_process_substitution_advanced.sh
./examples/043_home.sh
./examples/044_find_example.sh
./examples/045_shell_calling_perl.sh
./examples/046_cd..sh
./examples/047_for_arithematic.sh
./examples/048_subprocess.sh
./examples/049_local.sh
./examples/050_test_ls_star_dot_sh.sh
./examples/051_primes.sh
./examples/052_numeric_computations.sh
./examples/053_gcd.sh
./examples/054_fibonacci.sh
./examples/055_factorize.sh
./examples/056_send_args.sh
./examples/057_case.sh
./examples/058_advanced_bash_idioms.sh
./examples/059_issue3.sh
./examples/060_issue5.sh
./examples/061_test_local_names_preserved.sh
./examples/062_01_ambiguous_operators.sh
./examples/062_02_complex_parameter_expansions.sh
./examples/062_03_complex_heredocs.sh
./examples/062_04_nested_arithmetic.sh
./examples/062_05_nested_command_substitution.sh
./examples/062_06_process_substitution.sh
./examples/062_07_complex_brace_expansion.sh
./examples/062_08_simple_case.sh
./examples/062_09_complex_function.sh
./examples/062_10_simple_pipeline.sh
./examples/062_11_mixed_arithmetic.sh
./examples/062_12_complex_string_interpolation.sh
./examples/062_13_simple_test_expressions.sh
./examples/062_14_complex_array_operations.sh
./examples/062_15_complex_local_variables.sh
./examples/062_hard_to_lex.sh
./examples/063_01_deeply_nested_arithmetic.sh
./examples/063_02_complex_array_assignments.sh
./examples/063_03_nested_command_substitutions.sh
./examples/063_04_complex_parameter_expansion.sh
./examples/063_05_heredoc_with_complex_content.sh
./examples/063_06_complex_pipeline_background.sh
./examples/063_07_nested_if_statements.sh
./examples/063_08_complex_case_statement.sh
./examples/063_09_complex_function_parameter_handling.sh
./examples/063_10_complex_for_loop.sh
./examples/063_11_complex_while_loop.sh
./examples/063_12_complex_eval.sh
./examples/063_13_nested_subshells.sh
./examples/063_14_complex_redirects.sh
./examples/063_15_complex_function_definition.sh
./examples/063_16_complex_test_expressions.sh
./examples/063_17_nested_brace_expansion.sh
./examples/063_18_complex_here_string.sh
./examples/063_19_complex_function_call.sh
./examples/063_20_final_complex_construct.sh
./examples/063_hard_to_parse.sh
./examples/064_01_complex_nested_subshells.sh
./examples/064_02_nested_brace_expansions.sh
./examples/064_03_complex_parameter_expansion.sh
./examples/064_04_extended_glob_patterns.sh
./examples/064_05_complex_case_statement.sh
./examples/064_06_nested_arithmetic_expressions.sh
./examples/064_07_complex_array_operations.sh
./examples/064_08_heredocs_with_variable_interpolation.sh
./examples/064_09_process_substitution_pipeline.sh
./examples/064_10_nested_function_definitions.sh
./examples/064_11_complex_test_expressions.sh
./examples/064_12_brace_expansion_nested_sequences.sh
./examples/064_13_complex_string_manipulation.sh
./examples/064_14_nested_command_substitution_arithmetic.sh
./examples/064_15_complex_pipeline_multiple_redirects.sh
./examples/064_16_function_complex_argument_handling.sh
./examples/064_17_complex_while_loop_nested_conditionals.sh
./examples/064_18_array_slicing_manipulation.sh
./examples/064_19_complex_pattern_matching_extended_globs.sh
./examples/064_20_nested_subshells_environment_variables.sh
./examples/064_21_complex_string_interpolation_multiple_variables.sh
./examples/064_22_function_returning_complex_data_structures.sh
./examples/064_23_complex_error_handling_traps.sh
./examples/064_24_advanced_parameter_expansion.sh
./examples/064_25_complex_command_chaining.sh
./examples/064_hard_to_generate.sh
./get_regressions.sh
./h2s.sh
./history2summary.sh
./rename_files.sh
./simple_ls_test.sh
./test_arithmetic.sh
./test_arithmetic_eval.sh
./test_arithmetic_simple.sh
./test_array.sh
./test_array_slice.sh
./test_array_syntax.sh
./test_cd_dot_dot.sh
./test_comment_debug.sh
./test_comment_issue.sh
./test_comprehensive.sh
./test_debug.sh
./test_dollar_identifier.sh
./test_echo_minus.sh
./test_extended.sh
./test_function.sh
./test_function_simple.sh
./test_line.sh
./test_line1.sh
./test_line1_modified.sh
./test_local_names_preserved.sh
./test_ls_debug.sh
./test_ls_glob.sh
./test_ls_star.sh
./test_ls_star_dot_sh.sh
./test_ls_working.sh
./test_minimal.sh
./test_minimal_comment.sh
./test_minimal_new.sh
./test_mixed_pipeline.sh
./test_modern_perl_signatures.sh
./test_modern_perl_signatures_advanced.sh
./test_modern_perl_signatures_simple.sh
./test_newline_filter.sh
./test_param_expansion.sh
./test_perl_generation.sh
./test_plus_assign.sh
./test_problematic_part.sh
./test_section.sh
./test_simple.sh
./test_simple_array.sh
./test_simple_ls.sh
./test_single.sh
./test_single_arithmetic.sh
./test_spaces.sh
./test_syntax_error.sh
./test_utf8.sh
./test_very_large_output.sh
./test_while.sh
./test_whitespace.sh
./www/start_test_server.sh
