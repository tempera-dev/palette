#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "native_ingest_request_cost.h"



static native_ingest_request_cost_t *native_ingest_request_cost_create_internal(
    long amount_micros,
    palette_api_currency__e currency
    ) {
    native_ingest_request_cost_t *native_ingest_request_cost_local_var = malloc(sizeof(native_ingest_request_cost_t));
    if (!native_ingest_request_cost_local_var) {
        return NULL;
    }
    native_ingest_request_cost_local_var->amount_micros = amount_micros;
    native_ingest_request_cost_local_var->currency = currency;

    native_ingest_request_cost_local_var->_library_owned = 1;
    return native_ingest_request_cost_local_var;
}

__attribute__((deprecated)) native_ingest_request_cost_t *native_ingest_request_cost_create(
    long amount_micros,
    palette_api_currency__e currency
    ) {
    return native_ingest_request_cost_create_internal (
        amount_micros,
        currency
        );
}

void native_ingest_request_cost_free(native_ingest_request_cost_t *native_ingest_request_cost) {
    if(NULL == native_ingest_request_cost){
        return ;
    }
    if(native_ingest_request_cost->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "native_ingest_request_cost_free");
        return ;
    }
    listEntry_t *listEntry;
    free(native_ingest_request_cost);
}

cJSON *native_ingest_request_cost_convertToJSON(native_ingest_request_cost_t *native_ingest_request_cost) {
    cJSON *item = cJSON_CreateObject();

    // native_ingest_request_cost->amount_micros
    if (!native_ingest_request_cost->amount_micros) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "amountMicros", native_ingest_request_cost->amount_micros) == NULL) {
    goto fail; //Numeric
    }


    // native_ingest_request_cost->currency
    if (palette_api_currency__NULL == native_ingest_request_cost->currency) {
        goto fail;
    }
    cJSON *currency_local_JSON = currency_convertToJSON(native_ingest_request_cost->currency);
    if(currency_local_JSON == NULL) {
        goto fail; // custom
    }
    cJSON_AddItemToObject(item, "currency", currency_local_JSON);
    if(item->child == NULL) {
        goto fail;
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

native_ingest_request_cost_t *native_ingest_request_cost_parseFromJSON(cJSON *native_ingest_request_costJSON){

    native_ingest_request_cost_t *native_ingest_request_cost_local_var = NULL;

    // define the local variable for native_ingest_request_cost->currency
    palette_api_currency__e currency_local_nonprim = 0;

    // native_ingest_request_cost->amount_micros
    cJSON *amount_micros = cJSON_GetObjectItemCaseSensitive(native_ingest_request_costJSON, "amountMicros");
    if (cJSON_IsNull(amount_micros)) {
        amount_micros = NULL;
    }
    if (!amount_micros) {
        goto end;
    }


    if(!cJSON_IsNumber(amount_micros))
    {
    goto end; //Numeric
    }

    // native_ingest_request_cost->currency
    cJSON *currency = cJSON_GetObjectItemCaseSensitive(native_ingest_request_costJSON, "currency");
    if (cJSON_IsNull(currency)) {
        currency = NULL;
    }
    if (!currency) {
        goto end;
    }


    currency_local_nonprim = currency_parseFromJSON(currency); //custom


    native_ingest_request_cost_local_var = native_ingest_request_cost_create_internal (
        amount_micros->valuedouble,
        currency_local_nonprim
        );

    return native_ingest_request_cost_local_var;
end:
    if (currency_local_nonprim) {
        currency_local_nonprim = 0;
    }
    return NULL;

}
