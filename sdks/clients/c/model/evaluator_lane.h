/*
 * evaluator_lane.h
 *
 * 
 */

#ifndef _evaluator_lane_H_
#define _evaluator_lane_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

typedef struct evaluator_lane_t evaluator_lane_t;


// Enum  for evaluator_lane

typedef enum { palette_api_evaluator_lane__NULL = 0, palette_api_evaluator_lane__deterministic_wasi, palette_api_evaluator_lane__judge_broker, palette_api_evaluator_lane__human, palette_api_evaluator_lane__hybrid } palette_api_evaluator_lane__e;

char* evaluator_lane_evaluator_lane_ToString(palette_api_evaluator_lane__e evaluator_lane);

palette_api_evaluator_lane__e evaluator_lane_evaluator_lane_FromString(char* evaluator_lane);

cJSON *evaluator_lane_convertToJSON(palette_api_evaluator_lane__e evaluator_lane);

palette_api_evaluator_lane__e evaluator_lane_parseFromJSON(cJSON *evaluator_laneJSON);

#endif /* _evaluator_lane_H_ */

