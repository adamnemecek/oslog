//
//  os_log_wrapper.c
//  os_log_wrapper
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

#include "os_log_wrapper.h"

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
