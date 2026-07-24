#ifndef provider_secret_list_response_TEST
#define provider_secret_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define provider_secret_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/provider_secret_list_response.h"
provider_secret_list_response_t* instantiate_provider_secret_list_response(int include_optional);



provider_secret_list_response_t* instantiate_provider_secret_list_response(int include_optional) {
  provider_secret_list_response_t* provider_secret_list_response = NULL;
  if (include_optional) {
    provider_secret_list_response = provider_secret_list_response_create(
      "0",
      list_createList()
    );
  } else {
    provider_secret_list_response = provider_secret_list_response_create(
      "0",
      list_createList()
    );
  }

  return provider_secret_list_response;
}


#ifdef provider_secret_list_response_MAIN

void test_provider_secret_list_response(int include_optional) {
    provider_secret_list_response_t* provider_secret_list_response_1 = instantiate_provider_secret_list_response(include_optional);

	cJSON* jsonprovider_secret_list_response_1 = provider_secret_list_response_convertToJSON(provider_secret_list_response_1);
	printf("provider_secret_list_response :\n%s\n", cJSON_Print(jsonprovider_secret_list_response_1));
	provider_secret_list_response_t* provider_secret_list_response_2 = provider_secret_list_response_parseFromJSON(jsonprovider_secret_list_response_1);
	cJSON* jsonprovider_secret_list_response_2 = provider_secret_list_response_convertToJSON(provider_secret_list_response_2);
	printf("repeating provider_secret_list_response:\n%s\n", cJSON_Print(jsonprovider_secret_list_response_2));
}

int main() {
  test_provider_secret_list_response(1);
  test_provider_secret_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // provider_secret_list_response_MAIN
#endif // provider_secret_list_response_TEST
