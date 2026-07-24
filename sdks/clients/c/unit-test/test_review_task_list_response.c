#ifndef review_task_list_response_TEST
#define review_task_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define review_task_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/review_task_list_response.h"
review_task_list_response_t* instantiate_review_task_list_response(int include_optional);



review_task_list_response_t* instantiate_review_task_list_response(int include_optional) {
  review_task_list_response_t* review_task_list_response = NULL;
  if (include_optional) {
    review_task_list_response = review_task_list_response_create(
      "0",
      list_createList()
    );
  } else {
    review_task_list_response = review_task_list_response_create(
      "0",
      list_createList()
    );
  }

  return review_task_list_response;
}


#ifdef review_task_list_response_MAIN

void test_review_task_list_response(int include_optional) {
    review_task_list_response_t* review_task_list_response_1 = instantiate_review_task_list_response(include_optional);

	cJSON* jsonreview_task_list_response_1 = review_task_list_response_convertToJSON(review_task_list_response_1);
	printf("review_task_list_response :\n%s\n", cJSON_Print(jsonreview_task_list_response_1));
	review_task_list_response_t* review_task_list_response_2 = review_task_list_response_parseFromJSON(jsonreview_task_list_response_1);
	cJSON* jsonreview_task_list_response_2 = review_task_list_response_convertToJSON(review_task_list_response_2);
	printf("repeating review_task_list_response:\n%s\n", cJSON_Print(jsonreview_task_list_response_2));
}

int main() {
  test_review_task_list_response(1);
  test_review_task_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // review_task_list_response_MAIN
#endif // review_task_list_response_TEST
