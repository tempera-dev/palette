#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "status_error.h"


char* status_error_status_ToString(palette_api_status_error_STATUS_e status) {
    char* statusArray[] =  { "NULL", "CANCELLED", "UNKNOWN", "INVALID_ARGUMENT", "DEADLINE_EXCEEDED", "NOT_FOUND", "ALREADY_EXISTS", "PERMISSION_DENIED", "UNAUTHENTICATED", "RESOURCE_EXHAUSTED", "FAILED_PRECONDITION", "ABORTED", "OUT_OF_RANGE", "UNIMPLEMENTED", "INTERNAL", "UNAVAILABLE", "DATA_LOSS" };
    return statusArray[status];
}

palette_api_status_error_STATUS_e status_error_status_FromString(char* status){
    int stringToReturn = 0;
    char *statusArray[] =  { "NULL", "CANCELLED", "UNKNOWN", "INVALID_ARGUMENT", "DEADLINE_EXCEEDED", "NOT_FOUND", "ALREADY_EXISTS", "PERMISSION_DENIED", "UNAUTHENTICATED", "RESOURCE_EXHAUSTED", "FAILED_PRECONDITION", "ABORTED", "OUT_OF_RANGE", "UNIMPLEMENTED", "INTERNAL", "UNAVAILABLE", "DATA_LOSS" };
    size_t sizeofArray = sizeof(statusArray) / sizeof(statusArray[0]);
    while(stringToReturn < sizeofArray) {
        if(strcmp(status, statusArray[stringToReturn]) == 0) {
            return stringToReturn;
        }
        stringToReturn++;
    }
    return 0;
}

static status_error_t *status_error_create_internal(
    int code,
    list_t *details,
    char *message,
    char *request_id,
    palette_api_status_error_STATUS_e status
    ) {
    status_error_t *status_error_local_var = malloc(sizeof(status_error_t));
    if (!status_error_local_var) {
        return NULL;
    }
    status_error_local_var->code = code;
    status_error_local_var->details = details;
    status_error_local_var->message = message;
    status_error_local_var->request_id = request_id;
    status_error_local_var->status = status;

    status_error_local_var->_library_owned = 1;
    return status_error_local_var;
}

__attribute__((deprecated)) status_error_t *status_error_create(
    int code,
    list_t *details,
    char *message,
    char *request_id,
    palette_api_status_error_STATUS_e status
    ) {
    return status_error_create_internal (
        code,
        details,
        message,
        request_id,
        status
        );
}

void status_error_free(status_error_t *status_error) {
    if(NULL == status_error){
        return ;
    }
    if(status_error->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "status_error_free");
        return ;
    }
    listEntry_t *listEntry;
    if (status_error->details) {
        list_ForEach(listEntry, status_error->details) {
            free(listEntry->data);
        }
        list_freeList(status_error->details);
        status_error->details = NULL;
    }
    if (status_error->message) {
        free(status_error->message);
        status_error->message = NULL;
    }
    if (status_error->request_id) {
        free(status_error->request_id);
        status_error->request_id = NULL;
    }
    free(status_error);
}

cJSON *status_error_convertToJSON(status_error_t *status_error) {
    cJSON *item = cJSON_CreateObject();

    // status_error->code
    if (!status_error->code) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "code", status_error->code) == NULL) {
    goto fail; //Numeric
    }


    // status_error->details
    if(status_error->details) {
    cJSON *details = cJSON_AddArrayToObject(item, "details");
    if(details == NULL) {
        goto fail; //primitive container
    }

    listEntry_t *detailsListEntry;
    list_ForEach(detailsListEntry, status_error->details) {
    }
    }


    // status_error->message
    if (!status_error->message) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "message", status_error->message) == NULL) {
    goto fail; //String
    }


    // status_error->request_id
    if(status_error->request_id) {
    if(cJSON_AddStringToObject(item, "requestId", status_error->request_id) == NULL) {
    goto fail; //String
    }
    }


    // status_error->status
    if (palette_api_status_error_STATUS_NULL == status_error->status) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "status", status_error_status_ToString(status_error->status)) == NULL)
    {
    goto fail; //Enum
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

status_error_t *status_error_parseFromJSON(cJSON *status_errorJSON){

    status_error_t *status_error_local_var = NULL;

    // define the local list for status_error->details
    list_t *detailsList = NULL;

    // status_error->code
    cJSON *code = cJSON_GetObjectItemCaseSensitive(status_errorJSON, "code");
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

    // status_error->details
    cJSON *details = cJSON_GetObjectItemCaseSensitive(status_errorJSON, "details");
    if (cJSON_IsNull(details)) {
        details = NULL;
    }
    if (details) {
    cJSON *details_local = NULL;
    if(!cJSON_IsArray(details)) {
        goto end;//primitive container
    }
    detailsList = list_createList();

    cJSON_ArrayForEach(details_local, details)
    {
    }
    }

    // status_error->message
    cJSON *message = cJSON_GetObjectItemCaseSensitive(status_errorJSON, "message");
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

    // status_error->request_id
    cJSON *request_id = cJSON_GetObjectItemCaseSensitive(status_errorJSON, "requestId");
    if (cJSON_IsNull(request_id)) {
        request_id = NULL;
    }
    if (request_id) {
    if(!cJSON_IsString(request_id) && !cJSON_IsNull(request_id))
    {
    goto end; //String
    }
    }

    // status_error->status
    cJSON *status = cJSON_GetObjectItemCaseSensitive(status_errorJSON, "status");
    if (cJSON_IsNull(status)) {
        status = NULL;
    }
    if (!status) {
        goto end;
    }

    palette_api_status_error_STATUS_e statusVariable;

    if(!cJSON_IsString(status))
    {
    goto end; //Enum
    }
    statusVariable = status_error_status_FromString(status->valuestring);


    status_error_local_var = status_error_create_internal (
        code->valuedouble,
        details ? detailsList : NULL,
        strdup(message->valuestring),
        request_id && !cJSON_IsNull(request_id) ? strdup(request_id->valuestring) : NULL,
        statusVariable
        );

    return status_error_local_var;
end:
    if (detailsList) {
        list_freeList(detailsList);
        detailsList = NULL;
    }
    return NULL;

}
