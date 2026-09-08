#ifndef native_ingest_request_model_TEST
#define native_ingest_request_model_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define native_ingest_request_model_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/native_ingest_request_model.h"
native_ingest_request_model_t* instantiate_native_ingest_request_model(int include_optional);



native_ingest_request_model_t* instantiate_native_ingest_request_model(int include_optional) {
  native_ingest_request_model_t* native_ingest_request_model = NULL;
  if (include_optional) {
    native_ingest_request_model = native_ingest_request_model_create(
      "0",
      "0"
    );
  } else {
    native_ingest_request_model = native_ingest_request_model_create(
      "0",
      "0"
    );
  }

  return native_ingest_request_model;
}


#ifdef native_ingest_request_model_MAIN

void test_native_ingest_request_model(int include_optional) {
    native_ingest_request_model_t* native_ingest_request_model_1 = instantiate_native_ingest_request_model(include_optional);

	cJSON* jsonnative_ingest_request_model_1 = native_ingest_request_model_convertToJSON(native_ingest_request_model_1);
	printf("native_ingest_request_model :\n%s\n", cJSON_Print(jsonnative_ingest_request_model_1));
	native_ingest_request_model_t* native_ingest_request_model_2 = native_ingest_request_model_parseFromJSON(jsonnative_ingest_request_model_1);
	cJSON* jsonnative_ingest_request_model_2 = native_ingest_request_model_convertToJSON(native_ingest_request_model_2);
	printf("repeating native_ingest_request_model:\n%s\n", cJSON_Print(jsonnative_ingest_request_model_2));
}

int main() {
  test_native_ingest_request_model(1);
  test_native_ingest_request_model(0);

  printf("Hello world \n");
  return 0;
}

#endif // native_ingest_request_model_MAIN
#endif // native_ingest_request_model_TEST
