#ifndef connector_list_response_TEST
#define connector_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define connector_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/connector_list_response.h"
connector_list_response_t* instantiate_connector_list_response(int include_optional);



connector_list_response_t* instantiate_connector_list_response(int include_optional) {
  connector_list_response_t* connector_list_response = NULL;
  if (include_optional) {
    connector_list_response = connector_list_response_create(
      "0",
      list_createList()
    );
  } else {
    connector_list_response = connector_list_response_create(
      "0",
      list_createList()
    );
  }

  return connector_list_response;
}


#ifdef connector_list_response_MAIN

void test_connector_list_response(int include_optional) {
    connector_list_response_t* connector_list_response_1 = instantiate_connector_list_response(include_optional);

	cJSON* jsonconnector_list_response_1 = connector_list_response_convertToJSON(connector_list_response_1);
	printf("connector_list_response :\n%s\n", cJSON_Print(jsonconnector_list_response_1));
	connector_list_response_t* connector_list_response_2 = connector_list_response_parseFromJSON(jsonconnector_list_response_1);
	cJSON* jsonconnector_list_response_2 = connector_list_response_convertToJSON(connector_list_response_2);
	printf("repeating connector_list_response:\n%s\n", cJSON_Print(jsonconnector_list_response_2));
}

int main() {
  test_connector_list_response(1);
  test_connector_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // connector_list_response_MAIN
#endif // connector_list_response_TEST
