/*
 * api_scope.h
 *
 * 
 */

#ifndef _api_scope_H_
#define _api_scope_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct api_scope_t api_scope_t;


// Enum  for api_scope

typedef enum { palette_api_api_scope__NULL = 0, palette_api_api_scope__trace:write, palette_api_api_scope__trace:read, palette_api_api_scope__dataset:write, palette_api_api_scope__dataset:read, palette_api_api_scope__scenario:write, palette_api_api_scope__scenario:read, palette_api_api_scope__eval:run, palette_api_api_scope__pii:unmask, palette_api_api_scope__admin } palette_api_api_scope__e;

char* api_scope_api_scope_ToString(palette_api_api_scope__e api_scope);

palette_api_api_scope__e api_scope_api_scope_FromString(char* api_scope);

cJSON *api_scope_convertToJSON(palette_api_api_scope__e api_scope);

palette_api_api_scope__e api_scope_parseFromJSON(cJSON *api_scopeJSON);

#endif /* _api_scope_H_ */

