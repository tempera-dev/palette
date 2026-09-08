/*
 * status.h
 *
 * The google.rpc.Status error envelope required of every Tempera API (AIP-193).
 */

#ifndef _status_H_
#define _status_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct status_t status_t;

#include "any_type.h"
#include "status_error.h"



typedef struct status_t {
    status_error_t *error; // custom

    int _library_owned; // Is the library responsible for freeing this object?
} status_t;

__attribute__((deprecated)) status_t *status_create(
    status_error_t *error
);

void status_free(status_t *status);

status_t *status_parseFromJSON(cJSON *statusJSON);

cJSON *status_convertToJSON(status_t *status);

#endif /* _status_H_ */
