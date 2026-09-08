#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/search_span_list_response.h"
#include "../model/status.h"


search_span_list_response_t*
SearchAPI_searchSpans(apiClient_t *apiClient, char *tenantId, char *q, char *projectId, char *environmentId, char *traceId, char *spanId, char *kind, char *status, char *model, char *tool, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
