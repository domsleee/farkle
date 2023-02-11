scriptdir="$( dirname -- "$BASH_SOURCE"; )";

! rm "$scriptdir/callgrind"*

cargo build --release --bin mybin

valgrind --tool=callgrind \
 --callgrind-out-file="$scriptdir/callgrind.out" \
 ./target/release/mybin --score1 7000 --score2 9900

rustfilt -i "$scriptdir/callgrind.out" -o "$scriptdir/callgrind.filtered.out"
kcachegrind "$scriptdir/callgrind.filtered.out"
