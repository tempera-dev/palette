#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include "SearchAPI.h"

#define MAX_NUMBER_LENGTH 16
#define MAX_BUFFER_LENGTH 4096


search_span_list_response_t*
SearchAPI_searchSpans(apiClient_t *apiClient, char *tenantId, char *q, char *projectId, char *environmentId, char *traceId, char *spanId, char *kind, char *status, char *model, char *tool, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/search/{tenantId}/spans");

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
    char *keyQuery_q = NULL;
    char * valueQuery_q = NULL;
    keyValuePair_t *keyPairQuery_q = 0;
    if (q)
    {
        keyQuery_q = strdup("q");
        valueQuery_q = strdup((q));
        keyPairQuery_q = keyValuePair_create(keyQuery_q, valueQuery_q);
        list_addElement(localVarQueryParameters,keyPairQuery_q);
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
    char *keyQuery_spanId = NULL;
    char * valueQuery_spanId = NULL;
    keyValuePair_t *keyPairQuery_spanId = 0;
    if (spanId)
    {
        keyQuery_spanId = strdup("spanId");
        valueQuery_spanId = strdup((spanId));
        keyPairQuery_spanId = keyValuePair_create(keyQuery_spanId, valueQuery_spanId);
        list_addElement(localVarQueryParameters,keyPairQuery_spanId);
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
    char *keyQuery_tool = NULL;
    char * valueQuery_tool = NULL;
    keyValuePair_t *keyPairQuery_tool = 0;
    if (tool)
    {
        keyQuery_tool = strdup("tool");
        valueQuery_tool = strdup((tool));
        keyPairQuery_tool = keyValuePair_create(keyQuery_tool, valueQuery_tool);
        list_addElement(localVarQueryParameters,keyPairQuery_tool);
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
    //    printf("%s\n","Search spans");
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
    search_span_list_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *SearchAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = search_span_list_response_parseFromJSON(SearchAPIlocalVarJSON);
        cJSON_Delete(SearchAPIlocalVarJSON);
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
    if(keyQuery_q){
        free(keyQuery_q);
        keyQuery_q = NULL;
    }
    if(valueQuery_q){
        free(valueQuery_q);
        valueQuery_q = NULL;
    }
    if(keyPairQuery_q){
        keyValuePair_free(keyPairQuery_q);
        keyPairQuery_q = NULL;
    }
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
    if(keyQuery_spanId){
        free(keyQuery_spanId);
        keyQuery_spanId = NULL;
    }
    if(valueQuery_spanId){
        free(valueQuery_spanId);
        valueQuery_spanId = NULL;
    }
    if(keyPairQuery_spanId){
        keyValuePair_free(keyPairQuery_spanId);
        keyPairQuery_spanId = NULL;
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
    if(keyQuery_tool){
        free(keyQuery_tool);
        keyQuery_tool = NULL;
    }
    if(valueQuery_tool){
        free(valueQuery_tool);
        valueQuery_tool = NULL;
    }
    if(keyPairQuery_tool){
        keyValuePair_free(keyPairQuery_tool);
        keyPairQuery_tool = NULL;
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
