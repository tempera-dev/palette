/*
 * native_ingest_request_cost.h
 *
 *
 */

#ifndef _native_ingest_request_cost_H_
#define _native_ingest_request_cost_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct native_ingest_request_cost_t native_ingest_request_cost_t;

#include "currency.h"
#include "money.h"



typedef struct native_ingest_request_cost_t {
    long amount_micros; //numeric
    palette_api_currency__e currency; //referenced enum

    int _library_owned; // Is the library responsible for freeing this object?
} native_ingest_request_cost_t;

__attribute__((deprecated)) native_ingest_request_cost_t *native_ingest_request_cost_create(
    long amount_micros,
    palette_api_currency__e currency
);

void native_ingest_request_cost_free(native_ingest_request_cost_t *native_ingest_request_cost);

native_ingest_request_cost_t *native_ingest_request_cost_parseFromJSON(cJSON *native_ingest_request_costJSON);

cJSON *native_ingest_request_cost_convertToJSON(native_ingest_request_cost_t *native_ingest_request_cost);

#endif /* _native_ingest_request_cost_H_ */
