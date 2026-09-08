#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/judge_broker_outcome.h"
#include "../model/judge_ledger_list_response.h"
#include "../model/run_judge_eval_http_request.h"
#include "../model/status.h"


judge_broker_outcome_t*
JudgeAPI_judgeEvaluate(apiClient_t *apiClient, char *tenantId, char *projectId, run_judge_eval_http_request_t *run_judge_eval_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


judge_ledger_list_response_t*
JudgeAPI_judgeListLedger(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
