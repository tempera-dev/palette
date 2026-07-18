#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/error_response.h"
#include "../model/search_response.h"


search_response_t*
SearchAPI_searchSpans(apiClient_t *apiClient, char *tenant_id, char *q, char *project_id, char *environment_id, char *trace_id, char *span_id, char *kind, char *status, char *model, char *tool, int *limit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


