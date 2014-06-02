       package Help;
#
# Author: Mark Jason Dominus (mjd@plover.com)
# http://blog.plover.com/prog/perl/Help.pm.html
# http://perl.plover.com/yak/HelpHelp/
#
#  I hereby waive all copyright and related or neighboring rights
#  together with all associated claims and causes of action with
#  respect to this work to the extent possible under the law.
#
#

        use Carp 'croak';

        sub import {
          my ($selfclass, @classes) = @_;
          for my $class (@classes) {
            push @{"$class\::ISA"}, $selfclass;
          }
        }

        sub AUTOLOAD {
          my ($bottom_class, $method) = $AUTOLOAD =~ /(.*)::(.*)/;
          my %known_method;

          my @classes = ($bottom_class);
          while (@classes) {
            my $class = shift @classes;
            next if $class eq __PACKAGE__;
            unshift @classes, @{"$class\::ISA"};
            for my $name (keys %{"$class\::"}) {
              next unless defined &{"$class\::$name"};
              $known_method{$name} ||= $class;
            }
          }

          warn "Unknown method '$method' called on object of class $bottom_class\n";
          warn "Perhaps try:\n";
          for my $name (sort keys %known_method) {
            warn "  $name " . 
              ($known_method{$name} eq $bottom_class 
               ? "" 
               : "(inherited from $known_method{$name})") . 
                "\n";
          }
          croak "Aborting";
        }

        sub help {
          $AUTOLOAD = ref($_[0]) . '::(none)';
          goto &AUTOLOAD;
        }

        sub DESTROY {} 

        1;
