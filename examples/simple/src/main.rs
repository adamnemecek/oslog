extern crate oslog;
use oslog::{os_signpost_event_emit, OSLog, OSSignpostID};

use std::ffi::{CString, CStr};
fn main() {
    let log = OSLog::new("com.ngrid.app", "Rendering");
    let spid = OSSignpostID::new(&log);

    // println!("hello worl");
    let s = CString::new("czx").unwrap();
    os_signpost_event_emit(&log, spid, &s);
}
