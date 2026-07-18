#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/dataset_eval_report.h"
#include "../model/error_response.h"
#include "../model/run_deterministic_eval_request.h"
#include "../model/run_judge_dataset_eval_request.h"


dataset_eval_report_t*
EvalsAPI_evalsRunDeterministic(apiClient_t *apiClient, char *tenant_id, char *project_id, char *dataset_id, char *version_id, run_deterministic_eval_request_t *run_deterministic_eval_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


dataset_eval_report_t*
EvalsAPI_evalsRunJudge(apiClient_t *apiClient, char *tenant_id, char *project_id, char *dataset_id, char *version_id, run_judge_dataset_eval_request_t *run_judge_dataset_eval_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


