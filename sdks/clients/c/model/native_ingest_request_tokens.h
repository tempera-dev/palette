/*
 * native_ingest_request_tokens.h
 *
 *
 */

#ifndef _native_ingest_request_tokens_H_
#define _native_ingest_request_tokens_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct native_ingest_request_tokens_t native_ingest_request_tokens_t;

#include "token_counts.h"



typedef struct native_ingest_request_tokens_t {
    long cache_read; //numeric
    long input; //numeric
    long output; //numeric
    long reasoning; //numeric

    int _library_owned; // Is the library responsible for freeing this object?
} native_ingest_request_tokens_t;

__attribute__((deprecated)) native_ingest_request_tokens_t *native_ingest_request_tokens_create(
    long cache_read,
    long input,
    long output,
    long reasoning
);

void native_ingest_request_tokens_free(native_ingest_request_tokens_t *native_ingest_request_tokens);

native_ingest_request_tokens_t *native_ingest_request_tokens_parseFromJSON(cJSON *native_ingest_request_tokensJSON);

cJSON *native_ingest_request_tokens_convertToJSON(native_ingest_request_tokens_t *native_ingest_request_tokens);

#endif /* _native_ingest_request_tokens_H_ */
