/*
 * public_judge_audit_record.h
 *
 * Client-facing judge ledger row. The backing &#x60;provider&#x60;, the &#x60;provider_secret_id&#x60;, and our raw &#x60;provider_cost&#x60; are INTERNAL (staff-only) and must never reach a customer — exposing &#x60;provider_cost&#x60; alongside &#x60;charged_cost&#x60; would also leak our margin (billing-credits-contract §11). Only customer-facing fields appear here, including &#x60;charged_cost&#x60; (the amount the customer actually pays).
 */

#ifndef _public_judge_audit_record_H_
#define _public_judge_audit_record_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct public_judge_audit_record_t public_judge_audit_record_t;

#include "money.h"



typedef struct public_judge_audit_record_t {
    int cached; //boolean
    struct money_t *charged_cost; //model
    char *created_at; //date time
    char *evaluator_id; // string
    char *judge_call_id; // string
    char *model; // string
    char *project_id; // string
    char *request_hash; // string
    char *response_hash; // string
    double score; //numeric
    char *tenant_id; // string

    int _library_owned; // Is the library responsible for freeing this object?
} public_judge_audit_record_t;

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
);

void public_judge_audit_record_free(public_judge_audit_record_t *public_judge_audit_record);

public_judge_audit_record_t *public_judge_audit_record_parseFromJSON(cJSON *public_judge_audit_recordJSON);

cJSON *public_judge_audit_record_convertToJSON(public_judge_audit_record_t *public_judge_audit_record);

#endif /* _public_judge_audit_record_H_ */

