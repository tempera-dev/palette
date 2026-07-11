/*
 * span_status.h
 *
 * 
 */

#ifndef _span_status_H_
#define _span_status_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct span_status_t span_status_t;


// Enum  for span_status

typedef enum { palette_api_span_status__NULL = 0, palette_api_span_status__ok, palette_api_span_status__error, palette_api_span_status__unset } palette_api_span_status__e;

char* span_status_span_status_ToString(palette_api_span_status__e span_status);

palette_api_span_status__e span_status_span_status_FromString(char* span_status);

cJSON *span_status_convertToJSON(palette_api_span_status__e span_status);

palette_api_span_status__e span_status_parseFromJSON(cJSON *span_statusJSON);

#endif /* _span_status_H_ */

