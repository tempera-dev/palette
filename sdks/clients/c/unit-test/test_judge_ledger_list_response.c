#ifndef judge_ledger_list_response_TEST
#define judge_ledger_list_response_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define judge_ledger_list_response_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/judge_ledger_list_response.h"
judge_ledger_list_response_t* instantiate_judge_ledger_list_response(int include_optional);



judge_ledger_list_response_t* instantiate_judge_ledger_list_response(int include_optional) {
  judge_ledger_list_response_t* judge_ledger_list_response = NULL;
  if (include_optional) {
    judge_ledger_list_response = judge_ledger_list_response_create(
      "0",
      list_createList()
    );
  } else {
    judge_ledger_list_response = judge_ledger_list_response_create(
      "0",
      list_createList()
    );
  }

  return judge_ledger_list_response;
}


#ifdef judge_ledger_list_response_MAIN

void test_judge_ledger_list_response(int include_optional) {
    judge_ledger_list_response_t* judge_ledger_list_response_1 = instantiate_judge_ledger_list_response(include_optional);

	cJSON* jsonjudge_ledger_list_response_1 = judge_ledger_list_response_convertToJSON(judge_ledger_list_response_1);
	printf("judge_ledger_list_response :\n%s\n", cJSON_Print(jsonjudge_ledger_list_response_1));
	judge_ledger_list_response_t* judge_ledger_list_response_2 = judge_ledger_list_response_parseFromJSON(jsonjudge_ledger_list_response_1);
	cJSON* jsonjudge_ledger_list_response_2 = judge_ledger_list_response_convertToJSON(judge_ledger_list_response_2);
	printf("repeating judge_ledger_list_response:\n%s\n", cJSON_Print(jsonjudge_ledger_list_response_2));
}

int main() {
  test_judge_ledger_list_response(1);
  test_judge_ledger_list_response(0);

  printf("Hello world \n");
  return 0;
}

#endif // judge_ledger_list_response_MAIN
#endif // judge_ledger_list_response_TEST
