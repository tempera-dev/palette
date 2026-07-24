/*
 * connector_tool_list_response.h
 *
 *
 */

#ifndef _connector_tool_list_response_H_
#define _connector_tool_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct connector_tool_list_response_t connector_tool_list_response_t;

#include "connector_tool.h"



typedef struct connector_tool_list_response_t {
    char *next_page_token; // string
    list_t *tools; //nonprimitive container

    int _library_owned; // Is the library responsible for freeing this object?
} connector_tool_list_response_t;

__attribute__((deprecated)) connector_tool_list_response_t *connector_tool_list_response_create(
    char *next_page_token,
    list_t *tools
);

void connector_tool_list_response_free(connector_tool_list_response_t *connector_tool_list_response);

connector_tool_list_response_t *connector_tool_list_response_parseFromJSON(cJSON *connector_tool_list_responseJSON);

cJSON *connector_tool_list_response_convertToJSON(connector_tool_list_response_t *connector_tool_list_response);

#endif /* _connector_tool_list_response_H_ */
