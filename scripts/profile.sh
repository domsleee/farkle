scriptdir="$( dirname -- "$BASH_SOURCE"; )";

! rm "$scriptdir/callgrind"*

cargo build --release --bin mybin

valgrind --tool=callgrind \
 --callgrind-out-file="$scriptdir/callgrind.out" \
 ./target/release/mybin --scores 7500 7500

rustfilt -i "$scriptdir/callgrind.out" -o "$scriptdir/callgrind.filtered.out"
kcachegrind "$scriptdir/callgrind.filtered.out" &
