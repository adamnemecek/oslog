//
//  lib.rs
//  os_log
//
//  Created by Søren Mortensen on 28/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

extern crate log;

pub mod os_log;

use log::{Record, Level, Log, Metadata, SetLoggerError};

use std::ffi::CString;

use os_log::*;

struct OsLog {
    level: Level,
}

#[repr(u32)]
enum OsLogType {
    Default = OS_LOG_TYPE_DEFAULT,
    Info = OS_LOG_TYPE_INFO,
    Debug = OS_LOG_TYPE_DEBUG,
    Error = OS_LOG_TYPE_ERROR,
    Fault = OS_LOG_TYPE_FAULT,
}

impl Log for OsLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let string = format!("{}", record.args());
            let c_string = CString::new(string).unwrap();
            unsafe { _os_log_fault(c_string.as_ptr()); }
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(OsLog { level }))
        .map(|()| log::set_max_level(level.to_level_filter()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {
        let string = format!("{}", "This is a test log from os_log");
        let c_string = CString::new(string).unwrap();
        unsafe { _os_log_fault(c_string.as_ptr()); }
    }
}
