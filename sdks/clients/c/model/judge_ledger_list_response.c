#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "judge_ledger_list_response.h"



static judge_ledger_list_response_t *judge_ledger_list_response_create_internal(
    char *next_page_token,
    list_t *records
    ) {
    judge_ledger_list_response_t *judge_ledger_list_response_local_var = malloc(sizeof(judge_ledger_list_response_t));
    if (!judge_ledger_list_response_local_var) {
        return NULL;
    }
    judge_ledger_list_response_local_var->next_page_token = next_page_token;
    judge_ledger_list_response_local_var->records = records;

    judge_ledger_list_response_local_var->_library_owned = 1;
    return judge_ledger_list_response_local_var;
}

__attribute__((deprecated)) judge_ledger_list_response_t *judge_ledger_list_response_create(
    char *next_page_token,
    list_t *records
    ) {
    return judge_ledger_list_response_create_internal (
        next_page_token,
        records
        );
}

void judge_ledger_list_response_free(judge_ledger_list_response_t *judge_ledger_list_response) {
    if(NULL == judge_ledger_list_response){
        return ;
    }
    if(judge_ledger_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "judge_ledger_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (judge_ledger_list_response->next_page_token) {
        free(judge_ledger_list_response->next_page_token);
        judge_ledger_list_response->next_page_token = NULL;
    }
    if (judge_ledger_list_response->records) {
        list_ForEach(listEntry, judge_ledger_list_response->records) {
            public_judge_audit_record_free(listEntry->data);
        }
        list_freeList(judge_ledger_list_response->records);
        judge_ledger_list_response->records = NULL;
    }
    free(judge_ledger_list_response);
}

cJSON *judge_ledger_list_response_convertToJSON(judge_ledger_list_response_t *judge_ledger_list_response) {
    cJSON *item = cJSON_CreateObject();

    // judge_ledger_list_response->next_page_token
    if(judge_ledger_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", judge_ledger_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }


    // judge_ledger_list_response->records
    if (!judge_ledger_list_response->records) {
        goto fail;
    }
    cJSON *records = cJSON_AddArrayToObject(item, "records");
    if(records == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *recordsListEntry;
    if (judge_ledger_list_response->records) {
    list_ForEach(recordsListEntry, judge_ledger_list_response->records) {
    cJSON *itemLocal = public_judge_audit_record_convertToJSON(recordsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(records, itemLocal);
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

judge_ledger_list_response_t *judge_ledger_list_response_parseFromJSON(cJSON *judge_ledger_list_responseJSON){

    judge_ledger_list_response_t *judge_ledger_list_response_local_var = NULL;

    // define the local list for judge_ledger_list_response->records
    list_t *recordsList = NULL;

    // judge_ledger_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(judge_ledger_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }

    // judge_ledger_list_response->records
    cJSON *records = cJSON_GetObjectItemCaseSensitive(judge_ledger_list_responseJSON, "records");
    if (cJSON_IsNull(records)) {
        records = NULL;
    }
    if (!records) {
        goto end;
    }


    cJSON *records_local_nonprimitive = NULL;
    if(!cJSON_IsArray(records)){
        goto end; //nonprimitive container
    }

    recordsList = list_createList();

    cJSON_ArrayForEach(records_local_nonprimitive,records )
    {
        if(!cJSON_IsObject(records_local_nonprimitive)){
            goto end;
        }
        public_judge_audit_record_t *recordsItem = public_judge_audit_record_parseFromJSON(records_local_nonprimitive);

        list_addElement(recordsList, recordsItem);
    }


    judge_ledger_list_response_local_var = judge_ledger_list_response_create_internal (
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL,
        recordsList
        );

    return judge_ledger_list_response_local_var;
end:
    if (recordsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, recordsList) {
            public_judge_audit_record_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(recordsList);
        recordsList = NULL;
    }
    return NULL;

}
