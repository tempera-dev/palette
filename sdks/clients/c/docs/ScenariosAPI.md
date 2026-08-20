# ScenariosAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ScenariosAPI_scenariosCreate**](ScenariosAPI.md#ScenariosAPI_scenariosCreate) | **POST** /v1/scenarios/{tenantId}/{projectId} |
[**ScenariosAPI_scenariosGet**](ScenariosAPI.md#ScenariosAPI_scenariosGet) | **GET** /v1/scenarios/{tenantId}/{projectId}/{scenarioId} |
[**ScenariosAPI_scenariosList**](ScenariosAPI.md#ScenariosAPI_scenariosList) | **GET** /v1/scenarios/{tenantId}/{projectId} |
[**ScenariosAPI_scenariosMine**](ScenariosAPI.md#ScenariosAPI_scenariosMine) | **POST** /v1/scenarios/{tenantId}/{projectId}/mine |


# **ScenariosAPI_scenariosCreate**
```c
scenario_t* ScenariosAPI_scenariosCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_scenario_request_t *create_scenario_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**create_scenario_request** | **[create_scenario_request_t](create_scenario_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[scenario_t](scenario.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ScenariosAPI_scenariosGet**
```c
scenario_t* ScenariosAPI_scenariosGet(apiClient_t *apiClient, char *tenantId, char *projectId, char *scenarioId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**scenarioId** | **char \*** | scenario_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[scenario_t](scenario.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ScenariosAPI_scenariosList**
```c
list_scenarios_response_t* ScenariosAPI_scenariosList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**pageSize** | **int \*** | Maximum number of scenarios to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional]
**pageToken** | **char \*** | Opaque continuation token returned by the preceding list request. | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[list_scenarios_response_t](list_scenarios_response.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ScenariosAPI_scenariosMine**
```c
mine_scenarios_response_t* ScenariosAPI_scenariosMine(apiClient_t *apiClient, char *tenantId, char *projectId, mine_scenarios_request_t *mine_scenarios_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**mine_scenarios_request** | **[mine_scenarios_request_t](mine_scenarios_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[mine_scenarios_response_t](mine_scenarios_response.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
