/*
 * inconclusive_policy.h
 *
 * 
 */

#ifndef _inconclusive_policy_H_
#define _inconclusive_policy_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct inconclusive_policy_t inconclusive_policy_t;


// Enum  for inconclusive_policy

typedef enum { palette_api_inconclusive_policy__NULL = 0, palette_api_inconclusive_policy__pass, palette_api_inconclusive_policy__fail } palette_api_inconclusive_policy__e;

char* inconclusive_policy_inconclusive_policy_ToString(palette_api_inconclusive_policy__e inconclusive_policy);

palette_api_inconclusive_policy__e inconclusive_policy_inconclusive_policy_FromString(char* inconclusive_policy);

cJSON *inconclusive_policy_convertToJSON(palette_api_inconclusive_policy__e inconclusive_policy);

palette_api_inconclusive_policy__e inconclusive_policy_parseFromJSON(cJSON *inconclusive_policyJSON);

#endif /* _inconclusive_policy_H_ */

