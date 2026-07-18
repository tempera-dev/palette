#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/audit_event.h"
#include "../model/error_response.h"


list_t*
AuditAPI_auditList(apiClient_t *apiClient, char *tenant_id, char *project_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


