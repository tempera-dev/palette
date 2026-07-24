/*
 * review_task_list_response.h
 *
 *
 */

#ifndef _review_task_list_response_H_
#define _review_task_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct review_task_list_response_t review_task_list_response_t;

#include "review_task.h"



typedef struct review_task_list_response_t {
    char *next_page_token; // string
    list_t *tasks; //nonprimitive container

    int _library_owned; // Is the library responsible for freeing this object?
} review_task_list_response_t;

__attribute__((deprecated)) review_task_list_response_t *review_task_list_response_create(
    char *next_page_token,
    list_t *tasks
);

void review_task_list_response_free(review_task_list_response_t *review_task_list_response);

review_task_list_response_t *review_task_list_response_parseFromJSON(cJSON *review_task_list_responseJSON);

cJSON *review_task_list_response_convertToJSON(review_task_list_response_t *review_task_list_response);

#endif /* _review_task_list_response_H_ */
