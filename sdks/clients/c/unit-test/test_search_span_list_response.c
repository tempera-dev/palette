#ifndef search_span_list_response_TEST
#define search_span_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define search_span_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/search_span_list_response.h"
search_span_list_response_t* instantiate_search_span_list_response(int include_optional);



search_span_list_response_t* instantiate_search_span_list_response(int include_optional) {
  search_span_list_response_t* search_span_list_response = NULL;
  if (include_optional) {
    search_span_list_response = search_span_list_response_create(
      list_createList(),
      "0"
    );
  } else {
    search_span_list_response = search_span_list_response_create(
      list_createList(),
      "0"
    );
  }

  return search_span_list_response;
}


#ifdef search_span_list_response_MAIN

void test_search_span_list_response(int include_optional) {
    search_span_list_response_t* search_span_list_response_1 = instantiate_search_span_list_response(include_optional);

	cJSON* jsonsearch_span_list_response_1 = search_span_list_response_convertToJSON(search_span_list_response_1);
	printf("search_span_list_response :\n%s\n", cJSON_Print(jsonsearch_span_list_response_1));
	search_span_list_response_t* search_span_list_response_2 = search_span_list_response_parseFromJSON(jsonsearch_span_list_response_1);
	cJSON* jsonsearch_span_list_response_2 = search_span_list_response_convertToJSON(search_span_list_response_2);
	printf("repeating search_span_list_response:\n%s\n", cJSON_Print(jsonsearch_span_list_response_2));
}

int main() {
  test_search_span_list_response(1);
  test_search_span_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // search_span_list_response_MAIN
#endif // search_span_list_response_TEST
