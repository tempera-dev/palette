#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "native_ingest_request_model.h"



static native_ingest_request_model_t *native_ingest_request_model_create_internal(
    char *name,
    char *provider
    ) {
    native_ingest_request_model_t *native_ingest_request_model_local_var = malloc(sizeof(native_ingest_request_model_t));
    if (!native_ingest_request_model_local_var) {
        return NULL;
    }
    native_ingest_request_model_local_var->name = name;
    native_ingest_request_model_local_var->provider = provider;

    native_ingest_request_model_local_var->_library_owned = 1;
    return native_ingest_request_model_local_var;
}

__attribute__((deprecated)) native_ingest_request_model_t *native_ingest_request_model_create(
    char *name,
    char *provider
    ) {
    return native_ingest_request_model_create_internal (
        name,
        provider
        );
}

void native_ingest_request_model_free(native_ingest_request_model_t *native_ingest_request_model) {
    if(NULL == native_ingest_request_model){
        return ;
    }
    if(native_ingest_request_model->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "native_ingest_request_model_free");
        return ;
    }
    listEntry_t *listEntry;
    if (native_ingest_request_model->name) {
        free(native_ingest_request_model->name);
        native_ingest_request_model->name = NULL;
    }
    if (native_ingest_request_model->provider) {
        free(native_ingest_request_model->provider);
        native_ingest_request_model->provider = NULL;
    }
    free(native_ingest_request_model);
}

cJSON *native_ingest_request_model_convertToJSON(native_ingest_request_model_t *native_ingest_request_model) {
    cJSON *item = cJSON_CreateObject();

    // native_ingest_request_model->name
    if (!native_ingest_request_model->name) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "name", native_ingest_request_model->name) == NULL) {
    goto fail; //String
    }


    // native_ingest_request_model->provider
    if (!native_ingest_request_model->provider) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "provider", native_ingest_request_model->provider) == NULL) {
    goto fail; //String
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

native_ingest_request_model_t *native_ingest_request_model_parseFromJSON(cJSON *native_ingest_request_modelJSON){

    native_ingest_request_model_t *native_ingest_request_model_local_var = NULL;

    // native_ingest_request_model->name
    cJSON *name = cJSON_GetObjectItemCaseSensitive(native_ingest_request_modelJSON, "name");
    if (cJSON_IsNull(name)) {
        name = NULL;
    }
    if (!name) {
        goto end;
    }


    if(!cJSON_IsString(name))
    {
    goto end; //String
    }

    // native_ingest_request_model->provider
    cJSON *provider = cJSON_GetObjectItemCaseSensitive(native_ingest_request_modelJSON, "provider");
    if (cJSON_IsNull(provider)) {
        provider = NULL;
    }
    if (!provider) {
        goto end;
    }


    if(!cJSON_IsString(provider))
    {
    goto end; //String
    }


    native_ingest_request_model_local_var = native_ingest_request_model_create_internal (
        strdup(name->valuestring),
        strdup(provider->valuestring)
        );

    return native_ingest_request_model_local_var;
end:
    return NULL;

}
