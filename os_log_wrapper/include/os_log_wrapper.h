//
//  os_log_wrapper.h
//  os_log_wrapper
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

#ifndef oslogwrapper_h
#define oslogwrapper_h

#include <os/log.h>

void _os_log(const char *str);
void _os_log_info(const char *str);
void _os_log_debug(const char *str);
void _os_log_error(const char *str);
void _os_log_fault(const char *str);

#endif /* oslogwrapper_h */
