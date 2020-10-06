//use log::{Level, Log, Metadata, Record, SetLoggerError};

// use oslog_sys::_os_log_fault;
// use oslog_sys::{
//     OS_LOG_TYPE_DEBUG, OS_LOG_TYPE_DEFAULT, OS_LOG_TYPE_ERROR, OS_LOG_TYPE_FAULT, OS_LOG_TYPE_INFO,
// };
use oslog_sys::{
    os_log_type_t_OS_LOG_TYPE_DEBUG, os_log_type_t_OS_LOG_TYPE_DEFAULT,
    os_log_type_t_OS_LOG_TYPE_ERROR, os_log_type_t_OS_LOG_TYPE_FAULT,
    os_log_type_t_OS_LOG_TYPE_INFO,
};
use std::ffi::{CStr, CString};

// struct OsLog {
//     level: Level,
// }

#[repr(u8)]
enum OsLogType {
    Default = os_log_type_t_OS_LOG_TYPE_DEFAULT,
    Info = os_log_type_t_OS_LOG_TYPE_INFO,
    Debug = os_log_type_t_OS_LOG_TYPE_DEBUG,
    Error = os_log_type_t_OS_LOG_TYPE_ERROR,
    Fault = os_log_type_t_OS_LOG_TYPE_FAULT,
}

// impl Log for OsLog {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         metadata.level() <= self.level
//     }

//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//             let string = format!("{}", record.args());
//             let c_string = CString::new(string).unwrap();
//             unsafe {
//                 _os_log_fault(c_string.as_ptr());
//             }
//         }
//     }

//     fn flush(&self) {}
// }

// pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
//     log::set_boxed_logger(Box::new(OsLog { level }))
//         .map(|()| log::set_max_level(level.to_level_filter()))
// }

// use oslog_sys::os_log_t;

pub static POINTS_OF_INTEREST: &'static str = "PointsOfInterest";
pub static DYNAMIC_TRACING: &'static str = "DynamicTracing";
pub static DYNAMIC_STACK_TRACING: &'static str = "DynamicStackTracing";

pub struct OSLog {
    inner: oslog_sys::os_log_t,
}

impl OSLog {
    pub fn new(subsystem: &str, category: &str) -> Self {
        let c_subsystem = CString::new(subsystem).unwrap();
        let c_category = CString::new(category).unwrap();
        let inner = unsafe { oslog_sys::os_log_create(c_subsystem.as_ptr(), c_category.as_ptr()) };
        Self { inner }
    }
}

// impl std::fmt::Debug for OSLog {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&format!("OSLog({})", self.inner));

//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OSSignpostID {
    inner: oslog_sys::os_signpost_id_t,
}

impl OSSignpostID {
    pub fn new(log: &OSLog) -> Self {
        let inner = unsafe { oslog_sys::os_signpost_id_generate(log.inner) };
        Self { inner }
    }
}

// #define os_signpost_event_emit(log, event_id, name, ...)
#[inline]
pub fn os_signpost_event_emit(log: &OSLog, spid: OSSignpostID, msg: &CStr) {
    unsafe {
        oslog_sys::oslog_sys_signpost_event_emit(log.inner, spid.inner, msg.as_ptr());
    }
}

#[inline]
pub fn os_signpost_interval_begin(log: &OSLog, spid: OSSignpostID, msg: &CStr) {
    unsafe {
        oslog_sys::oslog_sys_signpost_interval_begin(log.inner, spid.inner, msg.as_ptr());
    }
}

#[inline]
pub fn os_signpost_interval_end(log: &OSLog, spid: OSSignpostID, msg: &CStr) {
    unsafe {
        oslog_sys::oslog_sys_signpost_interval_end(log.inner, spid.inner, msg.as_ptr());
    }
}

pub struct OSPoiLog {
    log: OSLog,
}

impl OSPoiLog {
    pub fn new() -> Self {
        let log = OSLog::new("com.ngrid.app", POINTS_OF_INTEREST);
        Self { log }
    }
}

fn test() {
    let log = OSLog::new("fdsa", "czx");
}
