#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/add_prompt_version_request.h"
#include "../model/create_prompt_request.h"
#include "../model/created_prompt.h"
#include "../model/prompt.h"
#include "../model/prompt_list_response.h"
#include "../model/prompt_version.h"
#include "../model/prompt_version_diff.h"
#include "../model/prompt_version_list_response.h"
#include "../model/status.h"


prompt_version_t*
PromptsAPI_promptsAddVersion(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, add_prompt_version_request_t *add_prompt_version_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


created_prompt_t*
PromptsAPI_promptsCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_prompt_request_t *create_prompt_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


prompt_version_diff_t*
PromptsAPI_promptsDiffVersions(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, char *from, char *to, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


prompt_t*
PromptsAPI_promptsGet(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


prompt_list_response_t*
PromptsAPI_promptsList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


prompt_version_list_response_t*
PromptsAPI_promptsListVersions(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
