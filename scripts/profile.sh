scriptdir="$( dirname -- "$BASH_SOURCE"; )";

! rm "$scriptdir/callgrind"*

cargo build --release --bin mybin

valgrind --tool=callgrind \
 --callgrind-out-file="$scriptdir/callgrind.out" \
 ./target/release/mybin --scores 8500 8500

rustfilt -i "$scriptdir/callgrind.out" -o "$scriptdir/callgrind.filtered.out"
kcachegrind "$scriptdir/callgrind.filtered.out" &
