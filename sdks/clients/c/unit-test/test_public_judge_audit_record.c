#ifndef public_judge_audit_record_TEST
#define public_judge_audit_record_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define public_judge_audit_record_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/public_judge_audit_record.h"
public_judge_audit_record_t* instantiate_public_judge_audit_record(int include_optional);

#include "test_money.c"


public_judge_audit_record_t* instantiate_public_judge_audit_record(int include_optional) {
  public_judge_audit_record_t* public_judge_audit_record = NULL;
  if (include_optional) {
    public_judge_audit_record = public_judge_audit_record_create(
      1,
       // false, not to have infinite recursion
      instantiate_money(0),
      "2013-10-20T19:20:30+01:00",
      "0",
      "0",
      "0",
      "0",
      "0",
      "0",
      1.337,
      "0"
    );
  } else {
    public_judge_audit_record = public_judge_audit_record_create(
      1,
      NULL,
      "2013-10-20T19:20:30+01:00",
      "0",
      "0",
      "0",
      "0",
      "0",
      "0",
      1.337,
      "0"
    );
  }

  return public_judge_audit_record;
}


#ifdef public_judge_audit_record_MAIN

void test_public_judge_audit_record(int include_optional) {
    public_judge_audit_record_t* public_judge_audit_record_1 = instantiate_public_judge_audit_record(include_optional);

	cJSON* jsonpublic_judge_audit_record_1 = public_judge_audit_record_convertToJSON(public_judge_audit_record_1);
	printf("public_judge_audit_record :\n%s\n", cJSON_Print(jsonpublic_judge_audit_record_1));
	public_judge_audit_record_t* public_judge_audit_record_2 = public_judge_audit_record_parseFromJSON(jsonpublic_judge_audit_record_1);
	cJSON* jsonpublic_judge_audit_record_2 = public_judge_audit_record_convertToJSON(public_judge_audit_record_2);
	printf("repeating public_judge_audit_record:\n%s\n", cJSON_Print(jsonpublic_judge_audit_record_2));
}

int main() {
  test_public_judge_audit_record(1);
  test_public_judge_audit_record(0);

  printf("Hello world \n");
  return 0;
}

#endif // public_judge_audit_record_MAIN
#endif // public_judge_audit_record_TEST
