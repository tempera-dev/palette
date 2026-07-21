#ifndef external_eval_evidence_kind_TEST
#define external_eval_evidence_kind_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define external_eval_evidence_kind_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/external_eval_evidence_kind.h"
external_eval_evidence_kind_t* instantiate_external_eval_evidence_kind(int include_optional);



external_eval_evidence_kind_t* instantiate_external_eval_evidence_kind(int include_optional) {
  external_eval_evidence_kind_t* external_eval_evidence_kind = NULL;
  if (include_optional) {
    external_eval_evidence_kind = external_eval_evidence_kind_create(
    );
  } else {
    external_eval_evidence_kind = external_eval_evidence_kind_create(
    );
  }

  return external_eval_evidence_kind;
}


#ifdef external_eval_evidence_kind_MAIN

void test_external_eval_evidence_kind(int include_optional) {
    external_eval_evidence_kind_t* external_eval_evidence_kind_1 = instantiate_external_eval_evidence_kind(include_optional);

	cJSON* jsonexternal_eval_evidence_kind_1 = external_eval_evidence_kind_convertToJSON(external_eval_evidence_kind_1);
	printf("external_eval_evidence_kind :\n%s\n", cJSON_Print(jsonexternal_eval_evidence_kind_1));
	external_eval_evidence_kind_t* external_eval_evidence_kind_2 = external_eval_evidence_kind_parseFromJSON(jsonexternal_eval_evidence_kind_1);
	cJSON* jsonexternal_eval_evidence_kind_2 = external_eval_evidence_kind_convertToJSON(external_eval_evidence_kind_2);
	printf("repeating external_eval_evidence_kind:\n%s\n", cJSON_Print(jsonexternal_eval_evidence_kind_2));
}

int main() {
  test_external_eval_evidence_kind(1);
  test_external_eval_evidence_kind(0);

  printf("Hello world \n");
  return 0;
}

#endif // external_eval_evidence_kind_MAIN
#endif // external_eval_evidence_kind_TEST
