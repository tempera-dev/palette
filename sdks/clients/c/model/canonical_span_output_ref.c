#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "canonical_span_output_ref.h"



static canonical_span_output_ref_t *canonical_span_output_ref_create_internal(
    char *artifact_id,
    char *mime_type,
    palette_api_redaction_class__e redaction_class,
    char *sha256,
    long size_bytes,
    char *uri
    ) {
    canonical_span_output_ref_t *canonical_span_output_ref_local_var = malloc(sizeof(canonical_span_output_ref_t));
    if (!canonical_span_output_ref_local_var) {
        return NULL;
    }
    canonical_span_output_ref_local_var->artifact_id = artifact_id;
    canonical_span_output_ref_local_var->mime_type = mime_type;
    canonical_span_output_ref_local_var->redaction_class = redaction_class;
    canonical_span_output_ref_local_var->sha256 = sha256;
    canonical_span_output_ref_local_var->size_bytes = size_bytes;
    canonical_span_output_ref_local_var->uri = uri;

    canonical_span_output_ref_local_var->_library_owned = 1;
    return canonical_span_output_ref_local_var;
}

__attribute__((deprecated)) canonical_span_output_ref_t *canonical_span_output_ref_create(
    char *artifact_id,
    char *mime_type,
    palette_api_redaction_class__e redaction_class,
    char *sha256,
    long size_bytes,
    char *uri
    ) {
    return canonical_span_output_ref_create_internal (
        artifact_id,
        mime_type,
        redaction_class,
        sha256,
        size_bytes,
        uri
        );
}

void canonical_span_output_ref_free(canonical_span_output_ref_t *canonical_span_output_ref) {
    if(NULL == canonical_span_output_ref){
        return ;
    }
    if(canonical_span_output_ref->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "canonical_span_output_ref_free");
        return ;
    }
    listEntry_t *listEntry;
    if (canonical_span_output_ref->artifact_id) {
        free(canonical_span_output_ref->artifact_id);
        canonical_span_output_ref->artifact_id = NULL;
    }
    if (canonical_span_output_ref->mime_type) {
        free(canonical_span_output_ref->mime_type);
        canonical_span_output_ref->mime_type = NULL;
    }
    if (canonical_span_output_ref->sha256) {
        free(canonical_span_output_ref->sha256);
        canonical_span_output_ref->sha256 = NULL;
    }
    if (canonical_span_output_ref->uri) {
        free(canonical_span_output_ref->uri);
        canonical_span_output_ref->uri = NULL;
    }
    free(canonical_span_output_ref);
}

cJSON *canonical_span_output_ref_convertToJSON(canonical_span_output_ref_t *canonical_span_output_ref) {
    cJSON *item = cJSON_CreateObject();

    // canonical_span_output_ref->artifact_id
    if (!canonical_span_output_ref->artifact_id) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "artifactId", canonical_span_output_ref->artifact_id) == NULL) {
    goto fail; //String
    }


    // canonical_span_output_ref->mime_type
    if (!canonical_span_output_ref->mime_type) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "mimeType", canonical_span_output_ref->mime_type) == NULL) {
    goto fail; //String
    }


    // canonical_span_output_ref->redaction_class
    if (palette_api_redaction_class__NULL == canonical_span_output_ref->redaction_class) {
        goto fail;
    }
    cJSON *redaction_class_local_JSON = redaction_class_convertToJSON(canonical_span_output_ref->redaction_class);
    if(redaction_class_local_JSON == NULL) {
        goto fail; // custom
    }
    cJSON_AddItemToObject(item, "redactionClass", redaction_class_local_JSON);
    if(item->child == NULL) {
        goto fail;
    }


    // canonical_span_output_ref->sha256
    if (!canonical_span_output_ref->sha256) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "sha256", canonical_span_output_ref->sha256) == NULL) {
    goto fail; //String
    }


    // canonical_span_output_ref->size_bytes
    if (!canonical_span_output_ref->size_bytes) {
        goto fail;
    }
    if(cJSON_AddNumberToObject(item, "sizeBytes", canonical_span_output_ref->size_bytes) == NULL) {
    goto fail; //Numeric
    }


    // canonical_span_output_ref->uri
    if (!canonical_span_output_ref->uri) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "uri", canonical_span_output_ref->uri) == NULL) {
    goto fail; //String
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

canonical_span_output_ref_t *canonical_span_output_ref_parseFromJSON(cJSON *canonical_span_output_refJSON){

    canonical_span_output_ref_t *canonical_span_output_ref_local_var = NULL;

    // define the local variable for canonical_span_output_ref->redaction_class
    palette_api_redaction_class__e redaction_class_local_nonprim = 0;

    // canonical_span_output_ref->artifact_id
    cJSON *artifact_id = cJSON_GetObjectItemCaseSensitive(canonical_span_output_refJSON, "artifactId");
    if (cJSON_IsNull(artifact_id)) {
        artifact_id = NULL;
    }
    if (!artifact_id) {
        goto end;
    }


    if(!cJSON_IsString(artifact_id))
    {
    goto end; //String
    }

    // canonical_span_output_ref->mime_type
    cJSON *mime_type = cJSON_GetObjectItemCaseSensitive(canonical_span_output_refJSON, "mimeType");
    if (cJSON_IsNull(mime_type)) {
        mime_type = NULL;
    }
    if (!mime_type) {
        goto end;
    }


    if(!cJSON_IsString(mime_type))
    {
    goto end; //String
    }

    // canonical_span_output_ref->redaction_class
    cJSON *redaction_class = cJSON_GetObjectItemCaseSensitive(canonical_span_output_refJSON, "redactionClass");
    if (cJSON_IsNull(redaction_class)) {
        redaction_class = NULL;
    }
    if (!redaction_class) {
        goto end;
    }


    redaction_class_local_nonprim = redaction_class_parseFromJSON(redaction_class); //custom

    // canonical_span_output_ref->sha256
    cJSON *sha256 = cJSON_GetObjectItemCaseSensitive(canonical_span_output_refJSON, "sha256");
    if (cJSON_IsNull(sha256)) {
        sha256 = NULL;
    }
    if (!sha256) {
        goto end;
    }


    if(!cJSON_IsString(sha256))
    {
    goto end; //String
    }

    // canonical_span_output_ref->size_bytes
    cJSON *size_bytes = cJSON_GetObjectItemCaseSensitive(canonical_span_output_refJSON, "sizeBytes");
    if (cJSON_IsNull(size_bytes)) {
        size_bytes = NULL;
    }
    if (!size_bytes) {
        goto end;
    }


    if(!cJSON_IsNumber(size_bytes))
    {
    goto end; //Numeric
    }

    // canonical_span_output_ref->uri
    cJSON *uri = cJSON_GetObjectItemCaseSensitive(canonical_span_output_refJSON, "uri");
    if (cJSON_IsNull(uri)) {
        uri = NULL;
    }
    if (!uri) {
        goto end;
    }


    if(!cJSON_IsString(uri))
    {
    goto end; //String
    }


    canonical_span_output_ref_local_var = canonical_span_output_ref_create_internal (
        strdup(artifact_id->valuestring),
        strdup(mime_type->valuestring),
        redaction_class_local_nonprim,
        strdup(sha256->valuestring),
        size_bytes->valuedouble,
        strdup(uri->valuestring)
        );

    return canonical_span_output_ref_local_var;
end:
    if (redaction_class_local_nonprim) {
        redaction_class_local_nonprim = 0;
    }
    return NULL;

}
