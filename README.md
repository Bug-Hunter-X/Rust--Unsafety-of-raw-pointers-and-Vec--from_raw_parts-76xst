# Rust: Unsafety of raw pointers and Vec::from_raw_parts

This repository demonstrates a common error in Rust when using raw pointers with `Vec`. Improper handling of raw pointers can lead to memory unsafety and program crashes. The code involves creating a `Vec`, getting a raw pointer to its data, potentially modifying the vector, and then trying to reconstruct a `Vec` from the raw pointer using `Vec::from_raw_parts`.  This approach is unsafe and prone to errors if not handled carefully.

## The Problem

The primary issue stems from the fact that `Vec`'s internal memory management might reallocate its underlying buffer during operations.  If the vector is reallocated while you hold the raw pointer `ptr`, the pointer becomes invalid.  Attempting to create a `Vec` from this invalid pointer using `Vec::from_raw_parts` is likely to result in a panic or undefined behavior.

## The Solution

The safest approach is to avoid using raw pointers with `Vec` whenever possible.  Use methods that provide safe memory management, such as borrowing (`&` and `&mut`) instead of direct pointer manipulation.