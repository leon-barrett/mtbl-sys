// Copyright (c) 2016 Leon Barrett
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.
//
//! Rust FFI interface to the [mtbl](https://github.com/farsightsec/mtbl) C
//! library for dealing with SSTables (write-once sorted map files).
//!
//! SSTables are basically constant on-disk maps, like those used by
//! [CDB](http://www.corpit.ru/mjt/tinycdb.html) (which also has [Rust
//! bindings](https://github.com/andrew-d/tinycdb-rs), except using sorted maps
//! instead of hashmaps.
//!
//! Version 0.2 of mtbl-sys covers the 0.8 version of the MTBL C library.
//!
//! # Function documentation
//!
//! For documentation about each function, see MTBL's extensive man pages, e.g.
//! `man mtbl_reader`.
//!
//! # More details about MTBL
//!
//! Quoting from the MTBL documentation:
//!
//! > mtbl is not a database library. It does not provide an updateable
//! > key-value data store, but rather exposes primitives for creating,
//! > searching and merging SSTable files. Unlike databases which use the
//! > SSTable data structure internally as part of their data store, management
//! > of SSTable files -- creation, merging, deletion, combining of search
//! > results from multiple SSTables -- is left to the discretion of the mtbl
//! > library user.
//!
//! > mtbl SSTable files consist of a sequence of data blocks containing sorted
//! > key-value pairs, where keys and values are arbitrary byte arrays. Data
//! > blocks are optionally compressed using zlib or the Snappy library. The
//! > data blocks are followed by an index block, allowing for fast searches
//! > over the keyspace.
//!
//! > The basic mtbl interface is the writer, which receives a sequence of
//! > key-value pairs in sorted order with no duplicate keys, and writes them
//! > to data blocks in the SSTable output file. An index containing offsets to
//! > data blocks and the last key in each data block is buffered in memory
//! > until the writer object is closed, at which point the index is written to
//! > the end of the SSTable file. This allows SSTable files to be written in a
//! > single pass with sequential I/O operations only.
//!
//! > Once written, SSTable files can be searched using the mtbl reader
//! > interface. Searches can retrieve key-value pairs based on an exact key
//! > match, a key prefix match, or a key range. Results are retrieved using a
//! > simple iterator interface.
//!
//! > The mtbl library also provides two utility interfaces which facilitate a
//! > sort-and-merge workflow for bulk data loading. The sorter interface
//! > receives arbitrarily ordered key-value pairs and provides them in sorted
//! > order, buffering to disk as needed. The merger interface reads from
//! > multiple SSTables simultaneously and provides the key-value pairs from
//! > the combined inputs in sorted order. Since mtbl does not allow duplicate
//! > keys in an SSTable file, both the sorter and merger interfaces require a
//! > caller-provided merge function which will be called to merge multiple
//! > values for the same key. These interfaces also make use of sequential I/O
//! > operations only.

#![crate_name = "mtbl_sys"]
#![crate_type = "lib"]
#![allow(dead_code,improper_ctypes)]

extern crate libc;
use libc::{c_char, c_int, c_uint, c_void, size_t};

/// Compression method used when writing an MTBL file.
#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub enum CompressionType {
    MTBL_COMPRESSION_NONE = 0,
    MTBL_COMPRESSION_SNAPPY = 1,
    MTBL_COMPRESSION_ZLIB = 2,
    MTBL_COMPRESSION_LZ4 = 3,
    MTBL_COMPRESSION_LZ4HC = 4,
}

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub enum MtblRes {
    mtbl_res_failure = 0,
    mtbl_res_success = 1,
}

#[repr(C)]
pub struct mtbl_iter;
#[repr(C)]
pub struct mtbl_source;

#[repr(C)]
pub struct mtbl_reader;
#[repr(C)]
pub struct mtbl_reader_options;
#[repr(C)]
pub struct mtbl_metadata;
#[repr(C)]
pub struct mtbl_writer;
#[repr(C)]
pub struct mtbl_writer_options;

#[repr(C)]
pub struct mtbl_merger;
#[repr(C)]
pub struct mtbl_merger_options;
#[repr(C)]
pub struct mtbl_fileset;
#[repr(C)]
pub struct mtbl_fileset_options;
#[repr(C)]
pub struct mtbl_sorter;
#[repr(C)]
pub struct mtbl_sorter_options;

#[link(name="mtbl")]
extern "C" {

    // iter

    pub fn mtbl_iter_destroy(iter: *mut *mut mtbl_iter);

    pub fn mtbl_iter_next(iter: *mut mtbl_iter,
                          key: *mut *const u8,
                          len_key: *mut size_t,
                          val: *mut *const u8,
                          len_val: *mut size_t)
                          -> MtblRes;

    // source

    pub fn mtbl_source_iter(source: *const mtbl_source) -> *mut mtbl_iter;

    pub fn mtbl_source_get(source: *const mtbl_source,
                           key: *const u8,
                           len_key: size_t)
                           -> *mut mtbl_iter;

    pub fn mtbl_source_get_prefix(source: *const mtbl_source,
                                  key: *const u8,
                                  len_key: size_t)
                                  -> *mut mtbl_iter;

    pub fn mtbl_source_get_range(source: *const mtbl_source,
                                 key0: *const u8,
                                 len_key0: size_t,
                                 key1: *const u8,
                                 len_key1: size_t)
                                 -> *mut mtbl_iter;

    fn mtbl_source_write(source: *const mtbl_source, writer: *mut mtbl_writer) -> MtblRes;

    // writer

    pub fn mtbl_writer_init(fname: *const c_char,
                            options: *const mtbl_writer_options)
                            -> *mut mtbl_writer;

    pub fn mtbl_writer_init_fd(fd: c_int, options: *const mtbl_writer_options) -> *mut mtbl_writer;

    pub fn mtbl_writer_destroy(writer: *mut *mut mtbl_writer);

    pub fn mtbl_writer_add(writer: *mut mtbl_writer,
                           key: *const u8,
                           len_key: size_t,
                           val: *const u8,
                           len_val: size_t)
                           -> MtblRes;

    // writer options

    pub fn mtbl_writer_options_init() -> *mut mtbl_writer_options;

    pub fn mtbl_writer_options_destroy(options: *mut *mut mtbl_writer_options);

    pub fn mtbl_writer_options_set_compression(options: *mut mtbl_writer_options,
                                               compression: CompressionType);

    pub fn mtbl_writer_options_set_block_size(options: *mut mtbl_writer_options, size: size_t);

    pub fn mtbl_writer_options_set_block_restart_interval(options: *mut mtbl_writer_options,
                                                          size: size_t);

    // reader

    pub fn mtbl_reader_init(fname: *const c_char,
                            options: *const mtbl_reader_options)
                            -> *mut mtbl_reader;

    pub fn mtbl_reader_init_fd(fd: c_int, options: *const mtbl_reader_options) -> *mut mtbl_reader;

    pub fn mtbl_reader_destroy(reader: *mut *mut mtbl_reader);

    pub fn mtbl_reader_source(reader: *mut mtbl_reader) -> *const mtbl_source;

    pub fn mtbl_reader_metadata(reader: *mut mtbl_reader) -> *const mtbl_metadata;

    // reader options

    pub fn mtbl_reader_options_init() -> *mut mtbl_reader_options;

    pub fn mtbl_reader_options_destroy(options: *mut *mut mtbl_reader_options);

    // defaults to false
    pub fn mtbl_reader_options_set_verify_checksums(options: *mut mtbl_reader_options,
                                                    verify_checksums: bool);

    // defaults to false
    pub fn mtbl_reader_options_set_madvise_random(options: *mut mtbl_reader_options,
                                                  madvise_random: bool);

    // reader metadata

    /// Byte offset in the MTBL file where the index begins.
    pub fn mtbl_metadata_index_block_offset(m: *const mtbl_metadata) -> u64;

    /// Maximum size of an uncompressed data block, see mtbl_writer(3).
    pub fn mtbl_metadata_data_block_size(m: *const mtbl_metadata) -> u64;

    /// One of the compression values allowed by mtbl_writer(3).
    pub fn mtbl_metadata_compression_algorithm(m: *const mtbl_metadata) -> CompressionType;

    /// Total number of key-value entries.
    pub fn mtbl_metadata_count_entries(m: *const mtbl_metadata) -> u64;

    /// Total number of data blocks.
    pub fn mtbl_metadata_count_data_blocks(m: *const mtbl_metadata) -> u64;

    /// Total number of bytes consumed by data blocks.
    pub fn mtbl_metadata_bytes_data_blocks(m: *const mtbl_metadata) -> u64;

    /// Total number of bytes consumed by the index.
    pub fn mtbl_metadata_bytes_index_block(m: *const mtbl_metadata) -> u64;

    /// Total number of bytes that all keys would occupy if stored end-to-end in a byte array with no delimiters.
    pub fn mtbl_metadata_bytes_keys(m: *const mtbl_metadata) -> u64;

    /// Total number of bytes that all values in the file would occupy if stored end-to-end in a byte array with no delimiters.
    pub fn mtbl_metadata_bytes_values(m: *const mtbl_metadata) -> u64;

    // merger

    pub fn mtbl_merger_init(options: *const mtbl_merger_options) -> *mut mtbl_merger;

    pub fn mtbl_merger_destroy(merger: *mut *mut mtbl_merger);

    pub fn mtbl_merger_add_source(merger: *mut mtbl_merger, source: *const mtbl_source);

    pub fn mtbl_merger_source(merger: *const mtbl_merger) -> *const mtbl_source;

    // merger options

    pub fn mtbl_merger_options_init() -> *mut mtbl_merger_options;

    pub fn mtbl_merger_options_destroy(options: *mut *mut mtbl_merger_options);

    pub fn mtbl_merger_options_set_merge_func(
            options: *mut mtbl_merger_options,
            merge_func: extern "C" fn(clos: *mut c_void,
                                      key: *const u8, len_key: size_t,
                                      val0: *const u8, len_val0: size_t,
                                      val1: *const u8, len_val1: size_t,
                                      merged_val: *mut *mut u8, len_merged_val: *mut size_t),
           clos: *mut c_void);

    // fileset

    pub fn mtbl_fileset_init(fname: *const c_char,
                             options: *const mtbl_fileset_options)
                             -> *mut mtbl_fileset;

    pub fn mtbl_fileset_destroy(fileset: *mut *mut mtbl_fileset);

    pub fn mtbl_fileset_reload(fileset: *mut mtbl_fileset);

    pub fn mtbl_fileset_reload_now(fileset: *mut mtbl_fileset);

    pub fn mtbl_fileset_source(fileset: *mut mtbl_fileset) -> *const mtbl_source;

    // fileset options

    pub fn mtbl_fileset_options_init() -> *mut mtbl_fileset_options;

    pub fn mtbl_fileset_options_destroy(options: *mut *mut mtbl_fileset_options);

    pub fn mtbl_fileset_options_set_merge_func(
            options: *mut mtbl_fileset_options,
            merge_func: extern "C" fn(clos: *mut c_void,
                                      key: *const u8, len_key: size_t,
                                      val0: *const u8, len_val0: size_t,
                                      val1: *const u8, len_val1: size_t,
                                      merged_val: *mut *mut u8, len_merged_val: *mut size_t),
            clos: *mut c_void);

    pub fn mtbl_fileset_options_set_reload_interval(options: *mut mtbl_fileset_options,
                                                    reload_interval: u32);

    // sorter

    pub fn mtbl_sorter_init(options: *const mtbl_sorter_options) -> *mut mtbl_sorter;

    pub fn mtbl_sorter_destroy(sorter: *mut *mut mtbl_sorter);

    pub fn mtbl_sorter_add(sorter: *mut mtbl_sorter,
                           key: *const u8,
                           len_key: size_t,
                           val: *const u8,
                           len_val: size_t)
                           -> MtblRes;

    pub fn mtbl_sorter_write(sorter: *mut mtbl_sorter, writer: *mut mtbl_writer) -> MtblRes;

    pub fn mtbl_sorter_iter(sorter: *mut mtbl_sorter) -> *mut mtbl_iter;

    // sorter options

    pub fn mtbl_sorter_options_init() -> *mut mtbl_sorter_options;

    pub fn mtbl_sorter_options_destroy(options: *mut *mut mtbl_sorter_options);

    pub fn mtbl_sorter_options_set_merge_func(options: *mut mtbl_sorter_options,
                                              merge_fp: extern "C" fn(clos: *mut c_void,
                                                                      key: *const u8,
                                                                      len_key: size_t,
                                                                      val0: *const u8,
                                                                      len_val0: size_t,
                                                                      val1: *const u8,
                                                                      len_val1: size_t,
                                                                      merged_val: *mut *mut u8,
                                                                      len_merged_val: *mut size_t)
                                                                     ,
                                              clos: *mut c_void);

    pub fn mtbl_sorter_options_set_temp_dir(options: *mut mtbl_sorter_options,
                                            path: *const c_char);

    pub fn mtbl_sorter_options_set_max_memory(options: *mut mtbl_sorter_options, size: size_t);

    // crc32c

    pub fn mtbl_crc32c(buffer: *const u8, length: size_t) -> u32;

    // fixed

    pub fn mtbl_fixed_encode32(dst: *mut u8, value: u32) -> size_t;

    pub fn mtbl_fixed_encode64(dst: *mut u8, value: u64) -> size_t;

    pub fn mtbl_fixed_decode32(ptr: *const u8) -> u32;

    pub fn mtbl_fixed_decode64(ptr: *const u8) -> u64;

    // varint

    pub fn mtbl_varint_length(v: u64) -> c_uint;

    pub fn mtbl_varint_length_packed(buf: *const u8, len_buf: size_t) -> c_uint;

    pub fn mtbl_varint_encode32(ptr: *mut u8, value: u32) -> size_t;

    pub fn mtbl_varint_encode64(ptr: *mut u8, value: u64) -> size_t;

    pub fn mtbl_varint_decode32(ptr: *const u8, value: *mut u32) -> size_t;

    pub fn mtbl_varint_decode64(ptr: *const u8, value: *mut u64) -> size_t;

}
