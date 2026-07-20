#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "external_eval_evidence_kind.h"


char* external_eval_evidence_kind_external_eval_evidence_kind_ToString(palette_api_external_eval_evidence_kind__e external_eval_evidence_kind) {
    char *external_eval_evidence_kindArray[] =  { "NULL", "result_bundle", "ab_decision" };
    return external_eval_evidence_kindArray[external_eval_evidence_kind];
}

palette_api_external_eval_evidence_kind__e external_eval_evidence_kind_external_eval_evidence_kind_FromString(char* external_eval_evidence_kind) {
    int stringToReturn = 0;
    char *external_eval_evidence_kindArray[] =  { "NULL", "result_bundle", "ab_decision" };
    size_t sizeofArray = sizeof(external_eval_evidence_kindArray) / sizeof(external_eval_evidence_kindArray[0]);
    while(stringToReturn < sizeofArray) {
        if(strcmp(external_eval_evidence_kind, external_eval_evidence_kindArray[stringToReturn]) == 0) {
            return stringToReturn;
        }
        stringToReturn++;
    }
    return 0;
}

cJSON *external_eval_evidence_kind_convertToJSON(palette_api_external_eval_evidence_kind__e external_eval_evidence_kind) {
    cJSON *item = cJSON_CreateObject();
    if(cJSON_AddStringToObject(item, "external_eval_evidence_kind", external_eval_evidence_kind_external_eval_evidence_kind_ToString(external_eval_evidence_kind)) == NULL) {
        goto fail;
    }
    return item;
fail:
    cJSON_Delete(item);
    return NULL;
}

palette_api_external_eval_evidence_kind__e external_eval_evidence_kind_parseFromJSON(cJSON *external_eval_evidence_kindJSON) {
    if(!cJSON_IsString(external_eval_evidence_kindJSON) || (external_eval_evidence_kindJSON->valuestring == NULL)) {
        return 0;
    }
    return external_eval_evidence_kind_external_eval_evidence_kind_FromString(external_eval_evidence_kindJSON->valuestring);
}
