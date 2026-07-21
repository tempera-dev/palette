/*
 * tempera_evidence_receipt.h
 *
 * 
 */

#ifndef _tempera_evidence_receipt_H_
#define _tempera_evidence_receipt_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct tempera_evidence_receipt_t tempera_evidence_receipt_t;

#include "external_eval_evidence_kind.h"
#include "tempera_evidence_summary.h"



typedef struct tempera_evidence_receipt_t {
    int created; //boolean
    char *declared_content_sha256; // string
    char *external_id; // string
    palette_api_external_eval_evidence_kind__e kind; //referenced enum
    char *project_id; // string
    char *public_key_sha256; // string
    char *schema_version; // string
    char *signature_sha256; // string
    char *signed_payload_sha256; // string
    char *source_schema_version; // string
    char *stored_at; //date time
    struct tempera_evidence_summary_t *summary; //model
    char *tenant_id; // string

    int _library_owned; // Is the library responsible for freeing this object?
} tempera_evidence_receipt_t;

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
);

void tempera_evidence_receipt_free(tempera_evidence_receipt_t *tempera_evidence_receipt);

tempera_evidence_receipt_t *tempera_evidence_receipt_parseFromJSON(cJSON *tempera_evidence_receiptJSON);

cJSON *tempera_evidence_receipt_convertToJSON(tempera_evidence_receipt_t *tempera_evidence_receipt);

#endif /* _tempera_evidence_receipt_H_ */

