/*
 * trace_list_response.h
 *
 *
 */

#ifndef _trace_list_response_H_
#define _trace_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct trace_list_response_t trace_list_response_t;

#include "run_summary.h"



typedef struct trace_list_response_t {
    char *next_page_token; // string
    list_t *runs; //nonprimitive container

    int _library_owned; // Is the library responsible for freeing this object?
} trace_list_response_t;

__attribute__((deprecated)) trace_list_response_t *trace_list_response_create(
    char *next_page_token,
    list_t *runs
);

void trace_list_response_free(trace_list_response_t *trace_list_response);

trace_list_response_t *trace_list_response_parseFromJSON(cJSON *trace_list_responseJSON);

cJSON *trace_list_response_convertToJSON(trace_list_response_t *trace_list_response);

#endif /* _trace_list_response_H_ */
