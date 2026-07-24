#ifndef error_status_TEST
#define error_status_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define error_status_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/error_status.h"
error_status_t* instantiate_error_status(int include_optional);



error_status_t* instantiate_error_status(int include_optional) {
  error_status_t* error_status = NULL;
  if (include_optional) {
    error_status = error_status_create(
      56,
      list_createList(),
      "0",
      "0"
    );
  } else {
    error_status = error_status_create(
      56,
      list_createList(),
      "0",
      "0"
    );
  }

  return error_status;
}


#ifdef error_status_MAIN

void test_error_status(int include_optional) {
    error_status_t* error_status_1 = instantiate_error_status(include_optional);

	cJSON* jsonerror_status_1 = error_status_convertToJSON(error_status_1);
	printf("error_status :\n%s\n", cJSON_Print(jsonerror_status_1));
	error_status_t* error_status_2 = error_status_parseFromJSON(jsonerror_status_1);
	cJSON* jsonerror_status_2 = error_status_convertToJSON(error_status_2);
	printf("repeating error_status:\n%s\n", cJSON_Print(jsonerror_status_2));
}

int main() {
  test_error_status(1);
  test_error_status(0);

  printf("Hello world \n");
  return 0;
}

#endif // error_status_MAIN
#endif // error_status_TEST
