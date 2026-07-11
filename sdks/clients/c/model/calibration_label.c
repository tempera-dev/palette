#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "calibration_label.h"


char* calibration_label_calibration_label_ToString(palette_api_calibration_label__e calibration_label) {
    char *calibration_labelArray[] =  { "NULL", "pass", "fail" };
    return calibration_labelArray[calibration_label];
}

palette_api_calibration_label__e calibration_label_calibration_label_FromString(char* calibration_label) {
    int stringToReturn = 0;
    char *calibration_labelArray[] =  { "NULL", "pass", "fail" };
    size_t sizeofArray = sizeof(calibration_labelArray) / sizeof(calibration_labelArray[0]);
    while(stringToReturn < sizeofArray) {
        if(strcmp(calibration_label, calibration_labelArray[stringToReturn]) == 0) {
            return stringToReturn;
        }
        stringToReturn++;
    }
    return 0;
}

cJSON *calibration_label_convertToJSON(palette_api_calibration_label__e calibration_label) {
    cJSON *item = cJSON_CreateObject();
    if(cJSON_AddStringToObject(item, "calibration_label", calibration_label_calibration_label_ToString(calibration_label)) == NULL) {
        goto fail;
    }
    return item;
fail:
    cJSON_Delete(item);
    return NULL;
}

palette_api_calibration_label__e calibration_label_parseFromJSON(cJSON *calibration_labelJSON) {
    if(!cJSON_IsString(calibration_labelJSON) || (calibration_labelJSON->valuestring == NULL)) {
        return 0;
    }
    return calibration_label_calibration_label_FromString(calibration_labelJSON->valuestring);
}
