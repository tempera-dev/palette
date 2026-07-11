/*
 * audit_outcome.h
 *
 * 
 */

#ifndef _audit_outcome_H_
#define _audit_outcome_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct audit_outcome_t audit_outcome_t;


// Enum  for audit_outcome

typedef enum { palette_api_audit_outcome__NULL = 0, palette_api_audit_outcome__allowed, palette_api_audit_outcome__denied } palette_api_audit_outcome__e;

char* audit_outcome_audit_outcome_ToString(palette_api_audit_outcome__e audit_outcome);

palette_api_audit_outcome__e audit_outcome_audit_outcome_FromString(char* audit_outcome);

cJSON *audit_outcome_convertToJSON(palette_api_audit_outcome__e audit_outcome);

palette_api_audit_outcome__e audit_outcome_parseFromJSON(cJSON *audit_outcomeJSON);

#endif /* _audit_outcome_H_ */

