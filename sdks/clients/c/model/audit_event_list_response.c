#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "audit_event_list_response.h"



static audit_event_list_response_t *audit_event_list_response_create_internal(
    list_t *events,
    char *next_page_token
    ) {
    audit_event_list_response_t *audit_event_list_response_local_var = malloc(sizeof(audit_event_list_response_t));
    if (!audit_event_list_response_local_var) {
        return NULL;
    }
    audit_event_list_response_local_var->events = events;
    audit_event_list_response_local_var->next_page_token = next_page_token;

    audit_event_list_response_local_var->_library_owned = 1;
    return audit_event_list_response_local_var;
}

__attribute__((deprecated)) audit_event_list_response_t *audit_event_list_response_create(
    list_t *events,
    char *next_page_token
    ) {
    return audit_event_list_response_create_internal (
        events,
        next_page_token
        );
}

void audit_event_list_response_free(audit_event_list_response_t *audit_event_list_response) {
    if(NULL == audit_event_list_response){
        return ;
    }
    if(audit_event_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "audit_event_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (audit_event_list_response->events) {
        list_ForEach(listEntry, audit_event_list_response->events) {
            audit_event_free(listEntry->data);
        }
        list_freeList(audit_event_list_response->events);
        audit_event_list_response->events = NULL;
    }
    if (audit_event_list_response->next_page_token) {
        free(audit_event_list_response->next_page_token);
        audit_event_list_response->next_page_token = NULL;
    }
    free(audit_event_list_response);
}

cJSON *audit_event_list_response_convertToJSON(audit_event_list_response_t *audit_event_list_response) {
    cJSON *item = cJSON_CreateObject();

    // audit_event_list_response->events
    if (!audit_event_list_response->events) {
        goto fail;
    }
    cJSON *events = cJSON_AddArrayToObject(item, "events");
    if(events == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *eventsListEntry;
    if (audit_event_list_response->events) {
    list_ForEach(eventsListEntry, audit_event_list_response->events) {
    cJSON *itemLocal = audit_event_convertToJSON(eventsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(events, itemLocal);
    }
    }


    // audit_event_list_response->next_page_token
    if(audit_event_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", audit_event_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

audit_event_list_response_t *audit_event_list_response_parseFromJSON(cJSON *audit_event_list_responseJSON){

    audit_event_list_response_t *audit_event_list_response_local_var = NULL;

    // define the local list for audit_event_list_response->events
    list_t *eventsList = NULL;

    // audit_event_list_response->events
    cJSON *events = cJSON_GetObjectItemCaseSensitive(audit_event_list_responseJSON, "events");
    if (cJSON_IsNull(events)) {
        events = NULL;
    }
    if (!events) {
        goto end;
    }


    cJSON *events_local_nonprimitive = NULL;
    if(!cJSON_IsArray(events)){
        goto end; //nonprimitive container
    }

    eventsList = list_createList();

    cJSON_ArrayForEach(events_local_nonprimitive,events )
    {
        if(!cJSON_IsObject(events_local_nonprimitive)){
            goto end;
        }
        audit_event_t *eventsItem = audit_event_parseFromJSON(events_local_nonprimitive);

        list_addElement(eventsList, eventsItem);
    }

    // audit_event_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(audit_event_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }


    audit_event_list_response_local_var = audit_event_list_response_create_internal (
        eventsList,
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL
        );

    return audit_event_list_response_local_var;
end:
    if (eventsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, eventsList) {
            audit_event_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(eventsList);
        eventsList = NULL;
    }
    return NULL;

}
