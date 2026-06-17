mod backfill;
mod diff;
mod index;
mod read;

const READ_CHUNK: usize = 64 * 1024;

pub use backfill::backfill;
pub use diff::{PackDiff, diff_indexes};
pub use index::{FileEntry, build_file_index, store_file_index};
pub use read::{guess_content_type, read_inner_file};
