#ifndef canonical_span_output_ref_TEST
#define canonical_span_output_ref_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define canonical_span_output_ref_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/canonical_span_output_ref.h"
canonical_span_output_ref_t* instantiate_canonical_span_output_ref(int include_optional);



canonical_span_output_ref_t* instantiate_canonical_span_output_ref(int include_optional) {
  canonical_span_output_ref_t* canonical_span_output_ref = NULL;
  if (include_optional) {
    canonical_span_output_ref = canonical_span_output_ref_create(
      "0",
      "0",
      palette_api_canonical_span_output_ref__public,
      "0",
      0,
      "0"
    );
  } else {
    canonical_span_output_ref = canonical_span_output_ref_create(
      "0",
      "0",
      palette_api_canonical_span_output_ref__public,
      "0",
      0,
      "0"
    );
  }

  return canonical_span_output_ref;
}


#ifdef canonical_span_output_ref_MAIN

void test_canonical_span_output_ref(int include_optional) {
    canonical_span_output_ref_t* canonical_span_output_ref_1 = instantiate_canonical_span_output_ref(include_optional);

	cJSON* jsoncanonical_span_output_ref_1 = canonical_span_output_ref_convertToJSON(canonical_span_output_ref_1);
	printf("canonical_span_output_ref :\n%s\n", cJSON_Print(jsoncanonical_span_output_ref_1));
	canonical_span_output_ref_t* canonical_span_output_ref_2 = canonical_span_output_ref_parseFromJSON(jsoncanonical_span_output_ref_1);
	cJSON* jsoncanonical_span_output_ref_2 = canonical_span_output_ref_convertToJSON(canonical_span_output_ref_2);
	printf("repeating canonical_span_output_ref:\n%s\n", cJSON_Print(jsoncanonical_span_output_ref_2));
}

int main() {
  test_canonical_span_output_ref(1);
  test_canonical_span_output_ref(0);

  printf("Hello world \n");
  return 0;
}

#endif // canonical_span_output_ref_MAIN
#endif // canonical_span_output_ref_TEST
