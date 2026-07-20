/*
 * import_tempera_evidence_request.h
 *
 * 
 */

#ifndef _import_tempera_evidence_request_H_
#define _import_tempera_evidence_request_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct import_tempera_evidence_request_t import_tempera_evidence_request_t;




typedef struct import_tempera_evidence_request_t {
    char *canonical_json; // string
    char *public_key_pem; // string
    char *signature_base64; // string

    int _library_owned; // Is the library responsible for freeing this object?
} import_tempera_evidence_request_t;

__attribute__((deprecated)) import_tempera_evidence_request_t *import_tempera_evidence_request_create(
    char *canonical_json,
    char *public_key_pem,
    char *signature_base64
);

void import_tempera_evidence_request_free(import_tempera_evidence_request_t *import_tempera_evidence_request);

import_tempera_evidence_request_t *import_tempera_evidence_request_parseFromJSON(cJSON *import_tempera_evidence_requestJSON);

cJSON *import_tempera_evidence_request_convertToJSON(import_tempera_evidence_request_t *import_tempera_evidence_request);

#endif /* _import_tempera_evidence_request_H_ */

