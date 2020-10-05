//
//  lib.rs
//  oslog
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

extern crate log;
extern crate oslog_sys;

use log::{Level, Log, Metadata, Record, SetLoggerError};

use oslog_sys::_os_log_fault;
use oslog_sys::{
    OS_TRACE_TYPE_DEBUG, OS_TRACE_TYPE_RELEASE, OS_TRACE_TYPE_ERROR, OS_TRACE_TYPE_FAULT, OS_TRACE_TYPE_INFO,
};

use std::ffi::CString;

struct OsLog {
    level: Level,
}

#[repr(u32)]
enum OsLogType {
    Release = OS_TRACE_TYPE_RELEASE,
    Info = OS_TRACE_TYPE_INFO,
    Debug = OS_TRACE_TYPE_DEBUG,
    Error = OS_TRACE_TYPE_ERROR,
    Fault = OS_TRACE_TYPE_FAULT,
}

impl Log for OsLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let string = format!("{}", record.args());
            let c_string = CString::new(string).unwrap();
            unsafe {
                _os_log_fault(c_string.as_ptr());
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(OsLog { level }))
        .map(|()| log::set_max_level(level.to_level_filter()))
}
