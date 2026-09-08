#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "status.h"



static status_t *status_create_internal(
    status_error_t *error
    ) {
    status_t *status_local_var = malloc(sizeof(status_t));
    if (!status_local_var) {
        return NULL;
    }
    status_local_var->error = error;

    status_local_var->_library_owned = 1;
    return status_local_var;
}

__attribute__((deprecated)) status_t *status_create(
    status_error_t *error
    ) {
    return status_create_internal (
        error
        );
}

void status_free(status_t *status) {
    if(NULL == status){
        return ;
    }
    if(status->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "status_free");
        return ;
    }
    listEntry_t *listEntry;
    if (status->error) {
        status_error_free(status->error);
        status->error = NULL;
    }
    free(status);
}

cJSON *status_convertToJSON(status_t *status) {
    cJSON *item = cJSON_CreateObject();

    // status->error
    if (!status->error) {
        goto fail;
    }
    cJSON *error_local_JSON = status_error_convertToJSON(status->error);
    if(error_local_JSON == NULL) {
        goto fail; // custom
    }
    cJSON_AddItemToObject(item, "error", error_local_JSON);
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

status_t *status_parseFromJSON(cJSON *statusJSON){

    status_t *status_local_var = NULL;

    // define the local variable for status->error
    status_error_t *error_local_nonprim = NULL;

    // status->error
    cJSON *error = cJSON_GetObjectItemCaseSensitive(statusJSON, "error");
    if (cJSON_IsNull(error)) {
        error = NULL;
    }
    if (!error) {
        goto end;
    }


    error_local_nonprim = status_error_parseFromJSON(error); //custom


    status_local_var = status_create_internal (
        error_local_nonprim
        );

    return status_local_var;
end:
    if (error_local_nonprim) {
        status_error_free(error_local_nonprim);
        error_local_nonprim = NULL;
    }
    return NULL;

}
