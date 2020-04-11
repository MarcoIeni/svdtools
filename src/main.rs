mod cli;
mod common;
mod interrupts;
mod mmap;
mod patch;

#[cfg(test)]
mod test_utils;

fn main() {
    cli::run();
}
