#ifndef status_error_TEST
#define status_error_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define status_error_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/status_error.h"
status_error_t* instantiate_status_error(int include_optional);



status_error_t* instantiate_status_error(int include_optional) {
  status_error_t* status_error = NULL;
  if (include_optional) {
    status_error = status_error_create(
      56,
      list_createList(),
      "0",
      "0",
      palette_api_status_error_STATUS_CANCELLED
    );
  } else {
    status_error = status_error_create(
      56,
      list_createList(),
      "0",
      "0",
      palette_api_status_error_STATUS_CANCELLED
    );
  }

  return status_error;
}


#ifdef status_error_MAIN

void test_status_error(int include_optional) {
    status_error_t* status_error_1 = instantiate_status_error(include_optional);

	cJSON* jsonstatus_error_1 = status_error_convertToJSON(status_error_1);
	printf("status_error :\n%s\n", cJSON_Print(jsonstatus_error_1));
	status_error_t* status_error_2 = status_error_parseFromJSON(jsonstatus_error_1);
	cJSON* jsonstatus_error_2 = status_error_convertToJSON(status_error_2);
	printf("repeating status_error:\n%s\n", cJSON_Print(jsonstatus_error_2));
}

int main() {
  test_status_error(1);
  test_status_error(0);

  printf("Hello world \n");
  return 0;
}

#endif // status_error_MAIN
#endif // status_error_TEST
