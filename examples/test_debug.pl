Running shell script: examples\003_pipeline.sh
Generated Perl code:
#!/usr/bin/env perl
use strict;
use warnings;
use File::Basename;

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

my $grep_result_1_1;
my @grep_lines_1_1 = split(/\n/, $output_1);
my @grep_filtered_1_1 = grep /\.txt$/, @grep_lines_1_1;
$grep_result_1_1 = join("\n", @grep_filtered_1_1);
$output_1 = $grep_result_1_1;

my @wc_lines_1_2 = split(/\n/, $output_1);
my $wc_line_count_1_2 = scalar(@wc_lines_1_2);
my $wc_result_1_2 = '';
$wc_result_1_2 .= "$wc_line_count_1_2 ";
$wc_result_1_2 =~ s/\s+$//;
$wc_result_1_2 .= "\n";
$output_1 = $wc_result_1_2;
print $output_1;
my $output_2;
$output_2 = '';
if (open(my $fh, '<', 'file.txt')) {
while (my $line = <$fh>) {
$line =~ s/\r\n?/\n/g; # Normalize line endings
$output_2 .= $line;
}
close($fh);
# Ensure content ends with newline to prevent line concatenation
$output_2 .= "\n" unless $output_2 =~ /\n$/;
} else {
warn "cat: file.txt: No such file or directory";
exit(1);
}

my @sort_lines_2_1 = split(/\n/, $output_2);
my @sort_sorted_2_1 = sort @sort_lines_2_1;
$output_2 = join("\n", @sort_sorted_2_1);

my @uniq_lines_2_2 = split(/\n/, $output_2);
@uniq_lines_2_2 = grep { $_ ne '' } @uniq_lines_2_2; # Filter out empty lines
my %uniq_counts_2_2;
foreach my $line (@uniq_lines_2_2) {
$uniq_counts_2_2{$line}++;
}
my @uniq_result_2_2;
foreach my $line (keys %uniq_counts_2_2) {
push @uniq_result_2_2, sprintf("%7d %s", $uniq_counts_2_2{$line}, $line);
}
$output_2 = join("\n", @uniq_result_2_2);

my @sort_lines_2_3 = split(/\n/, $output_2);
my @sort_sorted_2_3 = sort { 
    my @a_fields = grep { $_ ne '' } split(/\s+/, $a);
    my @b_fields = grep { $_ ne '' } split(/\s+/, $b);
    my $a_num = @a_fields > 0 ? $a_fields[0] : 0;
    my $b_num = @b_fields > 0 ? $b_fields[0] : 0;
    $a_num <=> $b_num || $a cmp $b;
} @sort_lines_2_3;
@sort_sorted_2_3 = reverse(@sort_sorted_2_3);
$output_2 = join("\n", @sort_sorted_2_3);
print $output_2;
my $output_3;
my @find_files_4;
sub find_files_4 {
    my ($dir, $pattern) = @_;
    if (opendir(my $dh, $dir)) {
        while (my $file = readdir($dh)) {
            next if $file eq '.' || $file eq '..';
            my $full_path = $dir eq '.' ? "./$file" : "$dir/$file";
            if (-d $full_path) {
                            } elsif ($file =~ /^$pattern$/) {
                push @find_files_4, $full_path;
            }
        }
        closedir($dh);
    }
}
find_files_4('.', '.*\.sh');
$output_3 = join("\n", @find_files_4);

my @xargs_files_3_1 = split(/\n/, $output_3);
my @xargs_matching_files_3_1;
foreach my $file (@xargs_files_3_1) {
next unless $file && -f $file;
if (open(my $fh, '<', $file)) {
my $xargs_found_3_1 = 0;
while (my $line = <$fh>) {
if ($line =~ /function/) {
$xargs_found_3_1 = 1;
last;
}
}
close($fh);
push @xargs_matching_files_3_1, $file if $xargs_found_3_1;
}
}
my $xargs_result_3_1 = join("\n", @xargs_matching_files_3_1);
$output_3 = $xargs_result_3_1;

my $set1 = "\\\\/";
my $input = $output_3;
my $tr_result_3_2 = '';
for my $char (split //, $input) {
    if (index($set1, $char) == -1) {
        $tr_result_3_2 .= $char;
    }
}
$output_3 = $tr_result_3_2;
print $output_3;
if (open(my $fh, '<', "file.txt")) {
    while (my $line = <$fh>) {
        $line =~ tr/'a'/'b'/;
        next unless $line =~ /hello/;
        print $line;
    }
close($fh);
} else {
    warn "cat: file.txt: No such file or directory";
    exit(1);
}
my $output_5;
$output_5 = '';
if (open(my $fh, '<', 'file.txt')) {
while (my $line = <$fh>) {
$line =~ s/\r\n?/\n/g; # Normalize line endings
$output_5 .= $line;
}
close($fh);
# Ensure content ends with newline to prevent line concatenation
$output_5 .= "\n" unless $output_5 =~ /\n$/;
} else {
warn "cat: file.txt: No such file or directory";
exit(1);
}

my @sort_lines_5_1 = split(/\n/, $output_5);
my @sort_sorted_5_1 = sort @sort_lines_5_1;
$output_5 = join("\n", @sort_sorted_5_1);

my $grep_result_5_2;
my @grep_lines_5_2 = split(/\n/, $output_5);
my @grep_filtered_5_2 = grep /hello/, @grep_lines_5_2;
$grep_result_5_2 = join("\n", @grep_filtered_5_2);
$output_5 = $grep_result_5_2;
print $output_5;


--- Running generated Perl code ---
74
      3 apple
      2 banana
      1 cherry.clean_dead_code.sh
.clean_dead_code_conservative.sh
.clean_dead_code_fixed.sh
.clean_dead_code_safe.sh
.clean_dead_code_safe_v2.sh
.clean_dead_code_simple.sh
.history2summary.sh
.test_function.sh
.test_function_simple.sh
.test_local_names_preserved.sh
.test_modern_perl_signatures.sh
.test_modern_perl_signatures_advanced.sh
.test_modern_perl_signatures_simple.sh
.test_perl_generation.sh