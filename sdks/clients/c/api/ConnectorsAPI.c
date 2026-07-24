#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include "ConnectorsAPI.h"

#define MAX_NUMBER_LENGTH 16
#define MAX_BUFFER_LENGTH 4096


connection_link_t*
ConnectorsAPI_connectorsConnect(apiClient_t *apiClient, char *tenantId, char *projectId, connect_connector_request_t *connect_connector_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/connectors/{tenantId}/{projectId}/connect");

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
    cJSON *localVarSingleItemJSON_connect_connector_request = NULL;
    if (connect_connector_request != NULL)
    {
        //not string, not binary
        localVarSingleItemJSON_connect_connector_request = connect_connector_request_convertToJSON(connect_connector_request);
        localVarBodyParameters = cJSON_Print(localVarSingleItemJSON_connect_connector_request);
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
    //    printf("%s\n","One-time login link to authorize the app");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","Invalid request, scope, or filter");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","Missing or invalid credentials");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","Credentials lack the required scope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 501) {
    //    printf("%s\n","Connector provider not configured");
    //}
    //nonprimitive not container
    connection_link_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *ConnectorsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = connection_link_parseFromJSON(ConnectorsAPIlocalVarJSON);
        cJSON_Delete(ConnectorsAPIlocalVarJSON);
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
    if (localVarSingleItemJSON_connect_connector_request) {
        cJSON_Delete(localVarSingleItemJSON_connect_connector_request);
        localVarSingleItemJSON_connect_connector_request = NULL;
    }
    free(localVarBodyParameters);
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

connector_skills_response_t*
ConnectorsAPI_connectorsGetSkills(apiClient_t *apiClient, char *tenantId, char *projectId, char *toolkit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/connectors/{tenantId}/{projectId}/skills");

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
    char *keyQuery_toolkit = NULL;
    char * valueQuery_toolkit = NULL;
    keyValuePair_t *keyPairQuery_toolkit = 0;
    if (toolkit)
    {
        keyQuery_toolkit = strdup("toolkit");
        valueQuery_toolkit = strdup((toolkit));
        keyPairQuery_toolkit = keyValuePair_create(keyQuery_toolkit, valueQuery_toolkit);
        list_addElement(localVarQueryParameters,keyPairQuery_toolkit);
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
    //    printf("%s\n","Generated prompting scaffold (skill cards) for a toolkit");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","Invalid request, scope, or filter");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","Missing or invalid credentials");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","Credentials lack the required scope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 501) {
    //    printf("%s\n","Connector provider not configured");
    //}
    //nonprimitive not container
    connector_skills_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *ConnectorsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = connector_skills_response_parseFromJSON(ConnectorsAPIlocalVarJSON);
        cJSON_Delete(ConnectorsAPIlocalVarJSON);
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
    if(keyQuery_toolkit){
        free(keyQuery_toolkit);
        keyQuery_toolkit = NULL;
    }
    if(valueQuery_toolkit){
        free(valueQuery_toolkit);
        valueQuery_toolkit = NULL;
    }
    if(keyPairQuery_toolkit){
        keyValuePair_free(keyPairQuery_toolkit);
        keyPairQuery_toolkit = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

tool_execution_t*
ConnectorsAPI_connectorsInvokeTool(apiClient_t *apiClient, char *tenantId, char *projectId, invoke_connector_request_t *invoke_connector_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/connectors/{tenantId}/{projectId}/invoke");

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
    cJSON *localVarSingleItemJSON_invoke_connector_request = NULL;
    if (invoke_connector_request != NULL)
    {
        //not string, not binary
        localVarSingleItemJSON_invoke_connector_request = invoke_connector_request_convertToJSON(invoke_connector_request);
        localVarBodyParameters = cJSON_Print(localVarSingleItemJSON_invoke_connector_request);
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
    //    printf("%s\n","Execute a connector tool and return its result envelope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","Invalid request, scope, or filter");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","Missing or invalid credentials");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","Credentials lack the required scope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 501) {
    //    printf("%s\n","Connector provider not configured");
    //}
    //nonprimitive not container
    tool_execution_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *ConnectorsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = tool_execution_parseFromJSON(ConnectorsAPIlocalVarJSON);
        cJSON_Delete(ConnectorsAPIlocalVarJSON);
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
    if (localVarSingleItemJSON_invoke_connector_request) {
        cJSON_Delete(localVarSingleItemJSON_invoke_connector_request);
        localVarSingleItemJSON_invoke_connector_request = NULL;
    }
    free(localVarBodyParameters);
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}

connector_list_response_t*
ConnectorsAPI_connectorsList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/connectors/{tenantId}/{projectId}");

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
    //    printf("%s\n","List connectable third-party apps (catalog)");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","Invalid request, scope, or filter");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","Missing or invalid credentials");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","Credentials lack the required scope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 501) {
    //    printf("%s\n","Connector provider not configured");
    //}
    //nonprimitive not container
    connector_list_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *ConnectorsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = connector_list_response_parseFromJSON(ConnectorsAPIlocalVarJSON);
        cJSON_Delete(ConnectorsAPIlocalVarJSON);
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

connector_tool_list_response_t*
ConnectorsAPI_connectorsListTools(apiClient_t *apiClient, char *tenantId, char *projectId, char *toolkit, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/connectors/{tenantId}/{projectId}/tools");

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
    char *keyQuery_toolkit = NULL;
    char * valueQuery_toolkit = NULL;
    keyValuePair_t *keyPairQuery_toolkit = 0;
    if (toolkit)
    {
        keyQuery_toolkit = strdup("toolkit");
        valueQuery_toolkit = strdup((toolkit));
        keyPairQuery_toolkit = keyValuePair_create(keyQuery_toolkit, valueQuery_toolkit);
        list_addElement(localVarQueryParameters,keyPairQuery_toolkit);
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
    //    printf("%s\n","List a toolkit&#39;s executable tools with input schemas");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","Invalid request, scope, or filter");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","Missing or invalid credentials");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","Credentials lack the required scope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 501) {
    //    printf("%s\n","Connector provider not configured");
    //}
    //nonprimitive not container
    connector_tool_list_response_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *ConnectorsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = connector_tool_list_response_parseFromJSON(ConnectorsAPIlocalVarJSON);
        cJSON_Delete(ConnectorsAPIlocalVarJSON);
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
    if(keyQuery_toolkit){
        free(keyQuery_toolkit);
        keyQuery_toolkit = NULL;
    }
    if(valueQuery_toolkit){
        free(valueQuery_toolkit);
        valueQuery_toolkit = NULL;
    }
    if(keyPairQuery_toolkit){
        keyValuePair_free(keyPairQuery_toolkit);
        keyPairQuery_toolkit = NULL;
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

connection_status_t*
ConnectorsAPI_connectorsStatus(apiClient_t *apiClient, char *tenantId, char *projectId, char *toolkit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id)
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
    char *localVarPath = strdup("/v1/connectors/{tenantId}/{projectId}/status");

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
    char *keyQuery_toolkit = NULL;
    char * valueQuery_toolkit = NULL;
    keyValuePair_t *keyPairQuery_toolkit = 0;
    if (toolkit)
    {
        keyQuery_toolkit = strdup("toolkit");
        valueQuery_toolkit = strdup((toolkit));
        keyPairQuery_toolkit = keyValuePair_create(keyQuery_toolkit, valueQuery_toolkit);
        list_addElement(localVarQueryParameters,keyPairQuery_toolkit);
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
    //    printf("%s\n","Connection status of a toolkit for this project");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 400) {
    //    printf("%s\n","Invalid request, scope, or filter");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 401) {
    //    printf("%s\n","Missing or invalid credentials");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 403) {
    //    printf("%s\n","Credentials lack the required scope");
    //}
    // uncomment below to debug the error response
    //if (apiClient->response_code == 501) {
    //    printf("%s\n","Connector provider not configured");
    //}
    //nonprimitive not container
    connection_status_t *elementToReturn = NULL;
    if(apiClient->response_code >= 200 && apiClient->response_code < 300) {
        cJSON *ConnectorsAPIlocalVarJSON = cJSON_Parse(apiClient->dataReceived);
        elementToReturn = connection_status_parseFromJSON(ConnectorsAPIlocalVarJSON);
        cJSON_Delete(ConnectorsAPIlocalVarJSON);
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
    if(keyQuery_toolkit){
        free(keyQuery_toolkit);
        keyQuery_toolkit = NULL;
    }
    if(valueQuery_toolkit){
        free(valueQuery_toolkit);
        valueQuery_toolkit = NULL;
    }
    if(keyPairQuery_toolkit){
        keyValuePair_free(keyPairQuery_toolkit);
        keyPairQuery_toolkit = NULL;
    }
    return elementToReturn;
end:
    free(localVarPath);
    return NULL;

}
