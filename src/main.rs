mod convert;
mod entry;
mod prelude;

// mod feed_handling;

pub use self::prelude::Result;

fn main() -> Result<()> {
    if let Err(e) = entry::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
