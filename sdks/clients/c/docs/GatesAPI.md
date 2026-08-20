# GatesAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GatesAPI_gatesCreate**](GatesAPI.md#GatesAPI_gatesCreate) | **POST** /v1/gates/{tenantId}/{projectId} |
[**GatesAPI_gatesRun**](GatesAPI.md#GatesAPI_gatesRun) | **POST** /v1/gates/{tenantId}/{projectId}/{gateId}/run |


# **GatesAPI_gatesCreate**
```c
gate_definition_t* GatesAPI_gatesCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_gate_request_t *create_gate_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**create_gate_request** | **[create_gate_request_t](create_gate_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[gate_definition_t](gate_definition.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GatesAPI_gatesRun**
```c
gate_run_report_t* GatesAPI_gatesRun(apiClient_t *apiClient, char *tenantId, char *projectId, char *gateId, run_gate_request_t *run_gate_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**gateId** | **char \*** | gate_id |
**run_gate_request** | **[run_gate_request_t](run_gate_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[gate_run_report_t](gate_run_report.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
