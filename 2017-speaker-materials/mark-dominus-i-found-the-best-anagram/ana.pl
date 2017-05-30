while (<>) {
  chomp;
  my $key = join "", sort split //, $_;
  push @{$words{$key}}, $_;
}

for my $anagrams (values %words) {
  print "@$anagrams\n" if @$anagrams > 1;
}
