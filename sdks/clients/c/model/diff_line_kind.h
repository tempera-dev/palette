/*
 * diff_line_kind.h
 *
 * 
 */

#ifndef _diff_line_kind_H_
#define _diff_line_kind_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct diff_line_kind_t diff_line_kind_t;


// Enum  for diff_line_kind

typedef enum { palette_api_diff_line_kind__NULL = 0, palette_api_diff_line_kind__unchanged, palette_api_diff_line_kind__added, palette_api_diff_line_kind__removed } palette_api_diff_line_kind__e;

char* diff_line_kind_diff_line_kind_ToString(palette_api_diff_line_kind__e diff_line_kind);

palette_api_diff_line_kind__e diff_line_kind_diff_line_kind_FromString(char* diff_line_kind);

cJSON *diff_line_kind_convertToJSON(palette_api_diff_line_kind__e diff_line_kind);

palette_api_diff_line_kind__e diff_line_kind_parseFromJSON(cJSON *diff_line_kindJSON);

#endif /* _diff_line_kind_H_ */

