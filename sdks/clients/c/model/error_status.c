#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "error_status.h"



static error_status_t *error_status_create_internal(
    int code,
    list_t *details,
    char *message,
    char *status
    ) {
    error_status_t *error_status_local_var = malloc(sizeof(error_status_t));
    if (!error_status_local_var) {
        return NULL;
    }
    error_status_local_var->code = code;
    error_status_local_var->details = details;
    error_status_local_var->message = message;
    error_status_local_var->status = status;

    error_status_local_var->_library_owned = 1;
    return error_status_local_var;
}

__attribute__((deprecated)) error_status_t *error_status_create(
    int code,
    list_t *details,
    char *message,
    char *status
    ) {
    return error_status_create_internal (
        code,
        details,
        message,
        status
        );
}

void error_status_free(error_status_t *error_status) {
    if(NULL == error_status){
        return ;
    }
    if(error_status->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "error_status_free");
        return ;
    }
    listEntry_t *listEntry;
    if (error_status->details) {
        list_ForEach(listEntry, error_status->details) {
            object_free(listEntry->data);
        }
        list_freeList(error_status->details);
        error_status->details = NULL;
    }
    if (error_status->message) {
        free(error_status->message);
        error_status->message = NULL;
    }
    if (error_status->status) {
        free(error_status->status);
        error_status->status = NULL;
    }
    free(error_status);
}

cJSON *error_status_convertToJSON(error_status_t *error_status) {
    cJSON *item = cJSON_CreateObject();

    // error_status->code
    if (!error_status->code) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "code", error_status->code) == NULL) {
    goto fail; //Numeric
    }


    // error_status->details
    if (!error_status->details) {
        goto fail;
    }
    cJSON *details = cJSON_AddArrayToObject(item, "details");
    if(details == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *detailsListEntry;
    if (error_status->details) {
    list_ForEach(detailsListEntry, error_status->details) {
    cJSON *itemLocal = object_convertToJSON(detailsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(details, itemLocal);
    }
    }


    // error_status->message
    if (!error_status->message) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "message", error_status->message) == NULL) {
    goto fail; //String
    }


    // error_status->status
    if (!error_status->status) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "status", error_status->status) == NULL) {
    goto fail; //String
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

error_status_t *error_status_parseFromJSON(cJSON *error_statusJSON){

    error_status_t *error_status_local_var = NULL;

    // define the local list for error_status->details
    list_t *detailsList = NULL;

    // error_status->code
    cJSON *code = cJSON_GetObjectItemCaseSensitive(error_statusJSON, "code");
    if (cJSON_IsNull(code)) {
        code = NULL;
    }
    if (!code) {
        goto end;
    }


    if(!cJSON_IsNumber(code))
    {
    goto end; //Numeric
    }

    // error_status->details
    cJSON *details = cJSON_GetObjectItemCaseSensitive(error_statusJSON, "details");
    if (cJSON_IsNull(details)) {
        details = NULL;
    }
    if (!details) {
        goto end;
    }


    cJSON *details_local_nonprimitive = NULL;
    if(!cJSON_IsArray(details)){
        goto end; //nonprimitive container
    }

    detailsList = list_createList();

    cJSON_ArrayForEach(details_local_nonprimitive,details )
    {
        if(!cJSON_IsObject(details_local_nonprimitive)){
            goto end;
        }
        object_t *detailsItem = object_parseFromJSON(details_local_nonprimitive);

        list_addElement(detailsList, detailsItem);
    }

    // error_status->message
    cJSON *message = cJSON_GetObjectItemCaseSensitive(error_statusJSON, "message");
    if (cJSON_IsNull(message)) {
        message = NULL;
    }
    if (!message) {
        goto end;
    }


    if(!cJSON_IsString(message))
    {
    goto end; //String
    }

    // error_status->status
    cJSON *status = cJSON_GetObjectItemCaseSensitive(error_statusJSON, "status");
    if (cJSON_IsNull(status)) {
        status = NULL;
    }
    if (!status) {
        goto end;
    }


    if(!cJSON_IsString(status))
    {
    goto end; //String
    }


    error_status_local_var = error_status_create_internal (
        code->valuedouble,
        detailsList,
        strdup(message->valuestring),
        strdup(status->valuestring)
        );

    return error_status_local_var;
end:
    if (detailsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, detailsList) {
            object_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(detailsList);
        detailsList = NULL;
    }
    return NULL;

}
