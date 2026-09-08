#ifndef native_ingest_request_tokens_TEST
#define native_ingest_request_tokens_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define native_ingest_request_tokens_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/native_ingest_request_tokens.h"
native_ingest_request_tokens_t* instantiate_native_ingest_request_tokens(int include_optional);



native_ingest_request_tokens_t* instantiate_native_ingest_request_tokens(int include_optional) {
  native_ingest_request_tokens_t* native_ingest_request_tokens = NULL;
  if (include_optional) {
    native_ingest_request_tokens = native_ingest_request_tokens_create(
      0,
      0,
      0,
      0
    );
  } else {
    native_ingest_request_tokens = native_ingest_request_tokens_create(
      0,
      0,
      0,
      0
    );
  }

  return native_ingest_request_tokens;
}


#ifdef native_ingest_request_tokens_MAIN

void test_native_ingest_request_tokens(int include_optional) {
    native_ingest_request_tokens_t* native_ingest_request_tokens_1 = instantiate_native_ingest_request_tokens(include_optional);

	cJSON* jsonnative_ingest_request_tokens_1 = native_ingest_request_tokens_convertToJSON(native_ingest_request_tokens_1);
	printf("native_ingest_request_tokens :\n%s\n", cJSON_Print(jsonnative_ingest_request_tokens_1));
	native_ingest_request_tokens_t* native_ingest_request_tokens_2 = native_ingest_request_tokens_parseFromJSON(jsonnative_ingest_request_tokens_1);
	cJSON* jsonnative_ingest_request_tokens_2 = native_ingest_request_tokens_convertToJSON(native_ingest_request_tokens_2);
	printf("repeating native_ingest_request_tokens:\n%s\n", cJSON_Print(jsonnative_ingest_request_tokens_2));
}

int main() {
  test_native_ingest_request_tokens(1);
  test_native_ingest_request_tokens(0);

  printf("Hello world \n");
  return 0;
}

#endif // native_ingest_request_tokens_MAIN
#endif // native_ingest_request_tokens_TEST
