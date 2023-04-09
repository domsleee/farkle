use std::io;

mod binlib;

fn main() -> Result<(), io::Error> {
    binlib::bin::run()
}