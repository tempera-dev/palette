/*
 * sampling_reason.h
 *
 * 
 */

#ifndef _sampling_reason_H_
#define _sampling_reason_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct sampling_reason_t sampling_reason_t;


// Enum  for sampling_reason

typedef enum { palette_api_sampling_reason__NULL = 0, palette_api_sampling_reason__error_trace, palette_api_sampling_reason__slow_trace, palette_api_sampling_reason__high_cost_trace, palette_api_sampling_reason__routine_sampled, palette_api_sampling_reason__routine_dropped } palette_api_sampling_reason__e;

char* sampling_reason_sampling_reason_ToString(palette_api_sampling_reason__e sampling_reason);

palette_api_sampling_reason__e sampling_reason_sampling_reason_FromString(char* sampling_reason);

cJSON *sampling_reason_convertToJSON(palette_api_sampling_reason__e sampling_reason);

palette_api_sampling_reason__e sampling_reason_parseFromJSON(cJSON *sampling_reasonJSON);

#endif /* _sampling_reason_H_ */

