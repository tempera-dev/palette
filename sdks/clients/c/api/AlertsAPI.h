#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/alert_decision.h"
#include "../model/evaluate_alert_request.h"
#include "../model/status.h"


alert_decision_t*
AlertsAPI_alertsEvaluate(apiClient_t *apiClient, char *tenantId, char *projectId, char *traceId, evaluate_alert_request_t *evaluate_alert_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


