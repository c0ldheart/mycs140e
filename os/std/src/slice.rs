// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A dynamically-sized view into a contiguous sequence, `[T]`.
//!
//! Slices are a view into a block of memory represented as a pointer and a
//! length.
//!
//! ```
//! // slicing a Vec
//! let vec = vec![1, 2, 3];
//! let int_slice = &vec[..];
//! // coercing an array to a slice
//! let str_slice: &[&str] = &["one", "two", "three"];
//! ```
//!
//! Slices are either mutable or shared. The shared slice type is `&[T]`,
//! while the mutable slice type is `&mut [T]`, where `T` represents the element
//! type. For example, you can mutate the block of memory that a mutable slice
//! points to:
//!
//! ```
//! let x = &mut [1, 2, 3];
//! x[1] = 7;
//! assert_eq!(x, &[1, 7, 3]);
//! ```
//!
//! Here are some of the things this module contains:
//!
//! ## Structs
//!
//! There are several structs that are useful for slices, such as [`Iter`], which
//! represents iteration over a slice.
//!
//! ## Trait Implementations
//!
//! There are several implementations of common traits for slices. Some examples
//! include:
//!
//! * [`Clone`]
//! * [`Eq`], [`Ord`] - for slices whose element type are [`Eq`] or [`Ord`].
//! * [`Hash`] - for slices whose element type is [`Hash`].
//!
//! ## Iteration
//!
//! The slices implement `IntoIterator`. The iterator yields references to the
//! slice elements.
//!
//! ```
//! let numbers = &[0, 1, 2];
//! for n in numbers {
//!     println!("{} is a number!", n);
//! }
//! ```
//!
//! The mutable slice yields mutable references to the elements:
//!
//! ```
//! let mut scores = [7, 8, 9];
//! for score in &mut scores[..] {
//!     *score += 1;
//! }
//! ```
//!
//! This iterator yields mutable references to the slice's elements, so while
//! the element type of the slice is `i32`, the element type of the iterator is
//! `&mut i32`.
//!
//! * [`.iter`] and [`.iter_mut`] are the explicit methods to return the default
//!   iterators.
//! * Further methods that return iterators are [`.split`], [`.splitn`],
//!   [`.chunks`], [`.windows`] and more.
//!
//! *[See also the slice primitive type](../../std/primitive.slice.html).*
//!
//! [`Clone`]: ../../std/clone/trait.Clone.html
//! [`Eq`]: ../../std/cmp/trait.Eq.html
//! [`Ord`]: ../../std/cmp/trait.Ord.html
//! [`Iter`]: struct.Iter.html
//! [`Hash`]: ../../std/hash/trait.Hash.html
//! [`.iter`]: ../../std/primitive.slice.html#method.iter
//! [`.iter_mut`]: ../../std/primitive.slice.html#method.iter_mut
//! [`.split`]: ../../std/primitive.slice.html#method.split
//! [`.splitn`]: ../../std/primitive.slice.html#method.splitn
//! [`.chunks`]: ../../std/primitive.slice.html#method.chunks
//! [`.windows`]: ../../std/primitive.slice.html#method.windows
#![stable(feature = "rust1", since = "1.0.0")]

// Many of the usings in this module are only used in the test configuration.
// It's cleaner to just turn off the unused_imports warning than to fix them.
#![cfg_attr(test, allow(unused_imports, dead_code))]

// use core::cmp::Ordering::{self /*, Less */};
// use core::mem::size_of;
// use core::mem;
// use core::ptr;
// use core::slice as core_slice;

// use borrow::{Borrow, BorrowMut, ToOwned};
// use boxed::Box;
// use vec::Vec;

#[stable(feature = "rust1", since = "1.0.0")]
pub use core::slice::{Chunks, Windows};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::slice::{Iter, IterMut};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::slice::{SplitMut, ChunksMut, Split};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::slice::{SplitN, RSplitN, SplitNMut, RSplitNMut};
#[unstable(feature = "slice_rsplit", issue = "41020")]
pub use core::slice::{RSplit, RSplitMut};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::slice::{from_raw_parts, from_raw_parts_mut};
#[unstable(feature = "from_ref", issue = "45703")]
pub use core::slice::{from_ref, from_mut};
#[unstable(feature = "slice_get_slice", issue = "35729")]
pub use core::slice::SliceIndex;
// #[unstable(feature = "exact_chunks", issue = "47115")]
// pub use core::slice::{ExactChunks, ExactChunksMut};

////////////////////////////////////////////////////////////////////////////////
// Basic slice extension methods
////////////////////////////////////////////////////////////////////////////////

// // HACK(japaric) needed for the implementation of `vec!` macro during testing
// // NB see the hack module in this file for more details
// #[cfg(test)]
// pub use self::hack::into_vec;

// // HACK(japaric) needed for the implementation of `Vec::clone` during testing
// // NB see the hack module in this file for more details
// #[cfg(test)]
// pub use self::hack::to_vec;

// // HACK(japaric): With cfg(test) `impl [T]` is not available, these three
// // functions are actually methods that are in `impl [T]` but not in
// // `core::slice::SliceExt` - we need to supply these functions for the
// // `test_permutations` test
// mod hack {
//     use boxed::Box;
//     use core::mem;

//     #[cfg(test)]
//     use string::ToString;
//     use vec::Vec;

//     pub fn into_vec<T>(mut b: Box<[T]>) -> Vec<T> {
//         unsafe {
//             let xs = Vec::from_raw_parts(b.as_mut_ptr(), b.len(), b.len());
//             mem::forget(b);
//             xs
//         }
//     }

//     #[inline]
//     pub fn to_vec<T>(s: &[T]) -> Vec<T>
//         where T: Clone
//     {
//         let mut vector = Vec::with_capacity(s.len());
//         vector.extend_from_slice(s);
//         vector
//     }
// }


////////////////////////////////////////////////////////////////////////////////
// Extension traits for slices over specific kinds of data
////////////////////////////////////////////////////////////////////////////////
#[unstable(feature = "slice_concat_ext",
           reason = "trait should not have to exist",
           issue = "27747")]
/// An extension trait for concatenating slices
///
/// While this trait is unstable, the methods are stable. `SliceConcatExt` is
/// included in the [standard library prelude], so you can use [`join()`] and
/// [`concat()`] as if they existed on `[T]` itself.
///
/// [standard library prelude]: ../../std/prelude/index.html
/// [`join()`]: #tymethod.join
/// [`concat()`]: #tymethod.concat
pub trait SliceConcatExt<T: ?Sized> {
    #[unstable(feature = "slice_concat_ext",
               reason = "trait should not have to exist",
               issue = "27747")]
    /// The resulting type after concatenation
    type Output;

    /// Flattens a slice of `T` into a single value `Self::Output`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(["hello", "world"].concat(), "helloworld");
    /// assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn concat(&self) -> Self::Output;

    /// Flattens a slice of `T` into a single value `Self::Output`, placing a
    /// given separator between each.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(["hello", "world"].join(" "), "hello world");
    /// assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
    /// ```
    #[stable(feature = "rename_connect_to_join", since = "1.3.0")]
    fn join(&self, sep: &T) -> Self::Output;

    #[stable(feature = "rust1", since = "1.0.0")]
    #[deprecated(since = "1.3.0", note = "renamed to join")]
    fn connect(&self, sep: &T) -> Self::Output;
}

// #[unstable(feature = "slice_concat_ext",
//            reason = "trait should not have to exist",
//            issue = "27747")]
// impl<T: Clone, V: Borrow<[T]>> SliceConcatExt<T> for [V] {
//     type Output = Vec<T>;

//     fn concat(&self) -> Vec<T> {
//         let size = self.iter().fold(0, |acc, v| acc + v.borrow().len());
//         let mut result = Vec::with_capacity(size);
//         for v in self {
//             result.extend_from_slice(v.borrow())
//         }
//         result
//     }

//     fn join(&self, sep: &T) -> Vec<T> {
//         let size = self.iter().fold(0, |acc, v| acc + v.borrow().len());
//         let mut result = Vec::with_capacity(size + self.len());
//         let mut first = true;
//         for v in self {
//             if first {
//                 first = false
//             } else {
//                 result.push(sep.clone())
//             }
//             result.extend_from_slice(v.borrow())
//         }
//         result
//     }

//     fn connect(&self, sep: &T) -> Vec<T> {
//         self.join(sep)
//     }
// }

////////////////////////////////////////////////////////////////////////////////
// Standard trait implementations for slices
////////////////////////////////////////////////////////////////////////////////

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T> Borrow<[T]> for Vec<T> {
//     fn borrow(&self) -> &[T] {
//         &self[..]
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T> BorrowMut<[T]> for Vec<T> {
//     fn borrow_mut(&mut self) -> &mut [T] {
//         &mut self[..]
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T: Clone> ToOwned for [T] {
//     type Owned = Vec<T>;
//     #[cfg(not(test))]
//     fn to_owned(&self) -> Vec<T> {
//         self.to_vec()
//     }

//     #[cfg(test)]
//     fn to_owned(&self) -> Vec<T> {
//         hack::to_vec(self)
//     }

//     fn clone_into(&self, target: &mut Vec<T>) {
//         // drop anything in target that will not be overwritten
//         target.truncate(self.len());
//         let len = target.len();

//         // reuse the contained values' allocations/resources.
//         target.clone_from_slice(&self[..len]);

//         // target.len <= self.len due to the truncate above, so the
//         // slice here is always in-bounds.
//         target.extend_from_slice(&self[len..]);
//     }
// }

////////////////////////////////////////////////////////////////////////////////
// Sorting
////////////////////////////////////////////////////////////////////////////////

// /// Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.
// ///
// /// This is the integral subroutine of insertion sort.
// fn insert_head<T, F>(v: &mut [T], is_less: &mut F)
//     where F: FnMut(&T, &T) -> bool
// {
//     if v.len() >= 2 && is_less(&v[1], &v[0]) {
//         unsafe {
//             // There are three ways to implement insertion here:
//             //
//             // 1. Swap adjacent elements until the first one gets to its final destination.
//             //    However, this way we copy data around more than is necessary. If elements are big
//             //    structures (costly to copy), this method will be slow.
//             //
//             // 2. Iterate until the right place for the first element is found. Then shift the
//             //    elements succeeding it to make room for it and finally place it into the
//             //    remaining hole. This is a good method.
//             //
//             // 3. Copy the first element into a temporary variable. Iterate until the right place
//             //    for it is found. As we go along, copy every traversed element into the slot
//             //    preceding it. Finally, copy data from the temporary variable into the remaining
//             //    hole. This method is very good. Benchmarks demonstrated slightly better
//             //    performance than with the 2nd method.
//             //
//             // All methods were benchmarked, and the 3rd showed best results. So we chose that one.
//             let mut tmp = mem::ManuallyDrop::new(ptr::read(&v[0]));

//             // Intermediate state of the insertion process is always tracked by `hole`, which
//             // serves two purposes:
//             // 1. Protects integrity of `v` from panics in `is_less`.
//             // 2. Fills the remaining hole in `v` in the end.
//             //
//             // Panic safety:
//             //
//             // If `is_less` panics at any point during the process, `hole` will get dropped and
//             // fill the hole in `v` with `tmp`, thus ensuring that `v` still holds every object it
//             // initially held exactly once.
//             let mut hole = InsertionHole {
//                 src: &mut *tmp,
//                 dest: &mut v[1],
//             };
//             ptr::copy_nonoverlapping(&v[1], &mut v[0], 1);

//             for i in 2..v.len() {
//                 if !is_less(&v[i], &*tmp) {
//                     break;
//                 }
//                 ptr::copy_nonoverlapping(&v[i], &mut v[i - 1], 1);
//                 hole.dest = &mut v[i];
//             }
//             // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
//         }
//     }

//     // When dropped, copies from `src` into `dest`.
//     struct InsertionHole<T> {
//         src: *mut T,
//         dest: *mut T,
//     }

//     impl<T> Drop for InsertionHole<T> {
//         fn drop(&mut self) {
//             unsafe { ptr::copy_nonoverlapping(self.src, self.dest, 1); }
//         }
//     }
// }

// /// Merges non-decreasing runs `v[..mid]` and `v[mid..]` using `buf` as temporary storage, and
// /// stores the result into `v[..]`.
// ///
// /// # Safety
// ///
// /// The two slices must be non-empty and `mid` must be in bounds. Buffer `buf` must be long enough
// /// to hold a copy of the shorter slice. Also, `T` must not be a zero-sized type.
// unsafe fn merge<T, F>(v: &mut [T], mid: usize, buf: *mut T, is_less: &mut F)
//     where F: FnMut(&T, &T) -> bool
// {
//     let len = v.len();
//     let v = v.as_mut_ptr();
//     let v_mid = v.offset(mid as isize);
//     let v_end = v.offset(len as isize);

//     // The merge process first copies the shorter run into `buf`. Then it traces the newly copied
//     // run and the longer run forwards (or backwards), comparing their next unconsumed elements and
//     // copying the lesser (or greater) one into `v`.
//     //
//     // As soon as the shorter run is fully consumed, the process is done. If the longer run gets
//     // consumed first, then we must copy whatever is left of the shorter run into the remaining
//     // hole in `v`.
//     //
//     // Intermediate state of the process is always tracked by `hole`, which serves two purposes:
//     // 1. Protects integrity of `v` from panics in `is_less`.
//     // 2. Fills the remaining hole in `v` if the longer run gets consumed first.
//     //
//     // Panic safety:
//     //
//     // If `is_less` panics at any point during the process, `hole` will get dropped and fill the
//     // hole in `v` with the unconsumed range in `buf`, thus ensuring that `v` still holds every
//     // object it initially held exactly once.
//     let mut hole;

//     if mid <= len - mid {
//         // The left run is shorter.
//         ptr::copy_nonoverlapping(v, buf, mid);
//         hole = MergeHole {
//             start: buf,
//             end: buf.offset(mid as isize),
//             dest: v,
//         };

//         // Initially, these pointers point to the beginnings of their arrays.
//         let left = &mut hole.start;
//         let mut right = v_mid;
//         let out = &mut hole.dest;

//         while *left < hole.end && right < v_end {
//             // Consume the lesser side.
//             // If equal, prefer the left run to maintain stability.
//             let to_copy = if is_less(&*right, &**left) {
//                 get_and_increment(&mut right)
//             } else {
//                 get_and_increment(left)
//             };
//             ptr::copy_nonoverlapping(to_copy, get_and_increment(out), 1);
//         }
//     } else {
//         // The right run is shorter.
//         ptr::copy_nonoverlapping(v_mid, buf, len - mid);
//         hole = MergeHole {
//             start: buf,
//             end: buf.offset((len - mid) as isize),
//             dest: v_mid,
//         };

//         // Initially, these pointers point past the ends of their arrays.
//         let left = &mut hole.dest;
//         let right = &mut hole.end;
//         let mut out = v_end;

//         while v < *left && buf < *right {
//             // Consume the greater side.
//             // If equal, prefer the right run to maintain stability.
//             let to_copy = if is_less(&*right.offset(-1), &*left.offset(-1)) {
//                 decrement_and_get(left)
//             } else {
//                 decrement_and_get(right)
//             };
//             ptr::copy_nonoverlapping(to_copy, decrement_and_get(&mut out), 1);
//         }
//     }
//     // Finally, `hole` gets dropped. If the shorter run was not fully consumed, whatever remains of
//     // it will now be copied into the hole in `v`.

//     unsafe fn get_and_increment<T>(ptr: &mut *mut T) -> *mut T {
//         let old = *ptr;
//         *ptr = ptr.offset(1);
//         old
//     }

//     unsafe fn decrement_and_get<T>(ptr: &mut *mut T) -> *mut T {
//         *ptr = ptr.offset(-1);
//         *ptr
//     }

//     // When dropped, copies the range `start..end` into `dest..`.
//     struct MergeHole<T> {
//         start: *mut T,
//         end: *mut T,
//         dest: *mut T,
//     }

//     impl<T> Drop for MergeHole<T> {
//         fn drop(&mut self) {
//             // `T` is not a zero-sized type, so it's okay to divide by its size.
//             let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
//             unsafe { ptr::copy_nonoverlapping(self.start, self.dest, len); }
//         }
//     }
// }

// /// This merge sort borrows some (but not all) ideas from TimSort, which is described in detail
// /// [here](http://svn.python.org/projects/python/trunk/Objects/listsort.txt).
// ///
// /// The algorithm identifies strictly descending and non-descending subsequences, which are called
// /// natural runs. There is a stack of pending runs yet to be merged. Each newly found run is pushed
// /// onto the stack, and then some pairs of adjacent runs are merged until these two invariants are
// /// satisfied:
// ///
// /// 1. for every `i` in `1..runs.len()`: `runs[i - 1].len > runs[i].len`
// /// 2. for every `i` in `2..runs.len()`: `runs[i - 2].len > runs[i - 1].len + runs[i].len`
// ///
// /// The invariants ensure that the total running time is `O(n log n)` worst-case.
// fn merge_sort<T, F>(v: &mut [T], mut is_less: F)
//     where F: FnMut(&T, &T) -> bool
// {
//     // Slices of up to this length get sorted using insertion sort.
//     const MAX_INSERTION: usize = 20;
//     // Very short runs are extended using insertion sort to span at least this many elements.
//     const MIN_RUN: usize = 10;

//     // Sorting has no meaningful behavior on zero-sized types.
//     if size_of::<T>() == 0 {
//         return;
//     }

//     let len = v.len();

//     // Short arrays get sorted in-place via insertion sort to avoid allocations.
//     if len <= MAX_INSERTION {
//         if len >= 2 {
//             for i in (0..len-1).rev() {
//                 insert_head(&mut v[i..], &mut is_less);
//             }
//         }
//         return;
//     }

//     // Allocate a buffer to use as scratch memory. We keep the length 0 so we can keep in it
//     // shallow copies of the contents of `v` without risking the dtors running on copies if
//     // `is_less` panics. When merging two sorted runs, this buffer holds a copy of the shorter run,
//     // which will always have length at most `len / 2`.
//     let mut buf = Vec::with_capacity(len / 2);

//     // In order to identify natural runs in `v`, we traverse it backwards. That might seem like a
//     // strange decision, but consider the fact that merges more often go in the opposite direction
//     // (forwards). According to benchmarks, merging forwards is slightly faster than merging
//     // backwards. To conclude, identifying runs by traversing backwards improves performance.
//     let mut runs = vec![];
//     let mut end = len;
//     while end > 0 {
//         // Find the next natural run, and reverse it if it's strictly descending.
//         let mut start = end - 1;
//         if start > 0 {
//             start -= 1;
//             unsafe {
//                 if is_less(v.get_unchecked(start + 1), v.get_unchecked(start)) {
//                     while start > 0 && is_less(v.get_unchecked(start),
//                                                v.get_unchecked(start - 1)) {
//                         start -= 1;
//                     }
//                     v[start..end].reverse();
//                 } else {
//                     while start > 0 && !is_less(v.get_unchecked(start),
//                                                 v.get_unchecked(start - 1)) {
//                         start -= 1;
//                     }
//                 }
//             }
//         }

//         // Insert some more elements into the run if it's too short. Insertion sort is faster than
//         // merge sort on short sequences, so this significantly improves performance.
//         while start > 0 && end - start < MIN_RUN {
//             start -= 1;
//             insert_head(&mut v[start..end], &mut is_less);
//         }

//         // Push this run onto the stack.
//         runs.push(Run {
//             start,
//             len: end - start,
//         });
//         end = start;

//         // Merge some pairs of adjacent runs to satisfy the invariants.
//         while let Some(r) = collapse(&runs) {
//             let left = runs[r + 1];
//             let right = runs[r];
//             unsafe {
//                 merge(&mut v[left.start .. right.start + right.len], left.len, buf.as_mut_ptr(),
//                       &mut is_less);
//             }
//             runs[r] = Run {
//                 start: left.start,
//                 len: left.len + right.len,
//             };
//             runs.remove(r + 1);
//         }
//     }

//     // Finally, exactly one run must remain in the stack.
//     debug_assert!(runs.len() == 1 && runs[0].start == 0 && runs[0].len == len);

//     // Examines the stack of runs and identifies the next pair of runs to merge. More specifically,
//     // if `Some(r)` is returned, that means `runs[r]` and `runs[r + 1]` must be merged next. If the
//     // algorithm should continue building a new run instead, `None` is returned.
//     //
//     // TimSort is infamous for its buggy implementations, as described here:
//     // http://envisage-project.eu/timsort-specification-and-verification/
//     //
//     // The gist of the story is: we must enforce the invariants on the top four runs on the stack.
//     // Enforcing them on just top three is not sufficient to ensure that the invariants will still
//     // hold for *all* runs in the stack.
//     //
//     // This function correctly checks invariants for the top four runs. Additionally, if the top
//     // run starts at index 0, it will always demand a merge operation until the stack is fully
//     // collapsed, in order to complete the sort.
//     #[inline]
//     fn collapse(runs: &[Run]) -> Option<usize> {
//         let n = runs.len();
//         if n >= 2 && (runs[n - 1].start == 0 ||
//                       runs[n - 2].len <= runs[n - 1].len ||
//                       (n >= 3 && runs[n - 3].len <= runs[n - 2].len + runs[n - 1].len) ||
//                       (n >= 4 && runs[n - 4].len <= runs[n - 3].len + runs[n - 2].len)) {
//             if n >= 3 && runs[n - 3].len < runs[n - 1].len {
//                 Some(n - 3)
//             } else {
//                 Some(n - 2)
//             }
//         } else {
//             None
//         }
//     }

//     #[derive(Clone, Copy)]
//     struct Run {
//         start: usize,
//         len: usize,
//     }
// }
