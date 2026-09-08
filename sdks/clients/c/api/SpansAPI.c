#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include "SpansAPI.h"

#define MAX_NUMBER_LENGTH 16
#define MAX_BUFFER_LENGTH 4096


canonical_span_t*
SpansAPI_spansGet(apiClient_t *apiClient, char *tenantId, char *traceId, char *spanId, int *unmask, char *reason, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/spans/{tenantId}/{traceId}/{spanId}");

    if(!tenantId)
        goto end;
    if(!traceId)
        goto end;
    if(!spanId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(traceId)+3 + strlen(spanId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_traceId = strlen(tenantId)+3 + strlen(traceId)+3 + strlen(spanId)+3 + sizeof("{ traceId }") - 1;
    if(traceId == NULL) {
        goto end;
    }
    char* localVarToReplace_traceId = malloc(sizeOfPathParams_traceId);
    sprintf(localVarToReplace_traceId, "{%s}", "traceId");

    localVarPath = strReplace(localVarPath, localVarToReplace_traceId, traceId);

    // Path Params
    long sizeOfPathParams_spanId = strlen(tenantId)+3 + strlen(traceId)+3 + strlen(spanId)+3 + sizeof("{ spanId }") - 1;
    if(spanId == NULL) {
        goto end;
    }
    char* localVarToReplace_spanId = malloc(sizeOfPathParams_spanId);
    sprintf(localVarToReplace_spanId, "{%s}", "spanId");

    localVarPath = strReplace(localVarPath, localVarToReplace_spanId, spanId);



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
    char *keyQuery_unmask = NULL;
    char * valueQuery_unmask = NULL;
    keyValuePair_t *keyPairQuery_unmask = 0;
    if (unmask)
    {
        keyQuery_unmask = strdup("unmask");
        valueQuery_unmask = calloc(1,MAX_NUMBER_LENGTH);
        snprintf(valueQuery_unmask, MAX_NUMBER_LENGTH, "%d", *unmask);
        keyPairQuery_unmask = keyValuePair_create(keyQuery_unmask, valueQuery_unmask);
        list_addElement(localVarQueryParameters,keyPairQuery_unmask);
    }

    // query parameters
    char *keyQuery_reason = NULL;
    char * valueQuery_reason = NULL;
    keyValuePair_t *keyPairQuery_reason = 0;
    if (reason)
    {
        keyQuery_reason = strdup("reason");
        valueQuery_reason = strdup((reason));
        keyPairQuery_reason = keyValuePair_create(keyQuery_reason, valueQuery_reason);
        list_addElement(localVarQueryParameters,keyPairQuery_reason);
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
    //    printf("%s\n","Get a canonical span");
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
    canonical_span_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *SpansAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = canonical_span_parseFromJSON(SpansAPIlocalVarJSON);
        cJSON_Delete(SpansAPIlocalVarJSON);
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
    free(localVarToReplace_traceId);
    free(localVarToReplace_spanId);
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
    if(keyQuery_unmask){
        free(keyQuery_unmask);
        keyQuery_unmask = NULL;
    }
    if(valueQuery_unmask){
        free(valueQuery_unmask);
        valueQuery_unmask = NULL;
    }
    if(keyPairQuery_unmask){
        keyValuePair_free(keyPairQuery_unmask);
        keyPairQuery_unmask = NULL;
    }
    if(keyQuery_reason){
        free(keyQuery_reason);
        keyQuery_reason = NULL;
    }
    if(valueQuery_reason){
        free(valueQuery_reason);
        valueQuery_reason = NULL;
    }
    if(keyPairQuery_reason){
        keyValuePair_free(keyPairQuery_reason);
        keyPairQuery_reason = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

span_io_response_t*
SpansAPI_spansGetIo(apiClient_t *apiClient, char *tenantId, char *traceId, char *spanId, int *unmask, char *reason, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/spans/{tenantId}/{traceId}/{spanId}/io");

    if(!tenantId)
        goto end;
    if(!traceId)
        goto end;
    if(!spanId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(traceId)+3 + strlen(spanId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_traceId = strlen(tenantId)+3 + strlen(traceId)+3 + strlen(spanId)+3 + sizeof("{ traceId }") - 1;
    if(traceId == NULL) {
        goto end;
    }
    char* localVarToReplace_traceId = malloc(sizeOfPathParams_traceId);
    sprintf(localVarToReplace_traceId, "{%s}", "traceId");

    localVarPath = strReplace(localVarPath, localVarToReplace_traceId, traceId);

    // Path Params
    long sizeOfPathParams_spanId = strlen(tenantId)+3 + strlen(traceId)+3 + strlen(spanId)+3 + sizeof("{ spanId }") - 1;
    if(spanId == NULL) {
        goto end;
    }
    char* localVarToReplace_spanId = malloc(sizeOfPathParams_spanId);
    sprintf(localVarToReplace_spanId, "{%s}", "spanId");

    localVarPath = strReplace(localVarPath, localVarToReplace_spanId, spanId);



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
    char *keyQuery_unmask = NULL;
    char * valueQuery_unmask = NULL;
    keyValuePair_t *keyPairQuery_unmask = 0;
    if (unmask)
    {
        keyQuery_unmask = strdup("unmask");
        valueQuery_unmask = calloc(1,MAX_NUMBER_LENGTH);
        snprintf(valueQuery_unmask, MAX_NUMBER_LENGTH, "%d", *unmask);
        keyPairQuery_unmask = keyValuePair_create(keyQuery_unmask, valueQuery_unmask);
        list_addElement(localVarQueryParameters,keyPairQuery_unmask);
    }

    // query parameters
    char *keyQuery_reason = NULL;
    char * valueQuery_reason = NULL;
    keyValuePair_t *keyPairQuery_reason = 0;
    if (reason)
    {
        keyQuery_reason = strdup("reason");
        valueQuery_reason = strdup((reason));
        keyPairQuery_reason = keyValuePair_create(keyQuery_reason, valueQuery_reason);
        list_addElement(localVarQueryParameters,keyPairQuery_reason);
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
    //    printf("%s\n","Get span input/output metadata");
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
    span_io_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *SpansAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = span_io_response_parseFromJSON(SpansAPIlocalVarJSON);
        cJSON_Delete(SpansAPIlocalVarJSON);
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
    free(localVarToReplace_traceId);
    free(localVarToReplace_spanId);
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
    if(keyQuery_unmask){
        free(keyQuery_unmask);
        keyQuery_unmask = NULL;
    }
    if(valueQuery_unmask){
        free(valueQuery_unmask);
        valueQuery_unmask = NULL;
    }
    if(keyPairQuery_unmask){
        keyValuePair_free(keyPairQuery_unmask);
        keyPairQuery_unmask = NULL;
    }
    if(keyQuery_reason){
        free(keyQuery_reason);
        keyQuery_reason = NULL;
    }
    if(valueQuery_reason){
        free(valueQuery_reason);
        valueQuery_reason = NULL;
    }
    if(keyPairQuery_reason){
        keyValuePair_free(keyPairQuery_reason);
        keyPairQuery_reason = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

