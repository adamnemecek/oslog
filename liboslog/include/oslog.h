#pragma once

#include <os/log.h>
#include <os/signpost.h>

// void _os_log(const char *str);
// void _os_log_info(const char *str);
// void _os_log_debug(const char *str);
// void _os_log_error(const char *str);
// void _os_log_fault(const char *str);

void oslog_sys_signpost_event_emit(os_log_t log, os_signpost_id_t spid, const char *str);
void oslog_sys_signpost_interval_begin(os_log_t log, os_signpost_id_t spid, const char *str);
void oslog_sys_signpost_interval_end(os_log_t log, os_signpost_id_t spid, const char *str);


