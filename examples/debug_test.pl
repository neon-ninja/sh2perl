#!/usr/bin/env perl
use strict;
use warnings;

print "=== Testing file.txt content ===\n";

my $output_2 = '';
if (open(my $fh, '<', 'file.txt')) {
    while (my $line = <$fh>) {
        $line =~ s/\r\n?/\n/g;
        $output_2 .= $line;
    }
    close($fh);
    $output_2 .= "\n" unless $output_2 =~ /\n$/;
}

print "Raw content length: " . length($output_2) . "\n";
my @sort_lines = split(/\n/, $output_2);
print "Number of lines: " . scalar(@sort_lines) . "\n";

foreach my $i (0..$#sort_lines) {
    print "Line $i: [" . $sort_lines[$i] . "]\n";
}

print "\n=== After sorting ===\n";
my @sort_sorted = sort @sort_lines;
foreach my $i (0..$#sort_sorted) {
    print "Sorted Line $i: [" . $sort_sorted[$i] . "]\n";
}

print "\n=== After uniq -c ===\n";
my @uniq_lines = grep { $_ ne '' } @sort_sorted;
my %uniq_counts;
foreach my $line (@uniq_lines) {
    $uniq_counts{$line}++;
}
my @uniq_result;
foreach my $line (keys %uniq_counts) {
    push @uniq_result, sprintf("%7d %s", $uniq_counts{$line}, $line);
}

foreach my $line (@uniq_result) {
    print "Uniq result: [$line]\n";
}
