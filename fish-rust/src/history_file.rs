use crate::{
    history::{history_never_mmap, HistoryItem},
    path::{path_get_config_remoteness, DirRemoteness},
};
use errno::errno;
use libc::{c_void, EINTR};
use std::{os::fd::RawFd, result::IterMut};

/// History file types.
#[derive(Clone, Copy)]
pub enum HistoryFileType {
    history_type_fish_2_0,
    history_type_fish_1_x,
}

/// HistoryFileContents holds the read-only contents of a file.
pub struct HistoryFileContents {
    region: Box<MmapRegion>,

    // The memory mapped pointer and length.
    // The ptr aliases our region. The length may be slightly smaller, if there is a trailing
    // incomplete history item.
    start: usize,
    length: usize,

    // The type of the mapped file.
    // This is set at construction and not changed after.
    typ: HistoryFileType,
}

impl HistoryFileContents {
    /// Construct a history file contents from a file descriptor. The file descriptor is not closed.
    pub fn create(fd: RawFd) -> Box<HistoryFileContents> {
        todo!()
    }

    // Private constructors; use the static create() function.
    fn from_region(region: Box<MmapRegion>) -> Self {
        todo!()
    }
    // Construct from explicit data, not backed by a file. This is used in tests.
    fn from_data(data: &[u8]) -> Self {
        todo!()
    }

    /// Decode an item at a given offset.
    pub fn decode_item(&self, offset: usize) -> HistoryItem {
        todo!()
    }

    /// Support for iterating item offsets.
    /// The cursor should initially be 0.
    /// If cutoff is nonzero, skip items whose timestamp is newer than cutoff.
    /// \return the offset of the next item, or none() on end.
    pub fn offset_of_next_item(&self, cursor: &mut usize, cutoff: libc::time_t) -> Option<usize> {
        todo!()
    }

    /// Get the file type.
    pub fn typ(&self) -> HistoryFileType {
        self.typ
    }

    /// Get the size of the contents.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Return a pointer to the beginning.
    pub fn begin(&self) -> usize {
        self.address_at(0)
    }

    /// Return a pointer to one-past-the-end.
    pub fn end(&self) -> usize {
        self.address_at(self.length)
    }

    /// Access the address at a given offset.
    pub fn address_at(&self, offset: usize) -> usize {
        assert!(offset <= self.length, "Invalid offset");
        offset
    }
}

/// A type wrapping up the logic around mmap and munmap.
struct MmapRegion {}

/// Append a history item to a buffer, in preparation for outputting it to the history file.
pub fn append_history_item_to_buffer(item: &HistoryItem, buffer: &mut Vec<u8>) {
    todo!()
}

// Check if we should mmap the fd.
// Don't try mmap() on non-local filesystems.
fn should_mmap() -> bool {
    !history_never_mmap()
    &&
    // mmap only if we are known not-remote.
    path_get_config_remoteness() == DirRemoteness::local
}

// Read up to len bytes from fd into address, zeroing the rest.
// Return true on success, false on failure.
fn read_from_fd(fd: RawFd, dst: &mut [u8]) -> bool {
    let mut remaining = dst.len();
    let mut read = 0;
    while remaining > 0 {
        let mut amt =
            unsafe { libc::read(fd, (&mut dst[read]) as *mut u8 as *mut c_void, remaining) };
        if amt < 0 {
            if errno().0 != EINTR {
                return false;
            }
        } else if amt == 0 {
            break;
        } else {
            remaining -= amt as usize;
            read += amt as usize;
        }
    }
    dst[read..].fill(0u8);
    true
}

fn replace_all(s: &mut Vec<u8>, needle: &[u8], replacement: &[u8]) {
    let needle_len = needle.len();
    let replacement_len = replacement.len();
    let mut offset = 0;
    while let Some(reloffset) = s[offset..].windows(needle_len).position(|w| w == needle) {
        offset += reloffset;
        s.splice(
            offset..offset + replacement_len,
            replacement.iter().cloned(),
        );
        offset += replacement_len;
    }
}

/// Support for escaping and unescaping the nonstandard "yaml" format introduced in fish 2.0.
fn escape_yaml_fish_2_0(s: &mut Vec<u8>) {
    replace_all(s, b"\\", b"\\\\"); // replace one backslash with two
    replace_all(s, b"\n", b"\\\n"); // replace newline with backslash + literal n
}

/// This function is called frequently, so it ought to be fast.
fn unescape_yaml_fish_2_0(s: &mut Vec<u8>) {
    let mut cursor = 0;
    let mut size = s.len();
    while cursor < size {
        // Look for a backslash.
        match s[cursor..].iter().position(|c| *c == b'\\') {
            Some(relpos) => {}
            None => {
                // Either not found, or found as the last character.
            }
        }
        todo!()
    }
}
