#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "import_tempera_evidence_request.h"



static import_tempera_evidence_request_t *import_tempera_evidence_request_create_internal(
    char *canonical_json,
    char *public_key_pem,
    char *signature_base64
    ) {
    import_tempera_evidence_request_t *import_tempera_evidence_request_local_var = malloc(sizeof(import_tempera_evidence_request_t));
    if (!import_tempera_evidence_request_local_var) {
        return NULL;
    }
    import_tempera_evidence_request_local_var->canonical_json = canonical_json;
    import_tempera_evidence_request_local_var->public_key_pem = public_key_pem;
    import_tempera_evidence_request_local_var->signature_base64 = signature_base64;

    import_tempera_evidence_request_local_var->_library_owned = 1;
    return import_tempera_evidence_request_local_var;
}

__attribute__((deprecated)) import_tempera_evidence_request_t *import_tempera_evidence_request_create(
    char *canonical_json,
    char *public_key_pem,
    char *signature_base64
    ) {
    return import_tempera_evidence_request_create_internal (
        canonical_json,
        public_key_pem,
        signature_base64
        );
}

void import_tempera_evidence_request_free(import_tempera_evidence_request_t *import_tempera_evidence_request) {
    if(NULL == import_tempera_evidence_request){
        return ;
    }
    if(import_tempera_evidence_request->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "import_tempera_evidence_request_free");
        return ;
    }
    listEntry_t *listEntry;
    if (import_tempera_evidence_request->canonical_json) {
        free(import_tempera_evidence_request->canonical_json);
        import_tempera_evidence_request->canonical_json = NULL;
    }
    if (import_tempera_evidence_request->public_key_pem) {
        free(import_tempera_evidence_request->public_key_pem);
        import_tempera_evidence_request->public_key_pem = NULL;
    }
    if (import_tempera_evidence_request->signature_base64) {
        free(import_tempera_evidence_request->signature_base64);
        import_tempera_evidence_request->signature_base64 = NULL;
    }
    free(import_tempera_evidence_request);
}

cJSON *import_tempera_evidence_request_convertToJSON(import_tempera_evidence_request_t *import_tempera_evidence_request) {
    cJSON *item = cJSON_CreateObject();

    // import_tempera_evidence_request->canonical_json
    if (!import_tempera_evidence_request->canonical_json) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "canonical_json", import_tempera_evidence_request->canonical_json) == NULL) {
    goto fail; //String
    }


    // import_tempera_evidence_request->public_key_pem
    if (!import_tempera_evidence_request->public_key_pem) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "public_key_pem", import_tempera_evidence_request->public_key_pem) == NULL) {
    goto fail; //String
    }


    // import_tempera_evidence_request->signature_base64
    if (!import_tempera_evidence_request->signature_base64) {
        goto fail;
    }
    if(cJSON_AddStringToObject(item, "signature_base64", import_tempera_evidence_request->signature_base64) == NULL) {
    goto fail; //String
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

import_tempera_evidence_request_t *import_tempera_evidence_request_parseFromJSON(cJSON *import_tempera_evidence_requestJSON){

    import_tempera_evidence_request_t *import_tempera_evidence_request_local_var = NULL;

    // import_tempera_evidence_request->canonical_json
    cJSON *canonical_json = cJSON_GetObjectItemCaseSensitive(import_tempera_evidence_requestJSON, "canonical_json");
    if (cJSON_IsNull(canonical_json)) {
        canonical_json = NULL;
    }
    if (!canonical_json) {
        goto end;
    }

    
    if(!cJSON_IsString(canonical_json))
    {
    goto end; //String
    }

    // import_tempera_evidence_request->public_key_pem
    cJSON *public_key_pem = cJSON_GetObjectItemCaseSensitive(import_tempera_evidence_requestJSON, "public_key_pem");
    if (cJSON_IsNull(public_key_pem)) {
        public_key_pem = NULL;
    }
    if (!public_key_pem) {
        goto end;
    }

    
    if(!cJSON_IsString(public_key_pem))
    {
    goto end; //String
    }

    // import_tempera_evidence_request->signature_base64
    cJSON *signature_base64 = cJSON_GetObjectItemCaseSensitive(import_tempera_evidence_requestJSON, "signature_base64");
    if (cJSON_IsNull(signature_base64)) {
        signature_base64 = NULL;
    }
    if (!signature_base64) {
        goto end;
    }

    
    if(!cJSON_IsString(signature_base64))
    {
    goto end; //String
    }


    import_tempera_evidence_request_local_var = import_tempera_evidence_request_create_internal (
        strdup(canonical_json->valuestring),
        strdup(public_key_pem->valuestring),
        strdup(signature_base64->valuestring)
        );

    return import_tempera_evidence_request_local_var;
end:
    return NULL;

}
