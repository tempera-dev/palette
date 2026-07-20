#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/error_response.h"
#include "../model/import_tempera_evidence_request.h"
#include "../model/tempera_evidence_receipt.h"


tempera_evidence_receipt_t*
EvalResultsAPI_evalResultsGetTemperaEvidence(apiClient_t *apiClient, char *tenant_id, char *project_id, char *kind, char *external_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


tempera_evidence_receipt_t*
EvalResultsAPI_evalResultsImportTemperaBundle(apiClient_t *apiClient, char *tenant_id, char *project_id, import_tempera_evidence_request_t *import_tempera_evidence_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


tempera_evidence_receipt_t*
EvalResultsAPI_evalResultsRecordTemperaDecision(apiClient_t *apiClient, char *tenant_id, char *project_id, import_tempera_evidence_request_t *import_tempera_evidence_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


