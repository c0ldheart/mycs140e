mod raw;
mod atag;

pub use self::atag::*;

/// The address at which the firmware loads the ATAGS.
const ATAG_BASE: usize = 0x100;

/// An iterator over the ATAGS on this system.
pub struct Atags {
    ptr: &'static raw::Atag,
    first: bool,
}

impl Atags {
    /// Returns an instance of `Atags`, an iterator over ATAGS on this system.
    pub fn get() -> Atags {
        Atags {
            ptr: unsafe { &*(ATAG_BASE as *const raw::Atag) },
            first: true,
        }
    }
}

impl Iterator for Atags {
    type Item = Atag;

    fn next(&mut self) -> Option<Atag> {
        if self.first {
            self.first = false;
            Some(self.ptr.into())
        } else {
            match self.ptr.next() {
                Some(addr_next) => {
                    self.ptr = addr_next;
                    Some(addr_next.into())
                },
                None => None,
            }
        }

    }
}
