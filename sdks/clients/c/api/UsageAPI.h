#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/status.h"
#include "../model/usage_summary.h"


usage_summary_t*
UsageAPI_usageGetSummary(apiClient_t *apiClient, char *tenantId, char *projectId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


