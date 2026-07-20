#ifndef tempera_evidence_receipt_TEST
#define tempera_evidence_receipt_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define tempera_evidence_receipt_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/tempera_evidence_receipt.h"
tempera_evidence_receipt_t* instantiate_tempera_evidence_receipt(int include_optional);

#include "test_tempera_evidence_summary.c"


tempera_evidence_receipt_t* instantiate_tempera_evidence_receipt(int include_optional) {
  tempera_evidence_receipt_t* tempera_evidence_receipt = NULL;
  if (include_optional) {
    tempera_evidence_receipt = tempera_evidence_receipt_create(
      1,
      "0",
      "0",
      palette_api_tempera_evidence_receipt__result_bundle,
      "0",
      "0",
      "0",
      "0",
      "0",
      "0",
      "2013-10-20T19:20:30+01:00",
       // false, not to have infinite recursion
      instantiate_tempera_evidence_summary(0),
      "0"
    );
  } else {
    tempera_evidence_receipt = tempera_evidence_receipt_create(
      1,
      "0",
      "0",
      palette_api_tempera_evidence_receipt__result_bundle,
      "0",
      "0",
      "0",
      "0",
      "0",
      "0",
      "2013-10-20T19:20:30+01:00",
      NULL,
      "0"
    );
  }

  return tempera_evidence_receipt;
}


#ifdef tempera_evidence_receipt_MAIN

void test_tempera_evidence_receipt(int include_optional) {
    tempera_evidence_receipt_t* tempera_evidence_receipt_1 = instantiate_tempera_evidence_receipt(include_optional);

	cJSON* jsontempera_evidence_receipt_1 = tempera_evidence_receipt_convertToJSON(tempera_evidence_receipt_1);
	printf("tempera_evidence_receipt :\n%s\n", cJSON_Print(jsontempera_evidence_receipt_1));
	tempera_evidence_receipt_t* tempera_evidence_receipt_2 = tempera_evidence_receipt_parseFromJSON(jsontempera_evidence_receipt_1);
	cJSON* jsontempera_evidence_receipt_2 = tempera_evidence_receipt_convertToJSON(tempera_evidence_receipt_2);
	printf("repeating tempera_evidence_receipt:\n%s\n", cJSON_Print(jsontempera_evidence_receipt_2));
}

int main() {
  test_tempera_evidence_receipt(1);
  test_tempera_evidence_receipt(0);

  printf("Hello world \n");
  return 0;
}

#endif // tempera_evidence_receipt_MAIN
#endif // tempera_evidence_receipt_TEST
