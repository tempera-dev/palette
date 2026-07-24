#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "review_task_list_response.h"



static review_task_list_response_t *review_task_list_response_create_internal(
    char *next_page_token,
    list_t *tasks
    ) {
    review_task_list_response_t *review_task_list_response_local_var = malloc(sizeof(review_task_list_response_t));
    if (!review_task_list_response_local_var) {
        return NULL;
    }
    review_task_list_response_local_var->next_page_token = next_page_token;
    review_task_list_response_local_var->tasks = tasks;

    review_task_list_response_local_var->_library_owned = 1;
    return review_task_list_response_local_var;
}

__attribute__((deprecated)) review_task_list_response_t *review_task_list_response_create(
    char *next_page_token,
    list_t *tasks
    ) {
    return review_task_list_response_create_internal (
        next_page_token,
        tasks
        );
}

void review_task_list_response_free(review_task_list_response_t *review_task_list_response) {
    if(NULL == review_task_list_response){
        return ;
    }
    if(review_task_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "review_task_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (review_task_list_response->next_page_token) {
        free(review_task_list_response->next_page_token);
        review_task_list_response->next_page_token = NULL;
    }
    if (review_task_list_response->tasks) {
        list_ForEach(listEntry, review_task_list_response->tasks) {
            review_task_free(listEntry->data);
        }
        list_freeList(review_task_list_response->tasks);
        review_task_list_response->tasks = NULL;
    }
    free(review_task_list_response);
}

cJSON *review_task_list_response_convertToJSON(review_task_list_response_t *review_task_list_response) {
    cJSON *item = cJSON_CreateObject();

    // review_task_list_response->next_page_token
    if(review_task_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", review_task_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }


    // review_task_list_response->tasks
    if (!review_task_list_response->tasks) {
        goto fail;
    }
    cJSON *tasks = cJSON_AddArrayToObject(item, "tasks");
    if(tasks == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *tasksListEntry;
    if (review_task_list_response->tasks) {
    list_ForEach(tasksListEntry, review_task_list_response->tasks) {
    cJSON *itemLocal = review_task_convertToJSON(tasksListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(tasks, itemLocal);
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

review_task_list_response_t *review_task_list_response_parseFromJSON(cJSON *review_task_list_responseJSON){

    review_task_list_response_t *review_task_list_response_local_var = NULL;

    // define the local list for review_task_list_response->tasks
    list_t *tasksList = NULL;

    // review_task_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(review_task_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }

    // review_task_list_response->tasks
    cJSON *tasks = cJSON_GetObjectItemCaseSensitive(review_task_list_responseJSON, "tasks");
    if (cJSON_IsNull(tasks)) {
        tasks = NULL;
    }
    if (!tasks) {
        goto end;
    }


    cJSON *tasks_local_nonprimitive = NULL;
    if(!cJSON_IsArray(tasks)){
        goto end; //nonprimitive container
    }

    tasksList = list_createList();

    cJSON_ArrayForEach(tasks_local_nonprimitive,tasks )
    {
        if(!cJSON_IsObject(tasks_local_nonprimitive)){
            goto end;
        }
        review_task_t *tasksItem = review_task_parseFromJSON(tasks_local_nonprimitive);

        list_addElement(tasksList, tasksItem);
    }


    review_task_list_response_local_var = review_task_list_response_create_internal (
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL,
        tasksList
        );

    return review_task_list_response_local_var;
end:
    if (tasksList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, tasksList) {
            review_task_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(tasksList);
        tasksList = NULL;
    }
    return NULL;

}
