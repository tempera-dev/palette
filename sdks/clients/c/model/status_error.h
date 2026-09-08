/*
 * status_error.h
 *
 *
 */

#ifndef _status_error_H_
#define _status_error_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct status_error_t status_error_t;

#include "any_type.h"

// Enum STATUS for status_error

typedef enum  { palette_api_status_error_STATUS_NULL = 0, palette_api_status_error_STATUS_CANCELLED, palette_api_status_error_STATUS_UNKNOWN, palette_api_status_error_STATUS_INVALID_ARGUMENT, palette_api_status_error_STATUS_DEADLINE_EXCEEDED, palette_api_status_error_STATUS_NOT_FOUND, palette_api_status_error_STATUS_ALREADY_EXISTS, palette_api_status_error_STATUS_PERMISSION_DENIED, palette_api_status_error_STATUS_UNAUTHENTICATED, palette_api_status_error_STATUS_RESOURCE_EXHAUSTED, palette_api_status_error_STATUS_FAILED_PRECONDITION, palette_api_status_error_STATUS_ABORTED, palette_api_status_error_STATUS_OUT_OF_RANGE, palette_api_status_error_STATUS_UNIMPLEMENTED, palette_api_status_error_STATUS_INTERNAL, palette_api_status_error_STATUS_UNAVAILABLE, palette_api_status_error_STATUS_DATA_LOSS } palette_api_status_error_STATUS_e;

char* status_error_status_ToString(palette_api_status_error_STATUS_e status);

palette_api_status_error_STATUS_e status_error_status_FromString(char* status);



typedef struct status_error_t {
    int code; //numeric
    list_t *details; //primitive container
    char *message; // string
    char *request_id; // string
    palette_api_status_error_STATUS_e status; //enum

    int _library_owned; // Is the library responsible for freeing this object?
} status_error_t;

__attribute__((deprecated)) status_error_t *status_error_create(
    int code,
    list_t *details,
    char *message,
    char *request_id,
    palette_api_status_error_STATUS_e status
);

void status_error_free(status_error_t *status_error);

status_error_t *status_error_parseFromJSON(cJSON *status_errorJSON);

cJSON *status_error_convertToJSON(status_error_t *status_error);

#endif /* _status_error_H_ */
