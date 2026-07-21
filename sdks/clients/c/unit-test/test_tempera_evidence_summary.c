#ifndef tempera_evidence_summary_TEST
#define tempera_evidence_summary_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define tempera_evidence_summary_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/tempera_evidence_summary.h"
tempera_evidence_summary_t* instantiate_tempera_evidence_summary(int include_optional);



tempera_evidence_summary_t* instantiate_tempera_evidence_summary(int include_optional) {
  tempera_evidence_summary_t* tempera_evidence_summary = NULL;
  if (include_optional) {
    tempera_evidence_summary = tempera_evidence_summary_create(
      "0",
      "0",
      "0",
      "0",
      "0",
      "0"
    );
  } else {
    tempera_evidence_summary = tempera_evidence_summary_create(
      "0",
      "0",
      "0",
      "0",
      "0",
      "0"
    );
  }

  return tempera_evidence_summary;
}


#ifdef tempera_evidence_summary_MAIN

void test_tempera_evidence_summary(int include_optional) {
    tempera_evidence_summary_t* tempera_evidence_summary_1 = instantiate_tempera_evidence_summary(include_optional);

	cJSON* jsontempera_evidence_summary_1 = tempera_evidence_summary_convertToJSON(tempera_evidence_summary_1);
	printf("tempera_evidence_summary :\n%s\n", cJSON_Print(jsontempera_evidence_summary_1));
	tempera_evidence_summary_t* tempera_evidence_summary_2 = tempera_evidence_summary_parseFromJSON(jsontempera_evidence_summary_1);
	cJSON* jsontempera_evidence_summary_2 = tempera_evidence_summary_convertToJSON(tempera_evidence_summary_2);
	printf("repeating tempera_evidence_summary:\n%s\n", cJSON_Print(jsontempera_evidence_summary_2));
}

int main() {
  test_tempera_evidence_summary(1);
  test_tempera_evidence_summary(0);

  printf("Hello world \n");
  return 0;
}

#endif // tempera_evidence_summary_MAIN
#endif // tempera_evidence_summary_TEST
