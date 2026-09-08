/*
 * native_ingest_request_model.h
 *
 *
 */

#ifndef _native_ingest_request_model_H_
#define _native_ingest_request_model_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct native_ingest_request_model_t native_ingest_request_model_t;

#include "model_ref.h"



typedef struct native_ingest_request_model_t {
    char *name; // string
    char *provider; // string

    int _library_owned; // Is the library responsible for freeing this object?
} native_ingest_request_model_t;

__attribute__((deprecated)) native_ingest_request_model_t *native_ingest_request_model_create(
    char *name,
    char *provider
);

void native_ingest_request_model_free(native_ingest_request_model_t *native_ingest_request_model);

native_ingest_request_model_t *native_ingest_request_model_parseFromJSON(cJSON *native_ingest_request_modelJSON);

cJSON *native_ingest_request_model_convertToJSON(native_ingest_request_model_t *native_ingest_request_model);

#endif /* _native_ingest_request_model_H_ */
