/*
 * provider_secret_list_response.h
 *
 *
 */

#ifndef _provider_secret_list_response_H_
#define _provider_secret_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct provider_secret_list_response_t provider_secret_list_response_t;

#include "provider_secret_metadata.h"



typedef struct provider_secret_list_response_t {
    char *next_page_token; // string
    list_t *provider_secrets; //nonprimitive container

    int _library_owned; // Is the library responsible for freeing this object?
} provider_secret_list_response_t;

__attribute__((deprecated)) provider_secret_list_response_t *provider_secret_list_response_create(
    char *next_page_token,
    list_t *provider_secrets
);

void provider_secret_list_response_free(provider_secret_list_response_t *provider_secret_list_response);

provider_secret_list_response_t *provider_secret_list_response_parseFromJSON(cJSON *provider_secret_list_responseJSON);

cJSON *provider_secret_list_response_convertToJSON(provider_secret_list_response_t *provider_secret_list_response);

#endif /* _provider_secret_list_response_H_ */
