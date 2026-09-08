#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/api_key_created_response.h"
#include "../model/create_api_key_http_request.h"
#include "../model/revoked_api_key.h"
#include "../model/status.h"


api_key_created_response_t*
ApiKeysAPI_apiKeysCreate(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, create_api_key_http_request_t *create_api_key_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


revoked_api_key_t*
ApiKeysAPI_apiKeysRevoke(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, char *apiKeyId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


