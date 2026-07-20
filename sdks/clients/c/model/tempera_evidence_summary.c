#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "tempera_evidence_summary.h"



static tempera_evidence_summary_t *tempera_evidence_summary_create_internal(
    char *experiment_id,
    char *run_id,
    char *split,
    char *suite_id,
    char *suite_version,
    char *verdict
    ) {
    tempera_evidence_summary_t *tempera_evidence_summary_local_var = malloc(sizeof(tempera_evidence_summary_t));
    if (!tempera_evidence_summary_local_var) {
        return NULL;
    }
    tempera_evidence_summary_local_var->experiment_id = experiment_id;
    tempera_evidence_summary_local_var->run_id = run_id;
    tempera_evidence_summary_local_var->split = split;
    tempera_evidence_summary_local_var->suite_id = suite_id;
    tempera_evidence_summary_local_var->suite_version = suite_version;
    tempera_evidence_summary_local_var->verdict = verdict;

    tempera_evidence_summary_local_var->_library_owned = 1;
    return tempera_evidence_summary_local_var;
}

__attribute__((deprecated)) tempera_evidence_summary_t *tempera_evidence_summary_create(
    char *experiment_id,
    char *run_id,
    char *split,
    char *suite_id,
    char *suite_version,
    char *verdict
    ) {
    return tempera_evidence_summary_create_internal (
        experiment_id,
        run_id,
        split,
        suite_id,
        suite_version,
        verdict
        );
}

void tempera_evidence_summary_free(tempera_evidence_summary_t *tempera_evidence_summary) {
    if(NULL == tempera_evidence_summary){
        return ;
    }
    if(tempera_evidence_summary->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "tempera_evidence_summary_free");
        return ;
    }
    listEntry_t *listEntry;
    if (tempera_evidence_summary->experiment_id) {
        free(tempera_evidence_summary->experiment_id);
        tempera_evidence_summary->experiment_id = NULL;
    }
    if (tempera_evidence_summary->run_id) {
        free(tempera_evidence_summary->run_id);
        tempera_evidence_summary->run_id = NULL;
    }
    if (tempera_evidence_summary->split) {
        free(tempera_evidence_summary->split);
        tempera_evidence_summary->split = NULL;
    }
    if (tempera_evidence_summary->suite_id) {
        free(tempera_evidence_summary->suite_id);
        tempera_evidence_summary->suite_id = NULL;
    }
    if (tempera_evidence_summary->suite_version) {
        free(tempera_evidence_summary->suite_version);
        tempera_evidence_summary->suite_version = NULL;
    }
    if (tempera_evidence_summary->verdict) {
        free(tempera_evidence_summary->verdict);
        tempera_evidence_summary->verdict = NULL;
    }
    free(tempera_evidence_summary);
}

cJSON *tempera_evidence_summary_convertToJSON(tempera_evidence_summary_t *tempera_evidence_summary) {
    cJSON *item = cJSON_CreateObject();

    // tempera_evidence_summary->experiment_id
    if(tempera_evidence_summary->experiment_id) {
    if(cJSON_AddStringToObject(item, "experiment_id", tempera_evidence_summary->experiment_id) == NULL) {
    goto fail; //String
    }
    }


    // tempera_evidence_summary->run_id
    if(tempera_evidence_summary->run_id) {
    if(cJSON_AddStringToObject(item, "run_id", tempera_evidence_summary->run_id) == NULL) {
    goto fail; //String
    }
    }


    // tempera_evidence_summary->split
    if(tempera_evidence_summary->split) {
    if(cJSON_AddStringToObject(item, "split", tempera_evidence_summary->split) == NULL) {
    goto fail; //String
    }
    }


    // tempera_evidence_summary->suite_id
    if(tempera_evidence_summary->suite_id) {
    if(cJSON_AddStringToObject(item, "suite_id", tempera_evidence_summary->suite_id) == NULL) {
    goto fail; //String
    }
    }


    // tempera_evidence_summary->suite_version
    if(tempera_evidence_summary->suite_version) {
    if(cJSON_AddStringToObject(item, "suite_version", tempera_evidence_summary->suite_version) == NULL) {
    goto fail; //String
    }
    }


    // tempera_evidence_summary->verdict
    if(tempera_evidence_summary->verdict) {
    if(cJSON_AddStringToObject(item, "verdict", tempera_evidence_summary->verdict) == NULL) {
    goto fail; //String
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

tempera_evidence_summary_t *tempera_evidence_summary_parseFromJSON(cJSON *tempera_evidence_summaryJSON){

    tempera_evidence_summary_t *tempera_evidence_summary_local_var = NULL;

    // tempera_evidence_summary->experiment_id
    cJSON *experiment_id = cJSON_GetObjectItemCaseSensitive(tempera_evidence_summaryJSON, "experiment_id");
    if (cJSON_IsNull(experiment_id)) {
        experiment_id = NULL;
    }
    if (experiment_id) { 
    if(!cJSON_IsString(experiment_id) && !cJSON_IsNull(experiment_id))
    {
    goto end; //String
    }
    }

    // tempera_evidence_summary->run_id
    cJSON *run_id = cJSON_GetObjectItemCaseSensitive(tempera_evidence_summaryJSON, "run_id");
    if (cJSON_IsNull(run_id)) {
        run_id = NULL;
    }
    if (run_id) { 
    if(!cJSON_IsString(run_id) && !cJSON_IsNull(run_id))
    {
    goto end; //String
    }
    }

    // tempera_evidence_summary->split
    cJSON *split = cJSON_GetObjectItemCaseSensitive(tempera_evidence_summaryJSON, "split");
    if (cJSON_IsNull(split)) {
        split = NULL;
    }
    if (split) { 
    if(!cJSON_IsString(split) && !cJSON_IsNull(split))
    {
    goto end; //String
    }
    }

    // tempera_evidence_summary->suite_id
    cJSON *suite_id = cJSON_GetObjectItemCaseSensitive(tempera_evidence_summaryJSON, "suite_id");
    if (cJSON_IsNull(suite_id)) {
        suite_id = NULL;
    }
    if (suite_id) { 
    if(!cJSON_IsString(suite_id) && !cJSON_IsNull(suite_id))
    {
    goto end; //String
    }
    }

    // tempera_evidence_summary->suite_version
    cJSON *suite_version = cJSON_GetObjectItemCaseSensitive(tempera_evidence_summaryJSON, "suite_version");
    if (cJSON_IsNull(suite_version)) {
        suite_version = NULL;
    }
    if (suite_version) { 
    if(!cJSON_IsString(suite_version) && !cJSON_IsNull(suite_version))
    {
    goto end; //String
    }
    }

    // tempera_evidence_summary->verdict
    cJSON *verdict = cJSON_GetObjectItemCaseSensitive(tempera_evidence_summaryJSON, "verdict");
    if (cJSON_IsNull(verdict)) {
        verdict = NULL;
    }
    if (verdict) { 
    if(!cJSON_IsString(verdict) && !cJSON_IsNull(verdict))
    {
    goto end; //String
    }
    }


    tempera_evidence_summary_local_var = tempera_evidence_summary_create_internal (
        experiment_id && !cJSON_IsNull(experiment_id) ? strdup(experiment_id->valuestring) : NULL,
        run_id && !cJSON_IsNull(run_id) ? strdup(run_id->valuestring) : NULL,
        split && !cJSON_IsNull(split) ? strdup(split->valuestring) : NULL,
        suite_id && !cJSON_IsNull(suite_id) ? strdup(suite_id->valuestring) : NULL,
        suite_version && !cJSON_IsNull(suite_version) ? strdup(suite_version->valuestring) : NULL,
        verdict && !cJSON_IsNull(verdict) ? strdup(verdict->valuestring) : NULL
        );

    return tempera_evidence_summary_local_var;
end:
    return NULL;

}
