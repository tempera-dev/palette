#ifndef native_ingest_request_cost_TEST
#define native_ingest_request_cost_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define native_ingest_request_cost_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/native_ingest_request_cost.h"
native_ingest_request_cost_t* instantiate_native_ingest_request_cost(int include_optional);



native_ingest_request_cost_t* instantiate_native_ingest_request_cost(int include_optional) {
  native_ingest_request_cost_t* native_ingest_request_cost = NULL;
  if (include_optional) {
    native_ingest_request_cost = native_ingest_request_cost_create(
      56,
      palette_api_native_ingest_request_cost__USD
    );
  } else {
    native_ingest_request_cost = native_ingest_request_cost_create(
      56,
      palette_api_native_ingest_request_cost__USD
    );
  }

  return native_ingest_request_cost;
}


#ifdef native_ingest_request_cost_MAIN

void test_native_ingest_request_cost(int include_optional) {
    native_ingest_request_cost_t* native_ingest_request_cost_1 = instantiate_native_ingest_request_cost(include_optional);

	cJSON* jsonnative_ingest_request_cost_1 = native_ingest_request_cost_convertToJSON(native_ingest_request_cost_1);
	printf("native_ingest_request_cost :\n%s\n", cJSON_Print(jsonnative_ingest_request_cost_1));
	native_ingest_request_cost_t* native_ingest_request_cost_2 = native_ingest_request_cost_parseFromJSON(jsonnative_ingest_request_cost_1);
	cJSON* jsonnative_ingest_request_cost_2 = native_ingest_request_cost_convertToJSON(native_ingest_request_cost_2);
	printf("repeating native_ingest_request_cost:\n%s\n", cJSON_Print(jsonnative_ingest_request_cost_2));
}

int main() {
  test_native_ingest_request_cost(1);
  test_native_ingest_request_cost(0);

  printf("Hello world \n");
  return 0;
}

#endif // native_ingest_request_cost_MAIN
#endif // native_ingest_request_cost_TEST
