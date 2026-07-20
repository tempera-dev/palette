#ifndef import_tempera_evidence_request_TEST
#define import_tempera_evidence_request_TEST

// the following is to include only the main from the first c file
#ifndef TEST_MAIN
#define TEST_MAIN
#define import_tempera_evidence_request_MAIN
#endif // TEST_MAIN

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include "../external/cJSON.h"

#include "../model/import_tempera_evidence_request.h"
import_tempera_evidence_request_t* instantiate_import_tempera_evidence_request(int include_optional);



import_tempera_evidence_request_t* instantiate_import_tempera_evidence_request(int include_optional) {
  import_tempera_evidence_request_t* import_tempera_evidence_request = NULL;
  if (include_optional) {
    import_tempera_evidence_request = import_tempera_evidence_request_create(
      "0",
      "0",
      "0"
    );
  } else {
    import_tempera_evidence_request = import_tempera_evidence_request_create(
      "0",
      "0",
      "0"
    );
  }

  return import_tempera_evidence_request;
}


#ifdef import_tempera_evidence_request_MAIN

void test_import_tempera_evidence_request(int include_optional) {
    import_tempera_evidence_request_t* import_tempera_evidence_request_1 = instantiate_import_tempera_evidence_request(include_optional);

	cJSON* jsonimport_tempera_evidence_request_1 = import_tempera_evidence_request_convertToJSON(import_tempera_evidence_request_1);
	printf("import_tempera_evidence_request :\n%s\n", cJSON_Print(jsonimport_tempera_evidence_request_1));
	import_tempera_evidence_request_t* import_tempera_evidence_request_2 = import_tempera_evidence_request_parseFromJSON(jsonimport_tempera_evidence_request_1);
	cJSON* jsonimport_tempera_evidence_request_2 = import_tempera_evidence_request_convertToJSON(import_tempera_evidence_request_2);
	printf("repeating import_tempera_evidence_request:\n%s\n", cJSON_Print(jsonimport_tempera_evidence_request_2));
}

int main() {
  test_import_tempera_evidence_request(1);
  test_import_tempera_evidence_request(0);

  printf("Hello world \n");
  return 0;
}

#endif // import_tempera_evidence_request_MAIN
#endif // import_tempera_evidence_request_TEST
