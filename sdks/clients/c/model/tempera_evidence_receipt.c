#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "tempera_evidence_receipt.h"



static tempera_evidence_receipt_t *tempera_evidence_receipt_create_internal(
    int created,
    char *declared_content_sha256,
    char *external_id,
    palette_api_external_eval_evidence_kind__e kind,
    char *project_id,
    char *public_key_sha256,
    char *schema_version,
    char *signature_sha256,
    char *signed_payload_sha256,
    char *source_schema_version,
    char *stored_at,
    tempera_evidence_summary_t *summary,
    char *tenant_id
    ) {
    tempera_evidence_receipt_t *tempera_evidence_receipt_local_var = malloc(sizeof(tempera_evidence_receipt_t));
    if (!tempera_evidence_receipt_local_var) {
        return NULL;
    }
    tempera_evidence_receipt_local_var->created = created;
    tempera_evidence_receipt_local_var->declared_content_sha256 = declared_content_sha256;
    tempera_evidence_receipt_local_var->external_id = external_id;
    tempera_evidence_receipt_local_var->kind = kind;
    tempera_evidence_receipt_local_var->project_id = project_id;
    tempera_evidence_receipt_local_var->public_key_sha256 = public_key_sha256;
    tempera_evidence_receipt_local_var->schema_version = schema_version;
    tempera_evidence_receipt_local_var->signature_sha256 = signature_sha256;
    tempera_evidence_receipt_local_var->signed_payload_sha256 = signed_payload_sha256;
    tempera_evidence_receipt_local_var->source_schema_version = source_schema_version;
    tempera_evidence_receipt_local_var->stored_at = stored_at;
    tempera_evidence_receipt_local_var->summary = summary;
    tempera_evidence_receipt_local_var->tenant_id = tenant_id;

    tempera_evidence_receipt_local_var->_library_owned = 1;
    return tempera_evidence_receipt_local_var;
}

__attribute__((deprecated)) tempera_evidence_receipt_t *tempera_evidence_receipt_create(
    int created,
    char *declared_content_sha256,
    char *external_id,
    palette_api_external_eval_evidence_kind__e kind,
    char *project_id,
    char *public_key_sha256,
    char *schema_version,
    char *signature_sha256,
    char *signed_payload_sha256,
    char *source_schema_version,
    char *stored_at,
    tempera_evidence_summary_t *summary,
    char *tenant_id
    ) {
    return tempera_evidence_receipt_create_internal (
        created,
        declared_content_sha256,
        external_id,
        kind,
        project_id,
        public_key_sha256,
        schema_version,
        signature_sha256,
        signed_payload_sha256,
        source_schema_version,
        stored_at,
        summary,
        tenant_id
        );
}

void tempera_evidence_receipt_free(tempera_evidence_receipt_t *tempera_evidence_receipt) {
    if(NULL == tempera_evidence_receipt){
        return ;
    }
    if(tempera_evidence_receipt->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "tempera_evidence_receipt_free");
        return ;
    }
    listEntry_t *listEntry;
    if (tempera_evidence_receipt->declared_content_sha256) {
        free(tempera_evidence_receipt->declared_content_sha256);
        tempera_evidence_receipt->declared_content_sha256 = NULL;
    }
    if (tempera_evidence_receipt->external_id) {
        free(tempera_evidence_receipt->external_id);
        tempera_evidence_receipt->external_id = NULL;
    }
    if (tempera_evidence_receipt->project_id) {
        free(tempera_evidence_receipt->project_id);
        tempera_evidence_receipt->project_id = NULL;
    }
    if (tempera_evidence_receipt->public_key_sha256) {
        free(tempera_evidence_receipt->public_key_sha256);
        tempera_evidence_receipt->public_key_sha256 = NULL;
    }
    if (tempera_evidence_receipt->schema_version) {
        free(tempera_evidence_receipt->schema_version);
        tempera_evidence_receipt->schema_version = NULL;
    }
    if (tempera_evidence_receipt->signature_sha256) {
        free(tempera_evidence_receipt->signature_sha256);
        tempera_evidence_receipt->signature_sha256 = NULL;
    }
    if (tempera_evidence_receipt->signed_payload_sha256) {
        free(tempera_evidence_receipt->signed_payload_sha256);
        tempera_evidence_receipt->signed_payload_sha256 = NULL;
    }
    if (tempera_evidence_receipt->source_schema_version) {
        free(tempera_evidence_receipt->source_schema_version);
        tempera_evidence_receipt->source_schema_version = NULL;
    }
    if (tempera_evidence_receipt->stored_at) {
        free(tempera_evidence_receipt->stored_at);
        tempera_evidence_receipt->stored_at = NULL;
    }
    if (tempera_evidence_receipt->summary) {
        tempera_evidence_summary_free(tempera_evidence_receipt->summary);
        tempera_evidence_receipt->summary = NULL;
    }
    if (tempera_evidence_receipt->tenant_id) {
        free(tempera_evidence_receipt->tenant_id);
        tempera_evidence_receipt->tenant_id = NULL;
    }
    free(tempera_evidence_receipt);
}

cJSON *tempera_evidence_receipt_convertToJSON(tempera_evidence_receipt_t *tempera_evidence_receipt) {
    cJSON *item = cJSON_CreateObject();

    // tempera_evidence_receipt->created
    if (!tempera_evidence_receipt->created) {
        goto fail;
    }
    if(cJSON_AddBoolToObject(item, "created", tempera_evidence_receipt->created) == NULL) {
    goto fail; //Bool
    }


    // tempera_evidence_receipt->declared_content_sha256
    if (!tempera_evidence_receipt->declared_content_sha256) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "declared_content_sha256", tempera_evidence_receipt->declared_content_sha256) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->external_id
    if (!tempera_evidence_receipt->external_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "external_id", tempera_evidence_receipt->external_id) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->kind
    if (palette_api_external_eval_evidence_kind__NULL == tempera_evidence_receipt->kind) {
        goto fail;
    }
    cJSON *kind_local_JSON = external_eval_evidence_kind_convertToJSON(tempera_evidence_receipt->kind);
    if(kind_local_JSON == NULL) {
        goto fail; // custom
    }
    cJSON_AddItemToObject(item, "kind", kind_local_JSON);
    if(item->child == NULL) {
        goto fail;
    }


    // tempera_evidence_receipt->project_id
    if (!tempera_evidence_receipt->project_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "project_id", tempera_evidence_receipt->project_id) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->public_key_sha256
    if (!tempera_evidence_receipt->public_key_sha256) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "public_key_sha256", tempera_evidence_receipt->public_key_sha256) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->schema_version
    if (!tempera_evidence_receipt->schema_version) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "schema_version", tempera_evidence_receipt->schema_version) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->signature_sha256
    if (!tempera_evidence_receipt->signature_sha256) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "signature_sha256", tempera_evidence_receipt->signature_sha256) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->signed_payload_sha256
    if (!tempera_evidence_receipt->signed_payload_sha256) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "signed_payload_sha256", tempera_evidence_receipt->signed_payload_sha256) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->source_schema_version
    if (!tempera_evidence_receipt->source_schema_version) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "source_schema_version", tempera_evidence_receipt->source_schema_version) == NULL) {
    goto fail; //String
    }


    // tempera_evidence_receipt->stored_at
    if (!tempera_evidence_receipt->stored_at) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "stored_at", tempera_evidence_receipt->stored_at) == NULL) {
    goto fail; //Date-Time
    }


    // tempera_evidence_receipt->summary
    if (!tempera_evidence_receipt->summary) {
        goto fail;
    }
    cJSON *summary_local_JSON = tempera_evidence_summary_convertToJSON(tempera_evidence_receipt->summary);
    if(summary_local_JSON == NULL) {
    goto fail; //model
    }
    cJSON_AddItemToObject(item, "summary", summary_local_JSON);
    if(item->child == NULL) {
    goto fail;
    }


    // tempera_evidence_receipt->tenant_id
    if (!tempera_evidence_receipt->tenant_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "tenant_id", tempera_evidence_receipt->tenant_id) == NULL) {
    goto fail; //String
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

tempera_evidence_receipt_t *tempera_evidence_receipt_parseFromJSON(cJSON *tempera_evidence_receiptJSON){

    tempera_evidence_receipt_t *tempera_evidence_receipt_local_var = NULL;

    // define the local variable for tempera_evidence_receipt->kind
    palette_api_external_eval_evidence_kind__e kind_local_nonprim = 0;

    // define the local variable for tempera_evidence_receipt->summary
    tempera_evidence_summary_t *summary_local_nonprim = NULL;

    // tempera_evidence_receipt->created
    cJSON *created = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "created");
    if (cJSON_IsNull(created)) {
        created = NULL;
    }
    if (!created) {
        goto end;
    }

    
    if(!cJSON_IsBool(created))
    {
    goto end; //Bool
    }

    // tempera_evidence_receipt->declared_content_sha256
    cJSON *declared_content_sha256 = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "declared_content_sha256");
    if (cJSON_IsNull(declared_content_sha256)) {
        declared_content_sha256 = NULL;
    }
    if (!declared_content_sha256) {
        goto end;
    }

    
    if(!cJSON_IsString(declared_content_sha256))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->external_id
    cJSON *external_id = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "external_id");
    if (cJSON_IsNull(external_id)) {
        external_id = NULL;
    }
    if (!external_id) {
        goto end;
    }

    
    if(!cJSON_IsString(external_id))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->kind
    cJSON *kind = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "kind");
    if (cJSON_IsNull(kind)) {
        kind = NULL;
    }
    if (!kind) {
        goto end;
    }

    
    kind_local_nonprim = external_eval_evidence_kind_parseFromJSON(kind); //custom

    // tempera_evidence_receipt->project_id
    cJSON *project_id = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "project_id");
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

    // tempera_evidence_receipt->public_key_sha256
    cJSON *public_key_sha256 = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "public_key_sha256");
    if (cJSON_IsNull(public_key_sha256)) {
        public_key_sha256 = NULL;
    }
    if (!public_key_sha256) {
        goto end;
    }

    
    if(!cJSON_IsString(public_key_sha256))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->schema_version
    cJSON *schema_version = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "schema_version");
    if (cJSON_IsNull(schema_version)) {
        schema_version = NULL;
    }
    if (!schema_version) {
        goto end;
    }

    
    if(!cJSON_IsString(schema_version))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->signature_sha256
    cJSON *signature_sha256 = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "signature_sha256");
    if (cJSON_IsNull(signature_sha256)) {
        signature_sha256 = NULL;
    }
    if (!signature_sha256) {
        goto end;
    }

    
    if(!cJSON_IsString(signature_sha256))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->signed_payload_sha256
    cJSON *signed_payload_sha256 = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "signed_payload_sha256");
    if (cJSON_IsNull(signed_payload_sha256)) {
        signed_payload_sha256 = NULL;
    }
    if (!signed_payload_sha256) {
        goto end;
    }

    
    if(!cJSON_IsString(signed_payload_sha256))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->source_schema_version
    cJSON *source_schema_version = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "source_schema_version");
    if (cJSON_IsNull(source_schema_version)) {
        source_schema_version = NULL;
    }
    if (!source_schema_version) {
        goto end;
    }

    
    if(!cJSON_IsString(source_schema_version))
    {
    goto end; //String
    }

    // tempera_evidence_receipt->stored_at
    cJSON *stored_at = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "stored_at");
    if (cJSON_IsNull(stored_at)) {
        stored_at = NULL;
    }
    if (!stored_at) {
        goto end;
    }

    
    if(!cJSON_IsString(stored_at) && !cJSON_IsNull(stored_at))
    {
    goto end; //DateTime
    }

    // tempera_evidence_receipt->summary
    cJSON *summary = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "summary");
    if (cJSON_IsNull(summary)) {
        summary = NULL;
    }
    if (!summary) {
        goto end;
    }

    
    summary_local_nonprim = tempera_evidence_summary_parseFromJSON(summary); //nonprimitive

    // tempera_evidence_receipt->tenant_id
    cJSON *tenant_id = cJSON_GetObjectItemCaseSensitive(tempera_evidence_receiptJSON, "tenant_id");
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


    tempera_evidence_receipt_local_var = tempera_evidence_receipt_create_internal (
        created->valueint,
        strdup(declared_content_sha256->valuestring),
        strdup(external_id->valuestring),
        kind_local_nonprim,
        strdup(project_id->valuestring),
        strdup(public_key_sha256->valuestring),
        strdup(schema_version->valuestring),
        strdup(signature_sha256->valuestring),
        strdup(signed_payload_sha256->valuestring),
        strdup(source_schema_version->valuestring),
        strdup(stored_at->valuestring),
        summary_local_nonprim,
        strdup(tenant_id->valuestring)
        );

    return tempera_evidence_receipt_local_var;
end:
    if (kind_local_nonprim) {
        kind_local_nonprim = 0;
    }
    if (summary_local_nonprim) {
        tempera_evidence_summary_free(summary_local_nonprim);
        summary_local_nonprim = NULL;
    }
    return NULL;

}
