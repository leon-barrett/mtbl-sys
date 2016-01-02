extern crate libc;
extern crate mtbl_sys;
extern crate tempfile;

use libc::size_t;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use tempfile::TempFile;

#[test]
fn test_basic_mtbl() {
    let mut tempfile_v = TempFile::shared(2).unwrap();
    let tempfile_writer = tempfile_v.pop().unwrap();
    let tempfile_reader = tempfile_v.pop().unwrap();
    unsafe {
        // Create a simple MTBL file.
        let mut options = mtbl_sys::mtbl_writer_options_init();
        let mut writer = mtbl_sys::mtbl_writer_init_fd(tempfile_writer.as_raw_fd(),
                                                       options);
        mtbl_sys::mtbl_writer_add(writer,
                                  "key".as_bytes().as_ptr(), 3,
                                  "value".as_bytes().as_ptr(), 5);
        mtbl_sys::mtbl_writer_options_destroy(&mut options);
        mtbl_sys::mtbl_writer_destroy(&mut writer);

        // Open the MTBL file.
        let mut options = mtbl_sys::mtbl_reader_options_init();
        let mut reader = mtbl_sys::mtbl_reader_init_fd(tempfile_reader.as_raw_fd(),
                                                       options);
        let source = mtbl_sys::mtbl_reader_source(reader);
        let mut keyptr: *const u8 = ptr::null();
        let mut keylen: size_t = 0;
        let mut valptr: *const u8 = ptr::null();
        let mut vallen: size_t = 0;

        // Verify that the key/value pair are present.
        let mut iter = mtbl_sys::mtbl_source_get(source,
                                                 "key".as_bytes().as_ptr(), 3);
        let retval = mtbl_sys::mtbl_iter_next(iter,
                                              &mut keyptr,
                                              &mut keylen,
                                              &mut valptr,
                                              &mut vallen);
        assert_eq!(mtbl_sys::MtblRes::mtbl_res_success, retval);
        let result = slice::from_raw_parts(valptr, vallen).to_vec();
        assert_eq!("value".as_bytes().to_vec(), result);
        mtbl_sys::mtbl_iter_destroy(&mut iter);

        // Verify that a non-added entry is not present.
        let mut iter = mtbl_sys::mtbl_source_get(source,
                                                 "not".as_bytes().as_ptr(), 3);
        let retval = mtbl_sys::mtbl_iter_next(iter,
                                              &mut keyptr,
                                              &mut keylen,
                                              &mut valptr,
                                              &mut vallen);
        assert_eq!(mtbl_sys::MtblRes::mtbl_res_failure, retval);

        mtbl_sys::mtbl_iter_destroy(&mut iter);
        mtbl_sys::mtbl_reader_options_destroy(&mut options);
        mtbl_sys::mtbl_reader_destroy(&mut reader);
    }
}
