#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include "PromptsAPI.h"

#define MAX_NUMBER_LENGTH 16
#define MAX_BUFFER_LENGTH 4096


prompt_version_t*
PromptsAPI_promptsAddVersion(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, add_prompt_version_request_t *add_prompt_version_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
{
    list_t    *localVarQueryParameters = NULL;
    list_t    *localVarHeaderParameters = list_createList();
    list_t    *localVarFormParameters = NULL;
    list_t *localVarHeaderType = list_createList();
    list_t *localVarContentType = list_createList();
    char      *localVarBodyParameters = NULL;
    size_t     localVarBodyLength = 0;

    // clear the error code from the previous api call
    apiClient->response_code = 0;

    // create the path
    char *localVarPath = strdup("/v1/prompts/{tenantId}/{projectId}/{promptId}/versions");

    if(!tenantId)
        goto end;
    if(!projectId)
        goto end;
    if(!promptId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_projectId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ projectId }") - 1;
    if(projectId == NULL) {
        goto end;
    }
    char* localVarToReplace_projectId = malloc(sizeOfPathParams_projectId);
    sprintf(localVarToReplace_projectId, "{%s}", "projectId");

    localVarPath = strReplace(localVarPath, localVarToReplace_projectId, projectId);

    // Path Params
    long sizeOfPathParams_promptId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ promptId }") - 1;
    if(promptId == NULL) {
        goto end;
    }
    char* localVarToReplace_promptId = malloc(sizeOfPathParams_promptId);
    sprintf(localVarToReplace_promptId, "{%s}", "promptId");

    localVarPath = strReplace(localVarPath, localVarToReplace_promptId, promptId);



    // header parameters
    char *keyHeader_authorization = NULL;
    char * valueHeader_authorization = 0;
    keyValuePair_t *keyPairHeader_authorization = 0;
    if (authorization) {
        keyHeader_authorization = strdup("authorization");
        valueHeader_authorization = strdup((authorization));
        keyPairHeader_authorization = keyValuePair_create(keyHeader_authorization, valueHeader_authorization);
        list_addElement(localVarHeaderParameters,keyPairHeader_authorization);
    }


    // header parameters
    char *keyHeader_x_palette_api_key = NULL;
    char * valueHeader_x_palette_api_key = 0;
    keyValuePair_t *keyPairHeader_x_palette_api_key = 0;
    if (x_palette_api_key) {
        keyHeader_x_palette_api_key = strdup("x-palette-api-key");
        valueHeader_x_palette_api_key = strdup((x_palette_api_key));
        keyPairHeader_x_palette_api_key = keyValuePair_create(keyHeader_x_palette_api_key, valueHeader_x_palette_api_key);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_api_key);
    }


    // header parameters
    char *keyHeader_x_palette_project_id = NULL;
    char * valueHeader_x_palette_project_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_project_id = 0;
    if (x_palette_project_id) {
        keyHeader_x_palette_project_id = strdup("x-palette-project-id");
        valueHeader_x_palette_project_id = strdup((x_palette_project_id));
        keyPairHeader_x_palette_project_id = keyValuePair_create(keyHeader_x_palette_project_id, valueHeader_x_palette_project_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_project_id);
    }


    // header parameters
    char *keyHeader_x_palette_environment_id = NULL;
    char * valueHeader_x_palette_environment_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_environment_id = 0;
    if (x_palette_environment_id) {
        keyHeader_x_palette_environment_id = strdup("x-palette-environment-id");
        valueHeader_x_palette_environment_id = strdup((x_palette_environment_id));
        keyPairHeader_x_palette_environment_id = keyValuePair_create(keyHeader_x_palette_environment_id, valueHeader_x_palette_environment_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_environment_id);
    }


    // Body Param
    cJSON *localVarSingleItemJSON_add_prompt_version_request = NULL;
    if (add_prompt_version_request != NULL)
    {
        //not string, not binary
        localVarSingleItemJSON_add_prompt_version_request = add_prompt_version_request_convertToJSON(add_prompt_version_request);
        localVarBodyParameters = cJSON_Print(localVarSingleItemJSON_add_prompt_version_request);
        localVarBodyLength = strlen(localVarBodyParameters);
    }
    list_addElement(localVarHeaderType,"application/json"); //produces
    list_addElement(localVarContentType,"application/json"); //consumes
    apiClient_invoke(apiClient,
                    localVarPath,
                    localVarQueryParameters,
                    localVarHeaderParameters,
                    localVarFormParameters,
                    localVarHeaderType,
                    localVarContentType,
                    localVarBodyParameters,
                    localVarBodyLength,
                    "POST");

    // uncomment below to debug the error response
    //if (apiClient->response_code == 200) {
    //    printf("%s\n","Append an immutable prompt version");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 404) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 0) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    //nonprimitive not container
    prompt_version_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *PromptsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = prompt_version_parseFromJSON(PromptsAPIlocalVarJSON);
        cJSON_Delete(PromptsAPIlocalVarJSON);
        if(elementToReturn == NULL) {
            // return 0;
        }
    }

    //return type
    if (apiClient->dataReceived) {
        free(apiClient->dataReceived);
        apiClient->dataReceived = NULL;
        apiClient->dataReceivedLen = 0;
    }

    list_freeList(localVarHeaderParameters);

    list_freeList(localVarHeaderType);
    list_freeList(localVarContentType);
    free(localVarPath);
    free(localVarToReplace_tenantId);
    free(localVarToReplace_projectId);
    free(localVarToReplace_promptId);
    if (keyHeader_authorization) {
        free(keyHeader_authorization);
        keyHeader_authorization = NULL;
    }
    if (valueHeader_authorization) {
        free(valueHeader_authorization);
        valueHeader_authorization = NULL;
    }
    free(keyPairHeader_authorization);
    if (keyHeader_x_palette_api_key) {
        free(keyHeader_x_palette_api_key);
        keyHeader_x_palette_api_key = NULL;
    }
    if (valueHeader_x_palette_api_key) {
        free(valueHeader_x_palette_api_key);
        valueHeader_x_palette_api_key = NULL;
    }
    free(keyPairHeader_x_palette_api_key);
    if (keyHeader_x_palette_project_id) {
        free(keyHeader_x_palette_project_id);
        keyHeader_x_palette_project_id = NULL;
    }
    if (valueHeader_x_palette_project_id) {
        free(valueHeader_x_palette_project_id);
        valueHeader_x_palette_project_id = NULL;
    }
    free(keyPairHeader_x_palette_project_id);
    if (keyHeader_x_palette_environment_id) {
        free(keyHeader_x_palette_environment_id);
        keyHeader_x_palette_environment_id = NULL;
    }
    if (valueHeader_x_palette_environment_id) {
        free(valueHeader_x_palette_environment_id);
        valueHeader_x_palette_environment_id = NULL;
    }
    free(keyPairHeader_x_palette_environment_id);
    if (localVarSingleItemJSON_add_prompt_version_request) {
        cJSON_Delete(localVarSingleItemJSON_add_prompt_version_request);
        localVarSingleItemJSON_add_prompt_version_request = NULL;
    }
    free(localVarBodyParameters);
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

created_prompt_t*
PromptsAPI_promptsCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_prompt_request_t *create_prompt_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
{
    list_t    *localVarQueryParameters = NULL;
    list_t    *localVarHeaderParameters = list_createList();
    list_t    *localVarFormParameters = NULL;
    list_t *localVarHeaderType = list_createList();
    list_t *localVarContentType = list_createList();
    char      *localVarBodyParameters = NULL;
    size_t     localVarBodyLength = 0;

    // clear the error code from the previous api call
    apiClient->response_code = 0;

    // create the path
    char *localVarPath = strdup("/v1/prompts/{tenantId}/{projectId}");

    if(!tenantId)
        goto end;
    if(!projectId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(projectId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_projectId = strlen(tenantId)+3 + strlen(projectId)+3 + sizeof("{ projectId }") - 1;
    if(projectId == NULL) {
        goto end;
    }
    char* localVarToReplace_projectId = malloc(sizeOfPathParams_projectId);
    sprintf(localVarToReplace_projectId, "{%s}", "projectId");

    localVarPath = strReplace(localVarPath, localVarToReplace_projectId, projectId);



    // header parameters
    char *keyHeader_authorization = NULL;
    char * valueHeader_authorization = 0;
    keyValuePair_t *keyPairHeader_authorization = 0;
    if (authorization) {
        keyHeader_authorization = strdup("authorization");
        valueHeader_authorization = strdup((authorization));
        keyPairHeader_authorization = keyValuePair_create(keyHeader_authorization, valueHeader_authorization);
        list_addElement(localVarHeaderParameters,keyPairHeader_authorization);
    }


    // header parameters
    char *keyHeader_x_palette_api_key = NULL;
    char * valueHeader_x_palette_api_key = 0;
    keyValuePair_t *keyPairHeader_x_palette_api_key = 0;
    if (x_palette_api_key) {
        keyHeader_x_palette_api_key = strdup("x-palette-api-key");
        valueHeader_x_palette_api_key = strdup((x_palette_api_key));
        keyPairHeader_x_palette_api_key = keyValuePair_create(keyHeader_x_palette_api_key, valueHeader_x_palette_api_key);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_api_key);
    }


    // header parameters
    char *keyHeader_x_palette_project_id = NULL;
    char * valueHeader_x_palette_project_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_project_id = 0;
    if (x_palette_project_id) {
        keyHeader_x_palette_project_id = strdup("x-palette-project-id");
        valueHeader_x_palette_project_id = strdup((x_palette_project_id));
        keyPairHeader_x_palette_project_id = keyValuePair_create(keyHeader_x_palette_project_id, valueHeader_x_palette_project_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_project_id);
    }


    // header parameters
    char *keyHeader_x_palette_environment_id = NULL;
    char * valueHeader_x_palette_environment_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_environment_id = 0;
    if (x_palette_environment_id) {
        keyHeader_x_palette_environment_id = strdup("x-palette-environment-id");
        valueHeader_x_palette_environment_id = strdup((x_palette_environment_id));
        keyPairHeader_x_palette_environment_id = keyValuePair_create(keyHeader_x_palette_environment_id, valueHeader_x_palette_environment_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_environment_id);
    }


    // Body Param
    cJSON *localVarSingleItemJSON_create_prompt_request = NULL;
    if (create_prompt_request != NULL)
    {
        //not string, not binary
        localVarSingleItemJSON_create_prompt_request = create_prompt_request_convertToJSON(create_prompt_request);
        localVarBodyParameters = cJSON_Print(localVarSingleItemJSON_create_prompt_request);
        localVarBodyLength = strlen(localVarBodyParameters);
    }
    list_addElement(localVarHeaderType,"application/json"); //produces
    list_addElement(localVarContentType,"application/json"); //consumes
    apiClient_invoke(apiClient,
                    localVarPath,
                    localVarQueryParameters,
                    localVarHeaderParameters,
                    localVarFormParameters,
                    localVarHeaderType,
                    localVarContentType,
                    localVarBodyParameters,
                    localVarBodyLength,
                    "POST");

    // uncomment below to debug the error response
    //if (apiClient->response_code == 200) {
    //    printf("%s\n","Create a prompt and its initial version");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 0) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    //nonprimitive not container
    created_prompt_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *PromptsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = created_prompt_parseFromJSON(PromptsAPIlocalVarJSON);
        cJSON_Delete(PromptsAPIlocalVarJSON);
        if(elementToReturn == NULL) {
            // return 0;
        }
    }

    //return type
    if (apiClient->dataReceived) {
        free(apiClient->dataReceived);
        apiClient->dataReceived = NULL;
        apiClient->dataReceivedLen = 0;
    }

    list_freeList(localVarHeaderParameters);

    list_freeList(localVarHeaderType);
    list_freeList(localVarContentType);
    free(localVarPath);
    free(localVarToReplace_tenantId);
    free(localVarToReplace_projectId);
    if (keyHeader_authorization) {
        free(keyHeader_authorization);
        keyHeader_authorization = NULL;
    }
    if (valueHeader_authorization) {
        free(valueHeader_authorization);
        valueHeader_authorization = NULL;
    }
    free(keyPairHeader_authorization);
    if (keyHeader_x_palette_api_key) {
        free(keyHeader_x_palette_api_key);
        keyHeader_x_palette_api_key = NULL;
    }
    if (valueHeader_x_palette_api_key) {
        free(valueHeader_x_palette_api_key);
        valueHeader_x_palette_api_key = NULL;
    }
    free(keyPairHeader_x_palette_api_key);
    if (keyHeader_x_palette_project_id) {
        free(keyHeader_x_palette_project_id);
        keyHeader_x_palette_project_id = NULL;
    }
    if (valueHeader_x_palette_project_id) {
        free(valueHeader_x_palette_project_id);
        valueHeader_x_palette_project_id = NULL;
    }
    free(keyPairHeader_x_palette_project_id);
    if (keyHeader_x_palette_environment_id) {
        free(keyHeader_x_palette_environment_id);
        keyHeader_x_palette_environment_id = NULL;
    }
    if (valueHeader_x_palette_environment_id) {
        free(valueHeader_x_palette_environment_id);
        valueHeader_x_palette_environment_id = NULL;
    }
    free(keyPairHeader_x_palette_environment_id);
    if (localVarSingleItemJSON_create_prompt_request) {
        cJSON_Delete(localVarSingleItemJSON_create_prompt_request);
        localVarSingleItemJSON_create_prompt_request = NULL;
    }
    free(localVarBodyParameters);
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

prompt_version_diff_t*
PromptsAPI_promptsDiffVersions(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, char *from, char *to, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
{
    list_t    *localVarQueryParameters = list_createList();
    list_t    *localVarHeaderParameters = list_createList();
    list_t    *localVarFormParameters = NULL;
    list_t *localVarHeaderType = list_createList();
    list_t *localVarContentType = NULL;
    char      *localVarBodyParameters = NULL;
    size_t     localVarBodyLength = 0;

    // clear the error code from the previous api call
    apiClient->response_code = 0;

    // create the path
    char *localVarPath = strdup("/v1/prompts/{tenantId}/{projectId}/{promptId}/diff");

    if(!tenantId)
        goto end;
    if(!projectId)
        goto end;
    if(!promptId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_projectId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ projectId }") - 1;
    if(projectId == NULL) {
        goto end;
    }
    char* localVarToReplace_projectId = malloc(sizeOfPathParams_projectId);
    sprintf(localVarToReplace_projectId, "{%s}", "projectId");

    localVarPath = strReplace(localVarPath, localVarToReplace_projectId, projectId);

    // Path Params
    long sizeOfPathParams_promptId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ promptId }") - 1;
    if(promptId == NULL) {
        goto end;
    }
    char* localVarToReplace_promptId = malloc(sizeOfPathParams_promptId);
    sprintf(localVarToReplace_promptId, "{%s}", "promptId");

    localVarPath = strReplace(localVarPath, localVarToReplace_promptId, promptId);



    // header parameters
    char *keyHeader_authorization = NULL;
    char * valueHeader_authorization = 0;
    keyValuePair_t *keyPairHeader_authorization = 0;
    if (authorization) {
        keyHeader_authorization = strdup("authorization");
        valueHeader_authorization = strdup((authorization));
        keyPairHeader_authorization = keyValuePair_create(keyHeader_authorization, valueHeader_authorization);
        list_addElement(localVarHeaderParameters,keyPairHeader_authorization);
    }


    // header parameters
    char *keyHeader_x_palette_api_key = NULL;
    char * valueHeader_x_palette_api_key = 0;
    keyValuePair_t *keyPairHeader_x_palette_api_key = 0;
    if (x_palette_api_key) {
        keyHeader_x_palette_api_key = strdup("x-palette-api-key");
        valueHeader_x_palette_api_key = strdup((x_palette_api_key));
        keyPairHeader_x_palette_api_key = keyValuePair_create(keyHeader_x_palette_api_key, valueHeader_x_palette_api_key);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_api_key);
    }


    // header parameters
    char *keyHeader_x_palette_project_id = NULL;
    char * valueHeader_x_palette_project_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_project_id = 0;
    if (x_palette_project_id) {
        keyHeader_x_palette_project_id = strdup("x-palette-project-id");
        valueHeader_x_palette_project_id = strdup((x_palette_project_id));
        keyPairHeader_x_palette_project_id = keyValuePair_create(keyHeader_x_palette_project_id, valueHeader_x_palette_project_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_project_id);
    }


    // header parameters
    char *keyHeader_x_palette_environment_id = NULL;
    char * valueHeader_x_palette_environment_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_environment_id = 0;
    if (x_palette_environment_id) {
        keyHeader_x_palette_environment_id = strdup("x-palette-environment-id");
        valueHeader_x_palette_environment_id = strdup((x_palette_environment_id));
        keyPairHeader_x_palette_environment_id = keyValuePair_create(keyHeader_x_palette_environment_id, valueHeader_x_palette_environment_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_environment_id);
    }


    // query parameters
    char *keyQuery_from = NULL;
    char * valueQuery_from = NULL;
    keyValuePair_t *keyPairQuery_from = 0;
    if (from)
    {
        keyQuery_from = strdup("from");
        valueQuery_from = strdup((from));
        keyPairQuery_from = keyValuePair_create(keyQuery_from, valueQuery_from);
        list_addElement(localVarQueryParameters,keyPairQuery_from);
    }

    // query parameters
    char *keyQuery_to = NULL;
    char * valueQuery_to = NULL;
    keyValuePair_t *keyPairQuery_to = 0;
    if (to)
    {
        keyQuery_to = strdup("to");
        valueQuery_to = strdup((to));
        keyPairQuery_to = keyValuePair_create(keyQuery_to, valueQuery_to);
        list_addElement(localVarQueryParameters,keyPairQuery_to);
    }
    list_addElement(localVarHeaderType,"application/json"); //produces
    apiClient_invoke(apiClient,
                    localVarPath,
                    localVarQueryParameters,
                    localVarHeaderParameters,
                    localVarFormParameters,
                    localVarHeaderType,
                    localVarContentType,
                    localVarBodyParameters,
                    localVarBodyLength,
                    "GET");

    // uncomment below to debug the error response
    //if (apiClient->response_code == 200) {
    //    printf("%s\n","Line diff between two prompt versions");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 404) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 0) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    //nonprimitive not container
    prompt_version_diff_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *PromptsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = prompt_version_diff_parseFromJSON(PromptsAPIlocalVarJSON);
        cJSON_Delete(PromptsAPIlocalVarJSON);
        if(elementToReturn == NULL) {
            // return 0;
        }
    }

    //return type
    if (apiClient->dataReceived) {
        free(apiClient->dataReceived);
        apiClient->dataReceived = NULL;
        apiClient->dataReceivedLen = 0;
    }
    list_freeList(localVarQueryParameters);
    list_freeList(localVarHeaderParameters);

    list_freeList(localVarHeaderType);

    free(localVarPath);
    free(localVarToReplace_tenantId);
    free(localVarToReplace_projectId);
    free(localVarToReplace_promptId);
    if (keyHeader_authorization) {
        free(keyHeader_authorization);
        keyHeader_authorization = NULL;
    }
    if (valueHeader_authorization) {
        free(valueHeader_authorization);
        valueHeader_authorization = NULL;
    }
    free(keyPairHeader_authorization);
    if (keyHeader_x_palette_api_key) {
        free(keyHeader_x_palette_api_key);
        keyHeader_x_palette_api_key = NULL;
    }
    if (valueHeader_x_palette_api_key) {
        free(valueHeader_x_palette_api_key);
        valueHeader_x_palette_api_key = NULL;
    }
    free(keyPairHeader_x_palette_api_key);
    if (keyHeader_x_palette_project_id) {
        free(keyHeader_x_palette_project_id);
        keyHeader_x_palette_project_id = NULL;
    }
    if (valueHeader_x_palette_project_id) {
        free(valueHeader_x_palette_project_id);
        valueHeader_x_palette_project_id = NULL;
    }
    free(keyPairHeader_x_palette_project_id);
    if (keyHeader_x_palette_environment_id) {
        free(keyHeader_x_palette_environment_id);
        keyHeader_x_palette_environment_id = NULL;
    }
    if (valueHeader_x_palette_environment_id) {
        free(valueHeader_x_palette_environment_id);
        valueHeader_x_palette_environment_id = NULL;
    }
    free(keyPairHeader_x_palette_environment_id);
    if(keyQuery_from){
        free(keyQuery_from);
        keyQuery_from = NULL;
    }
    if(valueQuery_from){
        free(valueQuery_from);
        valueQuery_from = NULL;
    }
    if(keyPairQuery_from){
        keyValuePair_free(keyPairQuery_from);
        keyPairQuery_from = NULL;
    }
    if(keyQuery_to){
        free(keyQuery_to);
        keyQuery_to = NULL;
    }
    if(valueQuery_to){
        free(valueQuery_to);
        valueQuery_to = NULL;
    }
    if(keyPairQuery_to){
        keyValuePair_free(keyPairQuery_to);
        keyPairQuery_to = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

prompt_t*
PromptsAPI_promptsGet(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
{
    list_t    *localVarQueryParameters = NULL;
    list_t    *localVarHeaderParameters = list_createList();
    list_t    *localVarFormParameters = NULL;
    list_t *localVarHeaderType = list_createList();
    list_t *localVarContentType = NULL;
    char      *localVarBodyParameters = NULL;
    size_t     localVarBodyLength = 0;

    // clear the error code from the previous api call
    apiClient->response_code = 0;

    // create the path
    char *localVarPath = strdup("/v1/prompts/{tenantId}/{projectId}/{promptId}");

    if(!tenantId)
        goto end;
    if(!projectId)
        goto end;
    if(!promptId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_projectId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ projectId }") - 1;
    if(projectId == NULL) {
        goto end;
    }
    char* localVarToReplace_projectId = malloc(sizeOfPathParams_projectId);
    sprintf(localVarToReplace_projectId, "{%s}", "projectId");

    localVarPath = strReplace(localVarPath, localVarToReplace_projectId, projectId);

    // Path Params
    long sizeOfPathParams_promptId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ promptId }") - 1;
    if(promptId == NULL) {
        goto end;
    }
    char* localVarToReplace_promptId = malloc(sizeOfPathParams_promptId);
    sprintf(localVarToReplace_promptId, "{%s}", "promptId");

    localVarPath = strReplace(localVarPath, localVarToReplace_promptId, promptId);



    // header parameters
    char *keyHeader_authorization = NULL;
    char * valueHeader_authorization = 0;
    keyValuePair_t *keyPairHeader_authorization = 0;
    if (authorization) {
        keyHeader_authorization = strdup("authorization");
        valueHeader_authorization = strdup((authorization));
        keyPairHeader_authorization = keyValuePair_create(keyHeader_authorization, valueHeader_authorization);
        list_addElement(localVarHeaderParameters,keyPairHeader_authorization);
    }


    // header parameters
    char *keyHeader_x_palette_api_key = NULL;
    char * valueHeader_x_palette_api_key = 0;
    keyValuePair_t *keyPairHeader_x_palette_api_key = 0;
    if (x_palette_api_key) {
        keyHeader_x_palette_api_key = strdup("x-palette-api-key");
        valueHeader_x_palette_api_key = strdup((x_palette_api_key));
        keyPairHeader_x_palette_api_key = keyValuePair_create(keyHeader_x_palette_api_key, valueHeader_x_palette_api_key);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_api_key);
    }


    // header parameters
    char *keyHeader_x_palette_project_id = NULL;
    char * valueHeader_x_palette_project_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_project_id = 0;
    if (x_palette_project_id) {
        keyHeader_x_palette_project_id = strdup("x-palette-project-id");
        valueHeader_x_palette_project_id = strdup((x_palette_project_id));
        keyPairHeader_x_palette_project_id = keyValuePair_create(keyHeader_x_palette_project_id, valueHeader_x_palette_project_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_project_id);
    }


    // header parameters
    char *keyHeader_x_palette_environment_id = NULL;
    char * valueHeader_x_palette_environment_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_environment_id = 0;
    if (x_palette_environment_id) {
        keyHeader_x_palette_environment_id = strdup("x-palette-environment-id");
        valueHeader_x_palette_environment_id = strdup((x_palette_environment_id));
        keyPairHeader_x_palette_environment_id = keyValuePair_create(keyHeader_x_palette_environment_id, valueHeader_x_palette_environment_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_environment_id);
    }

    list_addElement(localVarHeaderType,"application/json"); //produces
    apiClient_invoke(apiClient,
                    localVarPath,
                    localVarQueryParameters,
                    localVarHeaderParameters,
                    localVarFormParameters,
                    localVarHeaderType,
                    localVarContentType,
                    localVarBodyParameters,
                    localVarBodyLength,
                    "GET");

    // uncomment below to debug the error response
    //if (apiClient->response_code == 200) {
    //    printf("%s\n","Get a prompt&#39;s metadata");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 404) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 0) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    //nonprimitive not container
    prompt_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *PromptsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = prompt_parseFromJSON(PromptsAPIlocalVarJSON);
        cJSON_Delete(PromptsAPIlocalVarJSON);
        if(elementToReturn == NULL) {
            // return 0;
        }
    }

    //return type
    if (apiClient->dataReceived) {
        free(apiClient->dataReceived);
        apiClient->dataReceived = NULL;
        apiClient->dataReceivedLen = 0;
    }

    list_freeList(localVarHeaderParameters);

    list_freeList(localVarHeaderType);

    free(localVarPath);
    free(localVarToReplace_tenantId);
    free(localVarToReplace_projectId);
    free(localVarToReplace_promptId);
    if (keyHeader_authorization) {
        free(keyHeader_authorization);
        keyHeader_authorization = NULL;
    }
    if (valueHeader_authorization) {
        free(valueHeader_authorization);
        valueHeader_authorization = NULL;
    }
    free(keyPairHeader_authorization);
    if (keyHeader_x_palette_api_key) {
        free(keyHeader_x_palette_api_key);
        keyHeader_x_palette_api_key = NULL;
    }
    if (valueHeader_x_palette_api_key) {
        free(valueHeader_x_palette_api_key);
        valueHeader_x_palette_api_key = NULL;
    }
    free(keyPairHeader_x_palette_api_key);
    if (keyHeader_x_palette_project_id) {
        free(keyHeader_x_palette_project_id);
        keyHeader_x_palette_project_id = NULL;
    }
    if (valueHeader_x_palette_project_id) {
        free(valueHeader_x_palette_project_id);
        valueHeader_x_palette_project_id = NULL;
    }
    free(keyPairHeader_x_palette_project_id);
    if (keyHeader_x_palette_environment_id) {
        free(keyHeader_x_palette_environment_id);
        keyHeader_x_palette_environment_id = NULL;
    }
    if (valueHeader_x_palette_environment_id) {
        free(valueHeader_x_palette_environment_id);
        valueHeader_x_palette_environment_id = NULL;
    }
    free(keyPairHeader_x_palette_environment_id);
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

prompt_list_response_t*
PromptsAPI_promptsList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
{
    list_t    *localVarQueryParameters = list_createList();
    list_t    *localVarHeaderParameters = list_createList();
    list_t    *localVarFormParameters = NULL;
    list_t *localVarHeaderType = list_createList();
    list_t *localVarContentType = NULL;
    char      *localVarBodyParameters = NULL;
    size_t     localVarBodyLength = 0;

    // clear the error code from the previous api call
    apiClient->response_code = 0;

    // create the path
    char *localVarPath = strdup("/v1/prompts/{tenantId}/{projectId}");

    if(!tenantId)
        goto end;
    if(!projectId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(projectId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_projectId = strlen(tenantId)+3 + strlen(projectId)+3 + sizeof("{ projectId }") - 1;
    if(projectId == NULL) {
        goto end;
    }
    char* localVarToReplace_projectId = malloc(sizeOfPathParams_projectId);
    sprintf(localVarToReplace_projectId, "{%s}", "projectId");

    localVarPath = strReplace(localVarPath, localVarToReplace_projectId, projectId);



    // header parameters
    char *keyHeader_authorization = NULL;
    char * valueHeader_authorization = 0;
    keyValuePair_t *keyPairHeader_authorization = 0;
    if (authorization) {
        keyHeader_authorization = strdup("authorization");
        valueHeader_authorization = strdup((authorization));
        keyPairHeader_authorization = keyValuePair_create(keyHeader_authorization, valueHeader_authorization);
        list_addElement(localVarHeaderParameters,keyPairHeader_authorization);
    }


    // header parameters
    char *keyHeader_x_palette_api_key = NULL;
    char * valueHeader_x_palette_api_key = 0;
    keyValuePair_t *keyPairHeader_x_palette_api_key = 0;
    if (x_palette_api_key) {
        keyHeader_x_palette_api_key = strdup("x-palette-api-key");
        valueHeader_x_palette_api_key = strdup((x_palette_api_key));
        keyPairHeader_x_palette_api_key = keyValuePair_create(keyHeader_x_palette_api_key, valueHeader_x_palette_api_key);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_api_key);
    }


    // header parameters
    char *keyHeader_x_palette_project_id = NULL;
    char * valueHeader_x_palette_project_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_project_id = 0;
    if (x_palette_project_id) {
        keyHeader_x_palette_project_id = strdup("x-palette-project-id");
        valueHeader_x_palette_project_id = strdup((x_palette_project_id));
        keyPairHeader_x_palette_project_id = keyValuePair_create(keyHeader_x_palette_project_id, valueHeader_x_palette_project_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_project_id);
    }


    // header parameters
    char *keyHeader_x_palette_environment_id = NULL;
    char * valueHeader_x_palette_environment_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_environment_id = 0;
    if (x_palette_environment_id) {
        keyHeader_x_palette_environment_id = strdup("x-palette-environment-id");
        valueHeader_x_palette_environment_id = strdup((x_palette_environment_id));
        keyPairHeader_x_palette_environment_id = keyValuePair_create(keyHeader_x_palette_environment_id, valueHeader_x_palette_environment_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_environment_id);
    }


    // query parameters
    char *keyQuery_pageSize = NULL;
    char * valueQuery_pageSize = NULL;
    keyValuePair_t *keyPairQuery_pageSize = 0;
    if (pageSize)
    {
        keyQuery_pageSize = strdup("pageSize");
        valueQuery_pageSize = calloc(1,MAX_NUMBER_LENGTH);
        snprintf(valueQuery_pageSize, MAX_NUMBER_LENGTH, "%d", *pageSize);
        keyPairQuery_pageSize = keyValuePair_create(keyQuery_pageSize, valueQuery_pageSize);
        list_addElement(localVarQueryParameters,keyPairQuery_pageSize);
    }

    // query parameters
    char *keyQuery_pageToken = NULL;
    char * valueQuery_pageToken = NULL;
    keyValuePair_t *keyPairQuery_pageToken = 0;
    if (pageToken)
    {
        keyQuery_pageToken = strdup("pageToken");
        valueQuery_pageToken = strdup((pageToken));
        keyPairQuery_pageToken = keyValuePair_create(keyQuery_pageToken, valueQuery_pageToken);
        list_addElement(localVarQueryParameters,keyPairQuery_pageToken);
    }
    list_addElement(localVarHeaderType,"application/json"); //produces
    apiClient_invoke(apiClient,
                    localVarPath,
                    localVarQueryParameters,
                    localVarHeaderParameters,
                    localVarFormParameters,
                    localVarHeaderType,
                    localVarContentType,
                    localVarBodyParameters,
                    localVarBodyLength,
                    "GET");

    // uncomment below to debug the error response
    //if (apiClient->response_code == 200) {
    //    printf("%s\n","List prompts in a project");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 0) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    //nonprimitive not container
    prompt_list_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *PromptsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = prompt_list_response_parseFromJSON(PromptsAPIlocalVarJSON);
        cJSON_Delete(PromptsAPIlocalVarJSON);
        if(elementToReturn == NULL) {
            // return 0;
        }
    }

    //return type
    if (apiClient->dataReceived) {
        free(apiClient->dataReceived);
        apiClient->dataReceived = NULL;
        apiClient->dataReceivedLen = 0;
    }
    list_freeList(localVarQueryParameters);
    list_freeList(localVarHeaderParameters);

    list_freeList(localVarHeaderType);

    free(localVarPath);
    free(localVarToReplace_tenantId);
    free(localVarToReplace_projectId);
    if (keyHeader_authorization) {
        free(keyHeader_authorization);
        keyHeader_authorization = NULL;
    }
    if (valueHeader_authorization) {
        free(valueHeader_authorization);
        valueHeader_authorization = NULL;
    }
    free(keyPairHeader_authorization);
    if (keyHeader_x_palette_api_key) {
        free(keyHeader_x_palette_api_key);
        keyHeader_x_palette_api_key = NULL;
    }
    if (valueHeader_x_palette_api_key) {
        free(valueHeader_x_palette_api_key);
        valueHeader_x_palette_api_key = NULL;
    }
    free(keyPairHeader_x_palette_api_key);
    if (keyHeader_x_palette_project_id) {
        free(keyHeader_x_palette_project_id);
        keyHeader_x_palette_project_id = NULL;
    }
    if (valueHeader_x_palette_project_id) {
        free(valueHeader_x_palette_project_id);
        valueHeader_x_palette_project_id = NULL;
    }
    free(keyPairHeader_x_palette_project_id);
    if (keyHeader_x_palette_environment_id) {
        free(keyHeader_x_palette_environment_id);
        keyHeader_x_palette_environment_id = NULL;
    }
    if (valueHeader_x_palette_environment_id) {
        free(valueHeader_x_palette_environment_id);
        valueHeader_x_palette_environment_id = NULL;
    }
    free(keyPairHeader_x_palette_environment_id);
    if(keyQuery_pageSize){
        free(keyQuery_pageSize);
        keyQuery_pageSize = NULL;
    }
    if(valueQuery_pageSize){
        free(valueQuery_pageSize);
        valueQuery_pageSize = NULL;
    }
    if(keyPairQuery_pageSize){
        keyValuePair_free(keyPairQuery_pageSize);
        keyPairQuery_pageSize = NULL;
    }
    if(keyQuery_pageToken){
        free(keyQuery_pageToken);
        keyQuery_pageToken = NULL;
    }
    if(valueQuery_pageToken){
        free(valueQuery_pageToken);
        valueQuery_pageToken = NULL;
    }
    if(keyPairQuery_pageToken){
        keyValuePair_free(keyPairQuery_pageToken);
        keyPairQuery_pageToken = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

prompt_version_list_response_t*
PromptsAPI_promptsListVersions(apiClient_t *apiClient, char *tenantId, char *projectId, char *promptId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
{
    list_t    *localVarQueryParameters = list_createList();
    list_t    *localVarHeaderParameters = list_createList();
    list_t    *localVarFormParameters = NULL;
    list_t *localVarHeaderType = list_createList();
    list_t *localVarContentType = NULL;
    char      *localVarBodyParameters = NULL;
    size_t     localVarBodyLength = 0;

    // clear the error code from the previous api call
    apiClient->response_code = 0;

    // create the path
    char *localVarPath = strdup("/v1/prompts/{tenantId}/{projectId}/{promptId}/versions");

    if(!tenantId)
        goto end;
    if(!projectId)
        goto end;
    if(!promptId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_projectId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ projectId }") - 1;
    if(projectId == NULL) {
        goto end;
    }
    char* localVarToReplace_projectId = malloc(sizeOfPathParams_projectId);
    sprintf(localVarToReplace_projectId, "{%s}", "projectId");

    localVarPath = strReplace(localVarPath, localVarToReplace_projectId, projectId);

    // Path Params
    long sizeOfPathParams_promptId = strlen(tenantId)+3 + strlen(projectId)+3 + strlen(promptId)+3 + sizeof("{ promptId }") - 1;
    if(promptId == NULL) {
        goto end;
    }
    char* localVarToReplace_promptId = malloc(sizeOfPathParams_promptId);
    sprintf(localVarToReplace_promptId, "{%s}", "promptId");

    localVarPath = strReplace(localVarPath, localVarToReplace_promptId, promptId);



    // header parameters
    char *keyHeader_authorization = NULL;
    char * valueHeader_authorization = 0;
    keyValuePair_t *keyPairHeader_authorization = 0;
    if (authorization) {
        keyHeader_authorization = strdup("authorization");
        valueHeader_authorization = strdup((authorization));
        keyPairHeader_authorization = keyValuePair_create(keyHeader_authorization, valueHeader_authorization);
        list_addElement(localVarHeaderParameters,keyPairHeader_authorization);
    }


    // header parameters
    char *keyHeader_x_palette_api_key = NULL;
    char * valueHeader_x_palette_api_key = 0;
    keyValuePair_t *keyPairHeader_x_palette_api_key = 0;
    if (x_palette_api_key) {
        keyHeader_x_palette_api_key = strdup("x-palette-api-key");
        valueHeader_x_palette_api_key = strdup((x_palette_api_key));
        keyPairHeader_x_palette_api_key = keyValuePair_create(keyHeader_x_palette_api_key, valueHeader_x_palette_api_key);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_api_key);
    }


    // header parameters
    char *keyHeader_x_palette_project_id = NULL;
    char * valueHeader_x_palette_project_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_project_id = 0;
    if (x_palette_project_id) {
        keyHeader_x_palette_project_id = strdup("x-palette-project-id");
        valueHeader_x_palette_project_id = strdup((x_palette_project_id));
        keyPairHeader_x_palette_project_id = keyValuePair_create(keyHeader_x_palette_project_id, valueHeader_x_palette_project_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_project_id);
    }


    // header parameters
    char *keyHeader_x_palette_environment_id = NULL;
    char * valueHeader_x_palette_environment_id = 0;
    keyValuePair_t *keyPairHeader_x_palette_environment_id = 0;
    if (x_palette_environment_id) {
        keyHeader_x_palette_environment_id = strdup("x-palette-environment-id");
        valueHeader_x_palette_environment_id = strdup((x_palette_environment_id));
        keyPairHeader_x_palette_environment_id = keyValuePair_create(keyHeader_x_palette_environment_id, valueHeader_x_palette_environment_id);
        list_addElement(localVarHeaderParameters,keyPairHeader_x_palette_environment_id);
    }


    // query parameters
    char *keyQuery_pageSize = NULL;
    char * valueQuery_pageSize = NULL;
    keyValuePair_t *keyPairQuery_pageSize = 0;
    if (pageSize)
    {
        keyQuery_pageSize = strdup("pageSize");
        valueQuery_pageSize = calloc(1,MAX_NUMBER_LENGTH);
        snprintf(valueQuery_pageSize, MAX_NUMBER_LENGTH, "%d", *pageSize);
        keyPairQuery_pageSize = keyValuePair_create(keyQuery_pageSize, valueQuery_pageSize);
        list_addElement(localVarQueryParameters,keyPairQuery_pageSize);
    }

    // query parameters
    char *keyQuery_pageToken = NULL;
    char * valueQuery_pageToken = NULL;
    keyValuePair_t *keyPairQuery_pageToken = 0;
    if (pageToken)
    {
        keyQuery_pageToken = strdup("pageToken");
        valueQuery_pageToken = strdup((pageToken));
        keyPairQuery_pageToken = keyValuePair_create(keyQuery_pageToken, valueQuery_pageToken);
        list_addElement(localVarQueryParameters,keyPairQuery_pageToken);
    }
    list_addElement(localVarHeaderType,"application/json"); //produces
    apiClient_invoke(apiClient,
                    localVarPath,
                    localVarQueryParameters,
                    localVarHeaderParameters,
                    localVarFormParameters,
                    localVarHeaderType,
                    localVarContentType,
                    localVarBodyParameters,
                    localVarBodyLength,
                    "GET");

    // uncomment below to debug the error response
    //if (apiClient->response_code == 200) {
    //    printf("%s\n","List a prompt&#39;s versions oldest-first");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 404) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 0) {
    //    printf("%s\n","A google.rpc.Status error envelope.");
    //}
    //nonprimitive not container
    prompt_version_list_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *PromptsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = prompt_version_list_response_parseFromJSON(PromptsAPIlocalVarJSON);
        cJSON_Delete(PromptsAPIlocalVarJSON);
        if(elementToReturn == NULL) {
            // return 0;
        }
    }

    //return type
    if (apiClient->dataReceived) {
        free(apiClient->dataReceived);
        apiClient->dataReceived = NULL;
        apiClient->dataReceivedLen = 0;
    }
    list_freeList(localVarQueryParameters);
    list_freeList(localVarHeaderParameters);

    list_freeList(localVarHeaderType);

    free(localVarPath);
    free(localVarToReplace_tenantId);
    free(localVarToReplace_projectId);
    free(localVarToReplace_promptId);
    if (keyHeader_authorization) {
        free(keyHeader_authorization);
        keyHeader_authorization = NULL;
    }
    if (valueHeader_authorization) {
        free(valueHeader_authorization);
        valueHeader_authorization = NULL;
    }
    free(keyPairHeader_authorization);
    if (keyHeader_x_palette_api_key) {
        free(keyHeader_x_palette_api_key);
        keyHeader_x_palette_api_key = NULL;
    }
    if (valueHeader_x_palette_api_key) {
        free(valueHeader_x_palette_api_key);
        valueHeader_x_palette_api_key = NULL;
    }
    free(keyPairHeader_x_palette_api_key);
    if (keyHeader_x_palette_project_id) {
        free(keyHeader_x_palette_project_id);
        keyHeader_x_palette_project_id = NULL;
    }
    if (valueHeader_x_palette_project_id) {
        free(valueHeader_x_palette_project_id);
        valueHeader_x_palette_project_id = NULL;
    }
    free(keyPairHeader_x_palette_project_id);
    if (keyHeader_x_palette_environment_id) {
        free(keyHeader_x_palette_environment_id);
        keyHeader_x_palette_environment_id = NULL;
    }
    if (valueHeader_x_palette_environment_id) {
        free(valueHeader_x_palette_environment_id);
        valueHeader_x_palette_environment_id = NULL;
    }
    free(keyPairHeader_x_palette_environment_id);
    if(keyQuery_pageSize){
        free(keyQuery_pageSize);
        keyQuery_pageSize = NULL;
    }
    if(valueQuery_pageSize){
        free(valueQuery_pageSize);
        valueQuery_pageSize = NULL;
    }
    if(keyPairQuery_pageSize){
        keyValuePair_free(keyPairQuery_pageSize);
        keyPairQuery_pageSize = NULL;
    }
    if(keyQuery_pageToken){
        free(keyQuery_pageToken);
        keyQuery_pageToken = NULL;
    }
    if(valueQuery_pageToken){
        free(valueQuery_pageToken);
        valueQuery_pageToken = NULL;
    }
    if(keyPairQuery_pageToken){
        keyValuePair_free(keyPairQuery_pageToken);
        keyPairQuery_pageToken = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}
