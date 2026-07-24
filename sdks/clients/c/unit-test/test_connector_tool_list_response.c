#ifndef connector_tool_list_response_TEST
#define connector_tool_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define connector_tool_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/connector_tool_list_response.h"
connector_tool_list_response_t* instantiate_connector_tool_list_response(int include_optional);



connector_tool_list_response_t* instantiate_connector_tool_list_response(int include_optional) {
  connector_tool_list_response_t* connector_tool_list_response = NULL;
  if (include_optional) {
    connector_tool_list_response = connector_tool_list_response_create(
      "0",
      list_createList()
    );
  } else {
    connector_tool_list_response = connector_tool_list_response_create(
      "0",
      list_createList()
    );
  }

  return connector_tool_list_response;
}


#ifdef connector_tool_list_response_MAIN

void test_connector_tool_list_response(int include_optional) {
    connector_tool_list_response_t* connector_tool_list_response_1 = instantiate_connector_tool_list_response(include_optional);

	cJSON* jsonconnector_tool_list_response_1 = connector_tool_list_response_convertToJSON(connector_tool_list_response_1);
	printf("connector_tool_list_response :\n%s\n", cJSON_Print(jsonconnector_tool_list_response_1));
	connector_tool_list_response_t* connector_tool_list_response_2 = connector_tool_list_response_parseFromJSON(jsonconnector_tool_list_response_1);
	cJSON* jsonconnector_tool_list_response_2 = connector_tool_list_response_convertToJSON(connector_tool_list_response_2);
	printf("repeating connector_tool_list_response:\n%s\n", cJSON_Print(jsonconnector_tool_list_response_2));
}

int main() {
  test_connector_tool_list_response(1);
  test_connector_tool_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // connector_tool_list_response_MAIN
#endif // connector_tool_list_response_TEST
