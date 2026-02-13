pub mod blob;
pub mod map;
pub mod traits;
pub mod unix;

pub use blob::Blob;
pub use map::View;
pub use traits::Deallocator;
pub use unix::Handle;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
