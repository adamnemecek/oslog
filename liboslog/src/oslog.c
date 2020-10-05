//
//  oslog.c
//  liboslog
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

#include "oslog.h"

void _os_log(const char *str) {
    os_log(OS_LOG_DEFAULT, "%{public}s", str);
}

void _os_log_info(const char *str) {
    os_log_info(OS_LOG_DEFAULT, "%{public}s", str);
}

void _os_log_debug(const char *str) {
    os_log_debug(OS_LOG_DEFAULT, "%{public}s", str);
}

void _os_log_error(const char *str) {
    os_log_error(OS_LOG_DEFAULT, "%{public}s", str);
}

void _os_log_fault(const char *str) {
    os_log_fault(OS_LOG_DEFAULT, "%{public}s", str);
}


// OS_ENUM(os_signpost_type, uint8_t,
//     OS_SIGNPOST_EVENT           = 0x00,
//     OS_SIGNPOST_INTERVAL_BEGIN  = 0x01,
//     OS_SIGNPOST_INTERVAL_END    = 0x02,
// );

// #define os_signpost_event_emit(log, event_id, name, ...)

// #define os_signpost_interval_begin(log, interval_id, name, ...)

// https://developer.apple.com/documentation/os/os_signpost_interval_end?language=occ

// func os_signpost(_ type: OSSignpostType, dso: UnsafeRawPointer = #dsohandle, log: OSLog, name: StaticString, signpostID: OSSignpostID = .exclusive)
// #define os_signpost_interval_end(log, interval_id, name, ...)