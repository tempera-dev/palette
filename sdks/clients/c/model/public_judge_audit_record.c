#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "public_judge_audit_record.h"



static public_judge_audit_record_t *public_judge_audit_record_create_internal(
    int cached,
    money_t *charged_cost,
    char *created_at,
    char *evaluator_id,
    char *judge_call_id,
    char *model,
    char *project_id,
    char *request_hash,
    char *response_hash,
    double score,
    char *tenant_id
    ) {
    public_judge_audit_record_t *public_judge_audit_record_local_var = malloc(sizeof(public_judge_audit_record_t));
    if (!public_judge_audit_record_local_var) {
        return NULL;
    }
    public_judge_audit_record_local_var->cached = cached;
    public_judge_audit_record_local_var->charged_cost = charged_cost;
    public_judge_audit_record_local_var->created_at = created_at;
    public_judge_audit_record_local_var->evaluator_id = evaluator_id;
    public_judge_audit_record_local_var->judge_call_id = judge_call_id;
    public_judge_audit_record_local_var->model = model;
    public_judge_audit_record_local_var->project_id = project_id;
    public_judge_audit_record_local_var->request_hash = request_hash;
    public_judge_audit_record_local_var->response_hash = response_hash;
    public_judge_audit_record_local_var->score = score;
    public_judge_audit_record_local_var->tenant_id = tenant_id;

    public_judge_audit_record_local_var->_library_owned = 1;
    return public_judge_audit_record_local_var;
}

__attribute__((deprecated)) public_judge_audit_record_t *public_judge_audit_record_create(
    int cached,
    money_t *charged_cost,
    char *created_at,
    char *evaluator_id,
    char *judge_call_id,
    char *model,
    char *project_id,
    char *request_hash,
    char *response_hash,
    double score,
    char *tenant_id
    ) {
    return public_judge_audit_record_create_internal (
        cached,
        charged_cost,
        created_at,
        evaluator_id,
        judge_call_id,
        model,
        project_id,
        request_hash,
        response_hash,
        score,
        tenant_id
        );
}

void public_judge_audit_record_free(public_judge_audit_record_t *public_judge_audit_record) {
    if(NULL == public_judge_audit_record){
        return ;
    }
    if(public_judge_audit_record->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "public_judge_audit_record_free");
        return ;
    }
    listEntry_t *listEntry;
    if (public_judge_audit_record->charged_cost) {
        money_free(public_judge_audit_record->charged_cost);
        public_judge_audit_record->charged_cost = NULL;
    }
    if (public_judge_audit_record->created_at) {
        free(public_judge_audit_record->created_at);
        public_judge_audit_record->created_at = NULL;
    }
    if (public_judge_audit_record->evaluator_id) {
        free(public_judge_audit_record->evaluator_id);
        public_judge_audit_record->evaluator_id = NULL;
    }
    if (public_judge_audit_record->judge_call_id) {
        free(public_judge_audit_record->judge_call_id);
        public_judge_audit_record->judge_call_id = NULL;
    }
    if (public_judge_audit_record->model) {
        free(public_judge_audit_record->model);
        public_judge_audit_record->model = NULL;
    }
    if (public_judge_audit_record->project_id) {
        free(public_judge_audit_record->project_id);
        public_judge_audit_record->project_id = NULL;
    }
    if (public_judge_audit_record->request_hash) {
        free(public_judge_audit_record->request_hash);
        public_judge_audit_record->request_hash = NULL;
    }
    if (public_judge_audit_record->response_hash) {
        free(public_judge_audit_record->response_hash);
        public_judge_audit_record->response_hash = NULL;
    }
    if (public_judge_audit_record->tenant_id) {
        free(public_judge_audit_record->tenant_id);
        public_judge_audit_record->tenant_id = NULL;
    }
    free(public_judge_audit_record);
}

cJSON *public_judge_audit_record_convertToJSON(public_judge_audit_record_t *public_judge_audit_record) {
    cJSON *item = cJSON_CreateObject();

    // public_judge_audit_record->cached
    if (!public_judge_audit_record->cached) {
        goto fail;
    }
    if(cJSON_AddBoolToObject(item, "cached", public_judge_audit_record->cached) == NULL) {
    goto fail; //Bool
    }


    // public_judge_audit_record->charged_cost
    if (!public_judge_audit_record->charged_cost) {
        goto fail;
    }
    cJSON *charged_cost_local_JSON = money_convertToJSON(public_judge_audit_record->charged_cost);
    if(charged_cost_local_JSON == NULL) {
    goto fail; //model
    }
    cJSON_AddItemToObject(item, "charged_cost", charged_cost_local_JSON);
    if(item->child == NULL) {
    goto fail;
    }


    // public_judge_audit_record->created_at
    if (!public_judge_audit_record->created_at) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "created_at", public_judge_audit_record->created_at) == NULL) {
    goto fail; //Date-Time
    }


    // public_judge_audit_record->evaluator_id
    if (!public_judge_audit_record->evaluator_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "evaluator_id", public_judge_audit_record->evaluator_id) == NULL) {
    goto fail; //String
    }


    // public_judge_audit_record->judge_call_id
    if (!public_judge_audit_record->judge_call_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "judge_call_id", public_judge_audit_record->judge_call_id) == NULL) {
    goto fail; //String
    }


    // public_judge_audit_record->model
    if (!public_judge_audit_record->model) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "model", public_judge_audit_record->model) == NULL) {
    goto fail; //String
    }


    // public_judge_audit_record->project_id
    if (!public_judge_audit_record->project_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "project_id", public_judge_audit_record->project_id) == NULL) {
    goto fail; //String
    }


    // public_judge_audit_record->request_hash
    if (!public_judge_audit_record->request_hash) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "request_hash", public_judge_audit_record->request_hash) == NULL) {
    goto fail; //String
    }


    // public_judge_audit_record->response_hash
    if (!public_judge_audit_record->response_hash) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "response_hash", public_judge_audit_record->response_hash) == NULL) {
    goto fail; //String
    }


    // public_judge_audit_record->score
    if (!public_judge_audit_record->score) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "score", public_judge_audit_record->score) == NULL) {
    goto fail; //Numeric
    }


    // public_judge_audit_record->tenant_id
    if (!public_judge_audit_record->tenant_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "tenant_id", public_judge_audit_record->tenant_id) == NULL) {
    goto fail; //String
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

public_judge_audit_record_t *public_judge_audit_record_parseFromJSON(cJSON *public_judge_audit_recordJSON){

    public_judge_audit_record_t *public_judge_audit_record_local_var = NULL;

    // define the local variable for public_judge_audit_record->charged_cost
    money_t *charged_cost_local_nonprim = NULL;

    // public_judge_audit_record->cached
    cJSON *cached = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "cached");
    if (cJSON_IsNull(cached)) {
        cached = NULL;
    }
    if (!cached) {
        goto end;
    }

    
    if(!cJSON_IsBool(cached))
    {
    goto end; //Bool
    }

    // public_judge_audit_record->charged_cost
    cJSON *charged_cost = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "charged_cost");
    if (cJSON_IsNull(charged_cost)) {
        charged_cost = NULL;
    }
    if (!charged_cost) {
        goto end;
    }

    
    charged_cost_local_nonprim = money_parseFromJSON(charged_cost); //nonprimitive

    // public_judge_audit_record->created_at
    cJSON *created_at = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "created_at");
    if (cJSON_IsNull(created_at)) {
        created_at = NULL;
    }
    if (!created_at) {
        goto end;
    }

    
    if(!cJSON_IsString(created_at) && !cJSON_IsNull(created_at))
    {
    goto end; //DateTime
    }

    // public_judge_audit_record->evaluator_id
    cJSON *evaluator_id = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "evaluator_id");
    if (cJSON_IsNull(evaluator_id)) {
        evaluator_id = NULL;
    }
    if (!evaluator_id) {
        goto end;
    }

    
    if(!cJSON_IsString(evaluator_id))
    {
    goto end; //String
    }

    // public_judge_audit_record->judge_call_id
    cJSON *judge_call_id = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "judge_call_id");
    if (cJSON_IsNull(judge_call_id)) {
        judge_call_id = NULL;
    }
    if (!judge_call_id) {
        goto end;
    }

    
    if(!cJSON_IsString(judge_call_id))
    {
    goto end; //String
    }

    // public_judge_audit_record->model
    cJSON *model = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "model");
    if (cJSON_IsNull(model)) {
        model = NULL;
    }
    if (!model) {
        goto end;
    }

    
    if(!cJSON_IsString(model))
    {
    goto end; //String
    }

    // public_judge_audit_record->project_id
    cJSON *project_id = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "project_id");
    if (cJSON_IsNull(project_id)) {
        project_id = NULL;
    }
    if (!project_id) {
        goto end;
    }

    
    if(!cJSON_IsString(project_id))
    {
    goto end; //String
    }

    // public_judge_audit_record->request_hash
    cJSON *request_hash = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "request_hash");
    if (cJSON_IsNull(request_hash)) {
        request_hash = NULL;
    }
    if (!request_hash) {
        goto end;
    }

    
    if(!cJSON_IsString(request_hash))
    {
    goto end; //String
    }

    // public_judge_audit_record->response_hash
    cJSON *response_hash = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "response_hash");
    if (cJSON_IsNull(response_hash)) {
        response_hash = NULL;
    }
    if (!response_hash) {
        goto end;
    }

    
    if(!cJSON_IsString(response_hash))
    {
    goto end; //String
    }

    // public_judge_audit_record->score
    cJSON *score = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "score");
    if (cJSON_IsNull(score)) {
        score = NULL;
    }
    if (!score) {
        goto end;
    }

    
    if(!cJSON_IsNumber(score))
    {
    goto end; //Numeric
    }

    // public_judge_audit_record->tenant_id
    cJSON *tenant_id = cJSON_GetObjectItemCaseSensitive(public_judge_audit_recordJSON, "tenant_id");
    if (cJSON_IsNull(tenant_id)) {
        tenant_id = NULL;
    }
    if (!tenant_id) {
        goto end;
    }

    
    if(!cJSON_IsString(tenant_id))
    {
    goto end; //String
    }


    public_judge_audit_record_local_var = public_judge_audit_record_create_internal (
        cached->valueint,
        charged_cost_local_nonprim,
        strdup(created_at->valuestring),
        strdup(evaluator_id->valuestring),
        strdup(judge_call_id->valuestring),
        strdup(model->valuestring),
        strdup(project_id->valuestring),
        strdup(request_hash->valuestring),
        strdup(response_hash->valuestring),
        score->valuedouble,
        strdup(tenant_id->valuestring)
        );

    return public_judge_audit_record_local_var;
end:
    if (charged_cost_local_nonprim) {
        money_free(charged_cost_local_nonprim);
        charged_cost_local_nonprim = NULL;
    }
    return NULL;

}
