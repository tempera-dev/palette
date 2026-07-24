#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "connector_list_response.h"



static connector_list_response_t *connector_list_response_create_internal(
    char *next_page_token,
    list_t *toolkits
    ) {
    connector_list_response_t *connector_list_response_local_var = malloc(sizeof(connector_list_response_t));
    if (!connector_list_response_local_var) {
        return NULL;
    }
    connector_list_response_local_var->next_page_token = next_page_token;
    connector_list_response_local_var->toolkits = toolkits;

    connector_list_response_local_var->_library_owned = 1;
    return connector_list_response_local_var;
}

__attribute__((deprecated)) connector_list_response_t *connector_list_response_create(
    char *next_page_token,
    list_t *toolkits
    ) {
    return connector_list_response_create_internal (
        next_page_token,
        toolkits
        );
}

void connector_list_response_free(connector_list_response_t *connector_list_response) {
    if(NULL == connector_list_response){
        return ;
    }
    if(connector_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "connector_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (connector_list_response->next_page_token) {
        free(connector_list_response->next_page_token);
        connector_list_response->next_page_token = NULL;
    }
    if (connector_list_response->toolkits) {
        list_ForEach(listEntry, connector_list_response->toolkits) {
            toolkit_free(listEntry->data);
        }
        list_freeList(connector_list_response->toolkits);
        connector_list_response->toolkits = NULL;
    }
    free(connector_list_response);
}

cJSON *connector_list_response_convertToJSON(connector_list_response_t *connector_list_response) {
    cJSON *item = cJSON_CreateObject();

    // connector_list_response->next_page_token
    if(connector_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", connector_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }


    // connector_list_response->toolkits
    if (!connector_list_response->toolkits) {
        goto fail;
    }
    cJSON *toolkits = cJSON_AddArrayToObject(item, "toolkits");
    if(toolkits == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *toolkitsListEntry;
    if (connector_list_response->toolkits) {
    list_ForEach(toolkitsListEntry, connector_list_response->toolkits) {
    cJSON *itemLocal = toolkit_convertToJSON(toolkitsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(toolkits, itemLocal);
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

connector_list_response_t *connector_list_response_parseFromJSON(cJSON *connector_list_responseJSON){

    connector_list_response_t *connector_list_response_local_var = NULL;

    // define the local list for connector_list_response->toolkits
    list_t *toolkitsList = NULL;

    // connector_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(connector_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }

    // connector_list_response->toolkits
    cJSON *toolkits = cJSON_GetObjectItemCaseSensitive(connector_list_responseJSON, "toolkits");
    if (cJSON_IsNull(toolkits)) {
        toolkits = NULL;
    }
    if (!toolkits) {
        goto end;
    }


    cJSON *toolkits_local_nonprimitive = NULL;
    if(!cJSON_IsArray(toolkits)){
        goto end; //nonprimitive container
    }

    toolkitsList = list_createList();

    cJSON_ArrayForEach(toolkits_local_nonprimitive,toolkits )
    {
        if(!cJSON_IsObject(toolkits_local_nonprimitive)){
            goto end;
        }
        toolkit_t *toolkitsItem = toolkit_parseFromJSON(toolkits_local_nonprimitive);

        list_addElement(toolkitsList, toolkitsItem);
    }


    connector_list_response_local_var = connector_list_response_create_internal (
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL,
        toolkitsList
        );

    return connector_list_response_local_var;
end:
    if (toolkitsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, toolkitsList) {
            toolkit_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(toolkitsList);
        toolkitsList = NULL;
    }
    return NULL;

}
