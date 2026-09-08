#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "native_ingest_request_parent_span_id.h"



static native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_create_internal(
    ) {
    native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_local_var = malloc(sizeof(native_ingest_request_parent_span_id_t));
    if (!native_ingest_request_parent_span_id_local_var) {
        return NULL;
    }

    native_ingest_request_parent_span_id_local_var->_library_owned = 1;
    return native_ingest_request_parent_span_id_local_var;
}

__attribute__((deprecated)) native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_create(
    ) {
    return native_ingest_request_parent_span_id_create_internal (
        );
}

void native_ingest_request_parent_span_id_free(native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id) {
    if(NULL == native_ingest_request_parent_span_id){
        return ;
    }
    if(native_ingest_request_parent_span_id->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "native_ingest_request_parent_span_id_free");
        return ;
    }
    listEntry_t *listEntry;
    free(native_ingest_request_parent_span_id);
}

cJSON *native_ingest_request_parent_span_id_convertToJSON(native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id) {
    cJSON *item = cJSON_CreateObject();
    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_parseFromJSON(cJSON *native_ingest_request_parent_span_idJSON){

    native_ingest_request_parent_span_id_t *native_ingest_request_parent_span_id_local_var = NULL;


    native_ingest_request_parent_span_id_local_var = native_ingest_request_parent_span_id_create_internal (
        );

    return native_ingest_request_parent_span_id_local_var;
end:
    return NULL;

}
