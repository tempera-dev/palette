/*
 * audit_event_list_response.h
 *
 *
 */

#ifndef _audit_event_list_response_H_
#define _audit_event_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct audit_event_list_response_t audit_event_list_response_t;

#include "audit_event.h"



typedef struct audit_event_list_response_t {
    list_t *events; //nonprimitive container
    char *next_page_token; // string

    int _library_owned; // Is the library responsible for freeing this object?
} audit_event_list_response_t;

__attribute__((deprecated)) audit_event_list_response_t *audit_event_list_response_create(
    list_t *events,
    char *next_page_token
);

void audit_event_list_response_free(audit_event_list_response_t *audit_event_list_response);

audit_event_list_response_t *audit_event_list_response_parseFromJSON(cJSON *audit_event_list_responseJSON);

cJSON *audit_event_list_response_convertToJSON(audit_event_list_response_t *audit_event_list_response);

#endif /* _audit_event_list_response_H_ */
