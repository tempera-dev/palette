/*
 * error_status.h
 *
 *
 */

#ifndef _error_status_H_
#define _error_status_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct error_status_t error_status_t;

#include "object.h"



typedef struct error_status_t {
    int code; //numeric
    list_t *details; //nonprimitive container
    char *message; // string
    char *status; // string

    int _library_owned; // Is the library responsible for freeing this object?
} error_status_t;

__attribute__((deprecated)) error_status_t *error_status_create(
    int code,
    list_t *details,
    char *message,
    char *status
);

void error_status_free(error_status_t *error_status);

error_status_t *error_status_parseFromJSON(cJSON *error_statusJSON);

cJSON *error_status_convertToJSON(error_status_t *error_status);

#endif /* _error_status_H_ */
