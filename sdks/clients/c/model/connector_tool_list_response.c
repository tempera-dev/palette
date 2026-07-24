#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "connector_tool_list_response.h"



static connector_tool_list_response_t *connector_tool_list_response_create_internal(
    char *next_page_token,
    list_t *tools
    ) {
    connector_tool_list_response_t *connector_tool_list_response_local_var = malloc(sizeof(connector_tool_list_response_t));
    if (!connector_tool_list_response_local_var) {
        return NULL;
    }
    connector_tool_list_response_local_var->next_page_token = next_page_token;
    connector_tool_list_response_local_var->tools = tools;

    connector_tool_list_response_local_var->_library_owned = 1;
    return connector_tool_list_response_local_var;
}

__attribute__((deprecated)) connector_tool_list_response_t *connector_tool_list_response_create(
    char *next_page_token,
    list_t *tools
    ) {
    return connector_tool_list_response_create_internal (
        next_page_token,
        tools
        );
}

void connector_tool_list_response_free(connector_tool_list_response_t *connector_tool_list_response) {
    if(NULL == connector_tool_list_response){
        return ;
    }
    if(connector_tool_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "connector_tool_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (connector_tool_list_response->next_page_token) {
        free(connector_tool_list_response->next_page_token);
        connector_tool_list_response->next_page_token = NULL;
    }
    if (connector_tool_list_response->tools) {
        list_ForEach(listEntry, connector_tool_list_response->tools) {
            connector_tool_free(listEntry->data);
        }
        list_freeList(connector_tool_list_response->tools);
        connector_tool_list_response->tools = NULL;
    }
    free(connector_tool_list_response);
}

cJSON *connector_tool_list_response_convertToJSON(connector_tool_list_response_t *connector_tool_list_response) {
    cJSON *item = cJSON_CreateObject();

    // connector_tool_list_response->next_page_token
    if(connector_tool_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", connector_tool_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }


    // connector_tool_list_response->tools
    if (!connector_tool_list_response->tools) {
        goto fail;
    }
    cJSON *tools = cJSON_AddArrayToObject(item, "tools");
    if(tools == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *toolsListEntry;
    if (connector_tool_list_response->tools) {
    list_ForEach(toolsListEntry, connector_tool_list_response->tools) {
    cJSON *itemLocal = connector_tool_convertToJSON(toolsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(tools, itemLocal);
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

connector_tool_list_response_t *connector_tool_list_response_parseFromJSON(cJSON *connector_tool_list_responseJSON){

    connector_tool_list_response_t *connector_tool_list_response_local_var = NULL;

    // define the local list for connector_tool_list_response->tools
    list_t *toolsList = NULL;

    // connector_tool_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(connector_tool_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }

    // connector_tool_list_response->tools
    cJSON *tools = cJSON_GetObjectItemCaseSensitive(connector_tool_list_responseJSON, "tools");
    if (cJSON_IsNull(tools)) {
        tools = NULL;
    }
    if (!tools) {
        goto end;
    }


    cJSON *tools_local_nonprimitive = NULL;
    if(!cJSON_IsArray(tools)){
        goto end; //nonprimitive container
    }

    toolsList = list_createList();

    cJSON_ArrayForEach(tools_local_nonprimitive,tools )
    {
        if(!cJSON_IsObject(tools_local_nonprimitive)){
            goto end;
        }
        connector_tool_t *toolsItem = connector_tool_parseFromJSON(tools_local_nonprimitive);

        list_addElement(toolsList, toolsItem);
    }


    connector_tool_list_response_local_var = connector_tool_list_response_create_internal (
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL,
        toolsList
        );

    return connector_tool_list_response_local_var;
end:
    if (toolsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, toolsList) {
            connector_tool_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(toolsList);
        toolsList = NULL;
    }
    return NULL;

}
