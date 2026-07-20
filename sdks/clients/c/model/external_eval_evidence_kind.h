/*
 * external_eval_evidence_kind.h
 *
 * 
 */

#ifndef _external_eval_evidence_kind_H_
#define _external_eval_evidence_kind_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct external_eval_evidence_kind_t external_eval_evidence_kind_t;


// Enum  for external_eval_evidence_kind

typedef enum { palette_api_external_eval_evidence_kind__NULL = 0, palette_api_external_eval_evidence_kind__result_bundle, palette_api_external_eval_evidence_kind__ab_decision } palette_api_external_eval_evidence_kind__e;

char* external_eval_evidence_kind_external_eval_evidence_kind_ToString(palette_api_external_eval_evidence_kind__e external_eval_evidence_kind);

palette_api_external_eval_evidence_kind__e external_eval_evidence_kind_external_eval_evidence_kind_FromString(char* external_eval_evidence_kind);

cJSON *external_eval_evidence_kind_convertToJSON(palette_api_external_eval_evidence_kind__e external_eval_evidence_kind);

palette_api_external_eval_evidence_kind__e external_eval_evidence_kind_parseFromJSON(cJSON *external_eval_evidence_kindJSON);

#endif /* _external_eval_evidence_kind_H_ */

