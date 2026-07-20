/*
 * tempera_evidence_summary.h
 *
 * 
 */

#ifndef _tempera_evidence_summary_H_
#define _tempera_evidence_summary_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct tempera_evidence_summary_t tempera_evidence_summary_t;




typedef struct tempera_evidence_summary_t {
    char *experiment_id; // string
    char *run_id; // string
    char *split; // string
    char *suite_id; // string
    char *suite_version; // string
    char *verdict; // string

    int _library_owned; // Is the library responsible for freeing this object?
} tempera_evidence_summary_t;

__attribute__((deprecated)) tempera_evidence_summary_t *tempera_evidence_summary_create(
    char *experiment_id,
    char *run_id,
    char *split,
    char *suite_id,
    char *suite_version,
    char *verdict
);

void tempera_evidence_summary_free(tempera_evidence_summary_t *tempera_evidence_summary);

tempera_evidence_summary_t *tempera_evidence_summary_parseFromJSON(cJSON *tempera_evidence_summaryJSON);

cJSON *tempera_evidence_summary_convertToJSON(tempera_evidence_summary_t *tempera_evidence_summary);

#endif /* _tempera_evidence_summary_H_ */

