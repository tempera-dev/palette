/*
 * native_ingest_request_parent_span_id.h
 *
 *
 */

#ifndef _native_ingest_request_parent_span_id_H_
#define _native_ingest_request_parent_span_id_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct native_ingest_request_parent_span_id_t native_ingest_request_parent_span_id_t;




typedef struct native_ingest_request_parent_span_id_t {

    int _library_owned; // Is the library responsible for freeing this object?
} native_ingest_request_parent_span_id_t;

__attribute__((deprecated)) native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_create(
);

void native_ingest_request_parent_span_id_free(native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id);

native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_parseFromJSON(cJSON *native_ingest_request_parent_span_idJSON);

cJSON *native_ingest_request_parent_span_id_convertToJSON(native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id);

#endif /* _native_ingest_request_parent_span_id_H_ */
