#ifndef trace_list_response_TEST
#define trace_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define trace_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/trace_list_response.h"
trace_list_response_t* instantiate_trace_list_response(int include_optional);



trace_list_response_t* instantiate_trace_list_response(int include_optional) {
  trace_list_response_t* trace_list_response = NULL;
  if (include_optional) {
    trace_list_response = trace_list_response_create(
      "0",
      list_createList()
    );
  } else {
    trace_list_response = trace_list_response_create(
      "0",
      list_createList()
    );
  }

  return trace_list_response;
}


#ifdef trace_list_response_MAIN

void test_trace_list_response(int include_optional) {
    trace_list_response_t* trace_list_response_1 = instantiate_trace_list_response(include_optional);

	cJSON* jsontrace_list_response_1 = trace_list_response_convertToJSON(trace_list_response_1);
	printf("trace_list_response :\n%s\n", cJSON_Print(jsontrace_list_response_1));
	trace_list_response_t* trace_list_response_2 = trace_list_response_parseFromJSON(jsontrace_list_response_1);
	cJSON* jsontrace_list_response_2 = trace_list_response_convertToJSON(trace_list_response_2);
	printf("repeating trace_list_response:\n%s\n", cJSON_Print(jsontrace_list_response_2));
}

int main() {
  test_trace_list_response(1);
  test_trace_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // trace_list_response_MAIN
#endif // trace_list_response_TEST
