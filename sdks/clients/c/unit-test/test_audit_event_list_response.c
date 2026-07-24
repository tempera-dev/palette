#ifndef audit_event_list_response_TEST
#define audit_event_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define audit_event_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/audit_event_list_response.h"
audit_event_list_response_t* instantiate_audit_event_list_response(int include_optional);



audit_event_list_response_t* instantiate_audit_event_list_response(int include_optional) {
  audit_event_list_response_t* audit_event_list_response = NULL;
  if (include_optional) {
    audit_event_list_response = audit_event_list_response_create(
      list_createList(),
      "0"
    );
  } else {
    audit_event_list_response = audit_event_list_response_create(
      list_createList(),
      "0"
    );
  }

  return audit_event_list_response;
}


#ifdef audit_event_list_response_MAIN

void test_audit_event_list_response(int include_optional) {
    audit_event_list_response_t* audit_event_list_response_1 = instantiate_audit_event_list_response(include_optional);

	cJSON* jsonaudit_event_list_response_1 = audit_event_list_response_convertToJSON(audit_event_list_response_1);
	printf("audit_event_list_response :\n%s\n", cJSON_Print(jsonaudit_event_list_response_1));
	audit_event_list_response_t* audit_event_list_response_2 = audit_event_list_response_parseFromJSON(jsonaudit_event_list_response_1);
	cJSON* jsonaudit_event_list_response_2 = audit_event_list_response_convertToJSON(audit_event_list_response_2);
	printf("repeating audit_event_list_response:\n%s\n", cJSON_Print(jsonaudit_event_list_response_2));
}

int main() {
  test_audit_event_list_response(1);
  test_audit_event_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // audit_event_list_response_MAIN
#endif // audit_event_list_response_TEST
