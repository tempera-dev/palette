#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include "TracesAPI.h"

#define MAX_NUMBER_LENGTH 16
#define MAX_BUFFER_LENGTH 4096


trace_view_t*
TracesAPI_tracesGet(apiClient_t *apiClient, char *tenantId, char *traceId, int *unmask, char *reason, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/traces/{tenantId}/{traceId}");

    if(!tenantId)
        goto end;
    if(!traceId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + strlen(traceId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);

    // Path Params
    long sizeOfPathParams_traceId = strlen(tenantId)+3 + strlen(traceId)+3 + sizeof("{ traceId }") - 1;
    if(traceId == NULL) {
        goto end;
    }
    char* localVarToReplace_traceId = malloc(sizeOfPathParams_traceId);
    sprintf(localVarToReplace_traceId, "{%s}", "traceId");

    localVarPath = strReplace(localVarPath, localVarToReplace_traceId, traceId);



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
    //    printf("%s\n","Get a canonical trace");
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
    trace_view_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *TracesAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = trace_view_parseFromJSON(TracesAPIlocalVarJSON);
        cJSON_Delete(TracesAPIlocalVarJSON);
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

trace_list_response_t*
TracesAPI_tracesList(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, char *traceId, char *kind, char *status, char *startedAfter, char *startedBefore, char *model, char *release, long minCostMicros, long maxCostMicros, long minLatencyMs, long maxLatencyMs, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/traces/{tenantId}");

    if(!tenantId)
        goto end;


    // Path Params
    long sizeOfPathParams_tenantId = strlen(tenantId)+3 + sizeof("{ tenantId }") - 1;
    if(tenantId == NULL) {
        goto end;
    }
    char* localVarToReplace_tenantId = malloc(sizeOfPathParams_tenantId);
    sprintf(localVarToReplace_tenantId, "{%s}", "tenantId");

    localVarPath = strReplace(localVarPath, localVarToReplace_tenantId, tenantId);



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
    char *keyQuery_projectId = NULL;
    char * valueQuery_projectId = NULL;
    keyValuePair_t *keyPairQuery_projectId = 0;
    if (projectId)
    {
        keyQuery_projectId = strdup("projectId");
        valueQuery_projectId = strdup((projectId));
        keyPairQuery_projectId = keyValuePair_create(keyQuery_projectId, valueQuery_projectId);
        list_addElement(localVarQueryParameters,keyPairQuery_projectId);
    }

    // query parameters
    char *keyQuery_environmentId = NULL;
    char * valueQuery_environmentId = NULL;
    keyValuePair_t *keyPairQuery_environmentId = 0;
    if (environmentId)
    {
        keyQuery_environmentId = strdup("environmentId");
        valueQuery_environmentId = strdup((environmentId));
        keyPairQuery_environmentId = keyValuePair_create(keyQuery_environmentId, valueQuery_environmentId);
        list_addElement(localVarQueryParameters,keyPairQuery_environmentId);
    }

    // query parameters
    char *keyQuery_traceId = NULL;
    char * valueQuery_traceId = NULL;
    keyValuePair_t *keyPairQuery_traceId = 0;
    if (traceId)
    {
        keyQuery_traceId = strdup("traceId");
        valueQuery_traceId = strdup((traceId));
        keyPairQuery_traceId = keyValuePair_create(keyQuery_traceId, valueQuery_traceId);
        list_addElement(localVarQueryParameters,keyPairQuery_traceId);
    }

    // query parameters
    char *keyQuery_kind = NULL;
    char * valueQuery_kind = NULL;
    keyValuePair_t *keyPairQuery_kind = 0;
    if (kind)
    {
        keyQuery_kind = strdup("kind");
        valueQuery_kind = strdup((kind));
        keyPairQuery_kind = keyValuePair_create(keyQuery_kind, valueQuery_kind);
        list_addElement(localVarQueryParameters,keyPairQuery_kind);
    }

    // query parameters
    char *keyQuery_status = NULL;
    char * valueQuery_status = NULL;
    keyValuePair_t *keyPairQuery_status = 0;
    if (status)
    {
        keyQuery_status = strdup("status");
        valueQuery_status = strdup((status));
        keyPairQuery_status = keyValuePair_create(keyQuery_status, valueQuery_status);
        list_addElement(localVarQueryParameters,keyPairQuery_status);
    }

    // query parameters
    char *keyQuery_startedAfter = NULL;
    char * valueQuery_startedAfter = NULL;
    keyValuePair_t *keyPairQuery_startedAfter = 0;
    if (startedAfter)
    {
        keyQuery_startedAfter = strdup("startedAfter");
        valueQuery_startedAfter = strdup((startedAfter));
        keyPairQuery_startedAfter = keyValuePair_create(keyQuery_startedAfter, valueQuery_startedAfter);
        list_addElement(localVarQueryParameters,keyPairQuery_startedAfter);
    }

    // query parameters
    char *keyQuery_startedBefore = NULL;
    char * valueQuery_startedBefore = NULL;
    keyValuePair_t *keyPairQuery_startedBefore = 0;
    if (startedBefore)
    {
        keyQuery_startedBefore = strdup("startedBefore");
        valueQuery_startedBefore = strdup((startedBefore));
        keyPairQuery_startedBefore = keyValuePair_create(keyQuery_startedBefore, valueQuery_startedBefore);
        list_addElement(localVarQueryParameters,keyPairQuery_startedBefore);
    }

    // query parameters
    char *keyQuery_model = NULL;
    char * valueQuery_model = NULL;
    keyValuePair_t *keyPairQuery_model = 0;
    if (model)
    {
        keyQuery_model = strdup("model");
        valueQuery_model = strdup((model));
        keyPairQuery_model = keyValuePair_create(keyQuery_model, valueQuery_model);
        list_addElement(localVarQueryParameters,keyPairQuery_model);
    }

    // query parameters
    char *keyQuery_release = NULL;
    char * valueQuery_release = NULL;
    keyValuePair_t *keyPairQuery_release = 0;
    if (release)
    {
        keyQuery_release = strdup("release");
        valueQuery_release = strdup((release));
        keyPairQuery_release = keyValuePair_create(keyQuery_release, valueQuery_release);
        list_addElement(localVarQueryParameters,keyPairQuery_release);
    }

    // query parameters
    char *keyQuery_minCostMicros = NULL;
    long valueQuery_minCostMicros ;
    keyValuePair_t *keyPairQuery_minCostMicros = 0;
    if (minCostMicros)
    {
        keyQuery_minCostMicros = strdup("minCostMicros");
        valueQuery_minCostMicros = (minCostMicros);
        keyPairQuery_minCostMicros = keyValuePair_create(keyQuery_minCostMicros, &valueQuery_minCostMicros);
        list_addElement(localVarQueryParameters,keyPairQuery_minCostMicros);
    }

    // query parameters
    char *keyQuery_maxCostMicros = NULL;
    long valueQuery_maxCostMicros ;
    keyValuePair_t *keyPairQuery_maxCostMicros = 0;
    if (maxCostMicros)
    {
        keyQuery_maxCostMicros = strdup("maxCostMicros");
        valueQuery_maxCostMicros = (maxCostMicros);
        keyPairQuery_maxCostMicros = keyValuePair_create(keyQuery_maxCostMicros, &valueQuery_maxCostMicros);
        list_addElement(localVarQueryParameters,keyPairQuery_maxCostMicros);
    }

    // query parameters
    char *keyQuery_minLatencyMs = NULL;
    long valueQuery_minLatencyMs ;
    keyValuePair_t *keyPairQuery_minLatencyMs = 0;
    if (minLatencyMs)
    {
        keyQuery_minLatencyMs = strdup("minLatencyMs");
        valueQuery_minLatencyMs = (minLatencyMs);
        keyPairQuery_minLatencyMs = keyValuePair_create(keyQuery_minLatencyMs, &valueQuery_minLatencyMs);
        list_addElement(localVarQueryParameters,keyPairQuery_minLatencyMs);
    }

    // query parameters
    char *keyQuery_maxLatencyMs = NULL;
    long valueQuery_maxLatencyMs ;
    keyValuePair_t *keyPairQuery_maxLatencyMs = 0;
    if (maxLatencyMs)
    {
        keyQuery_maxLatencyMs = strdup("maxLatencyMs");
        valueQuery_maxLatencyMs = (maxLatencyMs);
        keyPairQuery_maxLatencyMs = keyValuePair_create(keyQuery_maxLatencyMs, &valueQuery_maxLatencyMs);
        list_addElement(localVarQueryParameters,keyPairQuery_maxLatencyMs);
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
    //    printf("%s\n","List trace run summaries");
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
    trace_list_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *TracesAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = trace_list_response_parseFromJSON(TracesAPIlocalVarJSON);
        cJSON_Delete(TracesAPIlocalVarJSON);
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
    if(keyQuery_projectId){
        free(keyQuery_projectId);
        keyQuery_projectId = NULL;
    }
    if(valueQuery_projectId){
        free(valueQuery_projectId);
        valueQuery_projectId = NULL;
    }
    if(keyPairQuery_projectId){
        keyValuePair_free(keyPairQuery_projectId);
        keyPairQuery_projectId = NULL;
    }
    if(keyQuery_environmentId){
        free(keyQuery_environmentId);
        keyQuery_environmentId = NULL;
    }
    if(valueQuery_environmentId){
        free(valueQuery_environmentId);
        valueQuery_environmentId = NULL;
    }
    if(keyPairQuery_environmentId){
        keyValuePair_free(keyPairQuery_environmentId);
        keyPairQuery_environmentId = NULL;
    }
    if(keyQuery_traceId){
        free(keyQuery_traceId);
        keyQuery_traceId = NULL;
    }
    if(valueQuery_traceId){
        free(valueQuery_traceId);
        valueQuery_traceId = NULL;
    }
    if(keyPairQuery_traceId){
        keyValuePair_free(keyPairQuery_traceId);
        keyPairQuery_traceId = NULL;
    }
    if(keyQuery_kind){
        free(keyQuery_kind);
        keyQuery_kind = NULL;
    }
    if(valueQuery_kind){
        free(valueQuery_kind);
        valueQuery_kind = NULL;
    }
    if(keyPairQuery_kind){
        keyValuePair_free(keyPairQuery_kind);
        keyPairQuery_kind = NULL;
    }
    if(keyQuery_status){
        free(keyQuery_status);
        keyQuery_status = NULL;
    }
    if(valueQuery_status){
        free(valueQuery_status);
        valueQuery_status = NULL;
    }
    if(keyPairQuery_status){
        keyValuePair_free(keyPairQuery_status);
        keyPairQuery_status = NULL;
    }
    if(keyQuery_startedAfter){
        free(keyQuery_startedAfter);
        keyQuery_startedAfter = NULL;
    }
    if(valueQuery_startedAfter){
        free(valueQuery_startedAfter);
        valueQuery_startedAfter = NULL;
    }
    if(keyPairQuery_startedAfter){
        keyValuePair_free(keyPairQuery_startedAfter);
        keyPairQuery_startedAfter = NULL;
    }
    if(keyQuery_startedBefore){
        free(keyQuery_startedBefore);
        keyQuery_startedBefore = NULL;
    }
    if(valueQuery_startedBefore){
        free(valueQuery_startedBefore);
        valueQuery_startedBefore = NULL;
    }
    if(keyPairQuery_startedBefore){
        keyValuePair_free(keyPairQuery_startedBefore);
        keyPairQuery_startedBefore = NULL;
    }
    if(keyQuery_model){
        free(keyQuery_model);
        keyQuery_model = NULL;
    }
    if(valueQuery_model){
        free(valueQuery_model);
        valueQuery_model = NULL;
    }
    if(keyPairQuery_model){
        keyValuePair_free(keyPairQuery_model);
        keyPairQuery_model = NULL;
    }
    if(keyQuery_release){
        free(keyQuery_release);
        keyQuery_release = NULL;
    }
    if(valueQuery_release){
        free(valueQuery_release);
        valueQuery_release = NULL;
    }
    if(keyPairQuery_release){
        keyValuePair_free(keyPairQuery_release);
        keyPairQuery_release = NULL;
    }
    if(keyQuery_minCostMicros){
        free(keyQuery_minCostMicros);
        keyQuery_minCostMicros = NULL;
    }
    if(keyPairQuery_minCostMicros){
        keyValuePair_free(keyPairQuery_minCostMicros);
        keyPairQuery_minCostMicros = NULL;
    }
    if(keyQuery_maxCostMicros){
        free(keyQuery_maxCostMicros);
        keyQuery_maxCostMicros = NULL;
    }
    if(keyPairQuery_maxCostMicros){
        keyValuePair_free(keyPairQuery_maxCostMicros);
        keyPairQuery_maxCostMicros = NULL;
    }
    if(keyQuery_minLatencyMs){
        free(keyQuery_minLatencyMs);
        keyQuery_minLatencyMs = NULL;
    }
    if(keyPairQuery_minLatencyMs){
        keyValuePair_free(keyPairQuery_minLatencyMs);
        keyPairQuery_minLatencyMs = NULL;
    }
    if(keyQuery_maxLatencyMs){
        free(keyQuery_maxLatencyMs);
        keyQuery_maxLatencyMs = NULL;
    }
    if(keyPairQuery_maxLatencyMs){
        keyValuePair_free(keyPairQuery_maxLatencyMs);
        keyPairQuery_maxLatencyMs = NULL;
    }
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
