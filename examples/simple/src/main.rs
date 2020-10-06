use oslog::{
    os_signpost_event_emit, os_signpost_interval_begin, os_signpost_interval_end, OSLog,
    OSSignpostID,
};

use std::ffi::CString;

fn main() {
    let log = OSLog::new("com.ngrid.app", oslog::POINTS_OF_INTEREST);
    let spid = OSSignpostID::new(&log);

    println!("hello worl");
    let s = CString::new("czx").unwrap();
    // os_signpost_event_emit(&log, spid, &s);
    os_signpost_interval_begin(&log, spid, &s);
    os_signpost_interval_end(&log, spid, &s);
}
