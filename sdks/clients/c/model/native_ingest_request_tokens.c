#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "native_ingest_request_tokens.h"



static native_ingest_request_tokens_t *native_ingest_request_tokens_create_internal(
    long cache_read,
    long input,
    long output,
    long reasoning
    ) {
    native_ingest_request_tokens_t *native_ingest_request_tokens_local_var = malloc(sizeof(native_ingest_request_tokens_t));
    if (!native_ingest_request_tokens_local_var) {
        return NULL;
    }
    native_ingest_request_tokens_local_var->cache_read = cache_read;
    native_ingest_request_tokens_local_var->input = input;
    native_ingest_request_tokens_local_var->output = output;
    native_ingest_request_tokens_local_var->reasoning = reasoning;

    native_ingest_request_tokens_local_var->_library_owned = 1;
    return native_ingest_request_tokens_local_var;
}

__attribute__((deprecated)) native_ingest_request_tokens_t *native_ingest_request_tokens_create(
    long cache_read,
    long input,
    long output,
    long reasoning
    ) {
    return native_ingest_request_tokens_create_internal (
        cache_read,
        input,
        output,
        reasoning
        );
}

void native_ingest_request_tokens_free(native_ingest_request_tokens_t *native_ingest_request_tokens) {
    if(NULL == native_ingest_request_tokens){
        return ;
    }
    if(native_ingest_request_tokens->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "native_ingest_request_tokens_free");
        return ;
    }
    listEntry_t *listEntry;
    free(native_ingest_request_tokens);
}

cJSON *native_ingest_request_tokens_convertToJSON(native_ingest_request_tokens_t *native_ingest_request_tokens) {
    cJSON *item = cJSON_CreateObject();

    // native_ingest_request_tokens->cache_read
    if (!native_ingest_request_tokens->cache_read) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "cacheRead", native_ingest_request_tokens->cache_read) == NULL) {
    goto fail; //Numeric
    }


    // native_ingest_request_tokens->input
    if (!native_ingest_request_tokens->input) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "input", native_ingest_request_tokens->input) == NULL) {
    goto fail; //Numeric
    }


    // native_ingest_request_tokens->output
    if (!native_ingest_request_tokens->output) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "output", native_ingest_request_tokens->output) == NULL) {
    goto fail; //Numeric
    }


    // native_ingest_request_tokens->reasoning
    if (!native_ingest_request_tokens->reasoning) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "reasoning", native_ingest_request_tokens->reasoning) == NULL) {
    goto fail; //Numeric
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

native_ingest_request_tokens_t *native_ingest_request_tokens_parseFromJSON(cJSON *native_ingest_request_tokensJSON){

    native_ingest_request_tokens_t *native_ingest_request_tokens_local_var = NULL;

    // native_ingest_request_tokens->cache_read
    cJSON *cache_read = cJSON_GetObjectItemCaseSensitive(native_ingest_request_tokensJSON, "cacheRead");
    if (cJSON_IsNull(cache_read)) {
        cache_read = NULL;
    }
    if (!cache_read) {
        goto end;
    }


    if(!cJSON_IsNumber(cache_read))
    {
    goto end; //Numeric
    }

    // native_ingest_request_tokens->input
    cJSON *input = cJSON_GetObjectItemCaseSensitive(native_ingest_request_tokensJSON, "input");
    if (cJSON_IsNull(input)) {
        input = NULL;
    }
    if (!input) {
        goto end;
    }


    if(!cJSON_IsNumber(input))
    {
    goto end; //Numeric
    }

    // native_ingest_request_tokens->output
    cJSON *output = cJSON_GetObjectItemCaseSensitive(native_ingest_request_tokensJSON, "output");
    if (cJSON_IsNull(output)) {
        output = NULL;
    }
    if (!output) {
        goto end;
    }


    if(!cJSON_IsNumber(output))
    {
    goto end; //Numeric
    }

    // native_ingest_request_tokens->reasoning
    cJSON *reasoning = cJSON_GetObjectItemCaseSensitive(native_ingest_request_tokensJSON, "reasoning");
    if (cJSON_IsNull(reasoning)) {
        reasoning = NULL;
    }
    if (!reasoning) {
        goto end;
    }


    if(!cJSON_IsNumber(reasoning))
    {
    goto end; //Numeric
    }


    native_ingest_request_tokens_local_var = native_ingest_request_tokens_create_internal (
        cache_read->valuedouble,
        input->valuedouble,
        output->valuedouble,
        reasoning->valuedouble
        );

    return native_ingest_request_tokens_local_var;
end:
    return NULL;

}
