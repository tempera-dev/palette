#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "trace_list_response.h"



static trace_list_response_t *trace_list_response_create_internal(
    char *next_page_token,
    list_t *runs
    ) {
    trace_list_response_t *trace_list_response_local_var = malloc(sizeof(trace_list_response_t));
    if (!trace_list_response_local_var) {
        return NULL;
    }
    trace_list_response_local_var->next_page_token = next_page_token;
    trace_list_response_local_var->runs = runs;

    trace_list_response_local_var->_library_owned = 1;
    return trace_list_response_local_var;
}

__attribute__((deprecated)) trace_list_response_t *trace_list_response_create(
    char *next_page_token,
    list_t *runs
    ) {
    return trace_list_response_create_internal (
        next_page_token,
        runs
        );
}

void trace_list_response_free(trace_list_response_t *trace_list_response) {
    if(NULL == trace_list_response){
        return ;
    }
    if(trace_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "trace_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (trace_list_response->next_page_token) {
        free(trace_list_response->next_page_token);
        trace_list_response->next_page_token = NULL;
    }
    if (trace_list_response->runs) {
        list_ForEach(listEntry, trace_list_response->runs) {
            run_summary_free(listEntry->data);
        }
        list_freeList(trace_list_response->runs);
        trace_list_response->runs = NULL;
    }
    free(trace_list_response);
}

cJSON *trace_list_response_convertToJSON(trace_list_response_t *trace_list_response) {
    cJSON *item = cJSON_CreateObject();

    // trace_list_response->next_page_token
    if(trace_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", trace_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }


    // trace_list_response->runs
    if (!trace_list_response->runs) {
        goto fail;
    }
    cJSON *runs = cJSON_AddArrayToObject(item, "runs");
    if(runs == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *runsListEntry;
    if (trace_list_response->runs) {
    list_ForEach(runsListEntry, trace_list_response->runs) {
    cJSON *itemLocal = run_summary_convertToJSON(runsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(runs, itemLocal);
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

trace_list_response_t *trace_list_response_parseFromJSON(cJSON *trace_list_responseJSON){

    trace_list_response_t *trace_list_response_local_var = NULL;

    // define the local list for trace_list_response->runs
    list_t *runsList = NULL;

    // trace_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(trace_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }

    // trace_list_response->runs
    cJSON *runs = cJSON_GetObjectItemCaseSensitive(trace_list_responseJSON, "runs");
    if (cJSON_IsNull(runs)) {
        runs = NULL;
    }
    if (!runs) {
        goto end;
    }


    cJSON *runs_local_nonprimitive = NULL;
    if(!cJSON_IsArray(runs)){
        goto end; //nonprimitive container
    }

    runsList = list_createList();

    cJSON_ArrayForEach(runs_local_nonprimitive,runs )
    {
        if(!cJSON_IsObject(runs_local_nonprimitive)){
            goto end;
        }
        run_summary_t *runsItem = run_summary_parseFromJSON(runs_local_nonprimitive);

        list_addElement(runsList, runsItem);
    }


    trace_list_response_local_var = trace_list_response_create_internal (
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL,
        runsList
        );

    return trace_list_response_local_var;
end:
    if (runsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, runsList) {
            run_summary_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(runsList);
        runsList = NULL;
    }
    return NULL;

}
