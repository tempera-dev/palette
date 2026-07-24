/*
 * judge_ledger_list_response.h
 *
 *
 */

#ifndef _judge_ledger_list_response_H_
#define _judge_ledger_list_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct judge_ledger_list_response_t judge_ledger_list_response_t;

#include "public_judge_audit_record.h"



typedef struct judge_ledger_list_response_t {
    char *next_page_token; // string
    list_t *records; //nonprimitive container

    int _library_owned; // Is the library responsible for freeing this object?
} judge_ledger_list_response_t;

__attribute__((deprecated)) judge_ledger_list_response_t *judge_ledger_list_response_create(
    char *next_page_token,
    list_t *records
);

void judge_ledger_list_response_free(judge_ledger_list_response_t *judge_ledger_list_response);

judge_ledger_list_response_t *judge_ledger_list_response_parseFromJSON(cJSON *judge_ledger_list_responseJSON);

cJSON *judge_ledger_list_response_convertToJSON(judge_ledger_list_response_t *judge_ledger_list_response);

#endif /* _judge_ledger_list_response_H_ */
