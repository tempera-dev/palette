#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/connect_connector_request.h"
#include "../model/connection_link.h"
#include "../model/connection_status.h"
#include "../model/connector_list_response.h"
#include "../model/connector_skills_response.h"
#include "../model/connector_tool_list_response.h"
#include "../model/invoke_connector_request.h"
#include "../model/status.h"
#include "../model/tool_execution.h"


connection_link_t*
ConnectorsAPI_connectorsConnect(apiClient_t *apiClient, char *tenantId, char *projectId, connect_connector_request_t *connect_connector_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


connector_skills_response_t*
ConnectorsAPI_connectorsGetSkills(apiClient_t *apiClient, char *tenantId, char *projectId, char *toolkit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


tool_execution_t*
ConnectorsAPI_connectorsInvokeTool(apiClient_t *apiClient, char *tenantId, char *projectId, invoke_connector_request_t *invoke_connector_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


connector_list_response_t*
ConnectorsAPI_connectorsList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


connector_tool_list_response_t*
ConnectorsAPI_connectorsListTools(apiClient_t *apiClient, char *tenantId, char *projectId, char *toolkit, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


connection_status_t*
ConnectorsAPI_connectorsStatus(apiClient_t *apiClient, char *tenantId, char *projectId, char *toolkit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
