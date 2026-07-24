/*
 * search_span_list_response.h
 *
 *
 */

#ifndef _search_span_list_response_H_
#define _search_span_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct search_span_list_response_t search_span_list_response_t;

#include "search_hit.h"



typedef struct search_span_list_response_t {
    list_t *hits; //nonprimitive container
    char *next_page_token; // string

    int _library_owned; // Is the library responsible for freeing this object?
} search_span_list_response_t;

__attribute__((deprecated)) search_span_list_response_t *search_span_list_response_create(
    list_t *hits,
    char *next_page_token
);

void search_span_list_response_free(search_span_list_response_t *search_span_list_response);

search_span_list_response_t *search_span_list_response_parseFromJSON(cJSON *search_span_list_responseJSON);

cJSON *search_span_list_response_convertToJSON(search_span_list_response_t *search_span_list_response);

#endif /* _search_span_list_response_H_ */
