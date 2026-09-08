#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/create_scenario_request.h"
#include "../model/list_scenarios_response.h"
#include "../model/mine_scenarios_request.h"
#include "../model/mine_scenarios_response.h"
#include "../model/scenario.h"
#include "../model/status.h"


scenario_t*
ScenariosAPI_scenariosCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_scenario_request_t *create_scenario_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


scenario_t*
ScenariosAPI_scenariosGet(apiClient_t *apiClient, char *tenantId, char *projectId, char *scenarioId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


list_scenarios_response_t*
ScenariosAPI_scenariosList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


mine_scenarios_response_t*
ScenariosAPI_scenariosMine(apiClient_t *apiClient, char *tenantId, char *projectId, mine_scenarios_request_t *mine_scenarios_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


