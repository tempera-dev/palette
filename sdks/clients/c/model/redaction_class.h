/*
 * redaction_class.h
 *
 * 
 */

#ifndef _redaction_class_H_
#define _redaction_class_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct redaction_class_t redaction_class_t;


// Enum  for redaction_class

typedef enum { palette_api_redaction_class__NULL = 0, palette_api_redaction_class___public, palette_api_redaction_class__internal, palette_api_redaction_class__sensitive, palette_api_redaction_class__secret } palette_api_redaction_class__e;

char* redaction_class_redaction_class_ToString(palette_api_redaction_class__e redaction_class);

palette_api_redaction_class__e redaction_class_redaction_class_FromString(char* redaction_class);

cJSON *redaction_class_convertToJSON(palette_api_redaction_class__e redaction_class);

palette_api_redaction_class__e redaction_class_parseFromJSON(cJSON *redaction_classJSON);

#endif /* _redaction_class_H_ */

