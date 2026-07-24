#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "search_span_list_response.h"



static search_span_list_response_t *search_span_list_response_create_internal(
    list_t *hits,
    char *next_page_token
    ) {
    search_span_list_response_t *search_span_list_response_local_var = malloc(sizeof(search_span_list_response_t));
    if (!search_span_list_response_local_var) {
        return NULL;
    }
    search_span_list_response_local_var->hits = hits;
    search_span_list_response_local_var->next_page_token = next_page_token;

    search_span_list_response_local_var->_library_owned = 1;
    return search_span_list_response_local_var;
}

__attribute__((deprecated)) search_span_list_response_t *search_span_list_response_create(
    list_t *hits,
    char *next_page_token
    ) {
    return search_span_list_response_create_internal (
        hits,
        next_page_token
        );
}

void search_span_list_response_free(search_span_list_response_t *search_span_list_response) {
    if(NULL == search_span_list_response){
        return ;
    }
    if(search_span_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "search_span_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (search_span_list_response->hits) {
        list_ForEach(listEntry, search_span_list_response->hits) {
            search_hit_free(listEntry->data);
        }
        list_freeList(search_span_list_response->hits);
        search_span_list_response->hits = NULL;
    }
    if (search_span_list_response->next_page_token) {
        free(search_span_list_response->next_page_token);
        search_span_list_response->next_page_token = NULL;
    }
    free(search_span_list_response);
}

cJSON *search_span_list_response_convertToJSON(search_span_list_response_t *search_span_list_response) {
    cJSON *item = cJSON_CreateObject();

    // search_span_list_response->hits
    if (!search_span_list_response->hits) {
        goto fail;
    }
    cJSON *hits = cJSON_AddArrayToObject(item, "hits");
    if(hits == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *hitsListEntry;
    if (search_span_list_response->hits) {
    list_ForEach(hitsListEntry, search_span_list_response->hits) {
    cJSON *itemLocal = search_hit_convertToJSON(hitsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(hits, itemLocal);
    }
    }


    // search_span_list_response->next_page_token
    if(search_span_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", search_span_list_response->next_page_token) == NULL) {
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

search_span_list_response_t *search_span_list_response_parseFromJSON(cJSON *search_span_list_responseJSON){

    search_span_list_response_t *search_span_list_response_local_var = NULL;

    // define the local list for search_span_list_response->hits
    list_t *hitsList = NULL;

    // search_span_list_response->hits
    cJSON *hits = cJSON_GetObjectItemCaseSensitive(search_span_list_responseJSON, "hits");
    if (cJSON_IsNull(hits)) {
        hits = NULL;
    }
    if (!hits) {
        goto end;
    }


    cJSON *hits_local_nonprimitive = NULL;
    if(!cJSON_IsArray(hits)){
        goto end; //nonprimitive container
    }

    hitsList = list_createList();

    cJSON_ArrayForEach(hits_local_nonprimitive,hits )
    {
        if(!cJSON_IsObject(hits_local_nonprimitive)){
            goto end;
        }
        search_hit_t *hitsItem = search_hit_parseFromJSON(hits_local_nonprimitive);

        list_addElement(hitsList, hitsItem);
    }

    // search_span_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(search_span_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }


    search_span_list_response_local_var = search_span_list_response_create_internal (
        hitsList,
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL
        );

    return search_span_list_response_local_var;
end:
    if (hitsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, hitsList) {
            search_hit_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(hitsList);
        hitsList = NULL;
    }
    return NULL;

}
