#ifndef native_ingest_request_parent_span_id_TEST
#define native_ingest_request_parent_span_id_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define native_ingest_request_parent_span_id_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/native_ingest_request_parent_span_id.h"
native_ingest_request_parent_span_id_t* instantiate_native_ingest_request_parent_span_id(int include_optional);



native_ingest_request_parent_span_id_t* instantiate_native_ingest_request_parent_span_id(int include_optional) {
  native_ingest_request_parent_span_id_t* native_ingest_request_parent_span_id = NULL;
  if (include_optional) {
    native_ingest_request_parent_span_id = native_ingest_request_parent_span_id_create(
    );
  } else {
    native_ingest_request_parent_span_id = native_ingest_request_parent_span_id_create(
    );
  }

  return native_ingest_request_parent_span_id;
}


#ifdef native_ingest_request_parent_span_id_MAIN

void test_native_ingest_request_parent_span_id(int include_optional) {
    native_ingest_request_parent_span_id_t* native_ingest_request_parent_span_id_1 = instantiate_native_ingest_request_parent_span_id(include_optional);

	cJSON* jsonnative_ingest_request_parent_span_id_1 = native_ingest_request_parent_span_id_convertToJSON(native_ingest_request_parent_span_id_1);
	printf("native_ingest_request_parent_span_id :\n%s\n", cJSON_Print(jsonnative_ingest_request_parent_span_id_1));
	native_ingest_request_parent_span_id_t* native_ingest_request_parent_span_id_2 = native_ingest_request_parent_span_id_parseFromJSON(jsonnative_ingest_request_parent_span_id_1);
	cJSON* jsonnative_ingest_request_parent_span_id_2 = native_ingest_request_parent_span_id_convertToJSON(native_ingest_request_parent_span_id_2);
	printf("repeating native_ingest_request_parent_span_id:\n%s\n", cJSON_Print(jsonnative_ingest_request_parent_span_id_2));
}

int main() {
  test_native_ingest_request_parent_span_id(1);
  test_native_ingest_request_parent_span_id(0);

  printf("Hello world \n");
  return 0;
}

#endif // native_ingest_request_parent_span_id_MAIN
#endif // native_ingest_request_parent_span_id_TEST
