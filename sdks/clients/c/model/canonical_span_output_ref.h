/*
 * canonical_span_output_ref.h
 *
 *
 */

#ifndef _canonical_span_output_ref_H_
#define _canonical_span_output_ref_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct canonical_span_output_ref_t canonical_span_output_ref_t;

#include "artifact_ref.h"
#include "redaction_class.h"



typedef struct canonical_span_output_ref_t {
    char *artifact_id; // string
    char *mime_type; // string
    palette_api_redaction_class__e redaction_class; //referenced enum
    char *sha256; // string
    long size_bytes; //numeric
    char *uri; // string

    int _library_owned; // Is the library responsible for freeing this object?
} canonical_span_output_ref_t;

__attribute__((deprecated)) canonical_span_output_ref_t *canonical_span_output_ref_create(
    char *artifact_id,
    char *mime_type,
    palette_api_redaction_class__e redaction_class,
    char *sha256,
    long size_bytes,
    char *uri
);

void canonical_span_output_ref_free(canonical_span_output_ref_t *canonical_span_output_ref);

canonical_span_output_ref_t *canonical_span_output_ref_parseFromJSON(cJSON *canonical_span_output_refJSON);

cJSON *canonical_span_output_ref_convertToJSON(canonical_span_output_ref_t *canonical_span_output_ref);

#endif /* _canonical_span_output_ref_H_ */
