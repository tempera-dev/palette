# TracesAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**TracesAPI_tracesGet**](TracesAPI.md#TracesAPI_tracesGet) | **GET** /v1/traces/{tenantId}/{traceId} |
[**TracesAPI_tracesList**](TracesAPI.md#TracesAPI_tracesList) | **GET** /v1/traces/{tenantId} |


# **TracesAPI_tracesGet**
```c
trace_view_t* TracesAPI_tracesGet(apiClient_t *apiClient, char *tenantId, char *traceId, int *unmask, char *reason, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**traceId** | **char \*** | trace_id |
**unmask** | **int \*** |  | [optional]
**reason** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[trace_view_t](trace_view.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **TracesAPI_tracesList**
```c
trace_list_response_t* TracesAPI_tracesList(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, char *traceId, char *kind, char *status, char *startedAfter, char *startedBefore, char *model, char *release, long minCostMicros, long maxCostMicros, long minLatencyMs, long maxLatencyMs, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** |  | [optional]
**environmentId** | **char \*** |  | [optional]
**traceId** | **char \*** |  | [optional]
**kind** | **char \*** |  | [optional]
**status** | **char \*** |  | [optional]
**startedAfter** | **char \*** |  | [optional]
**startedBefore** | **char \*** |  | [optional]
**model** | **char \*** |  | [optional]
**release** | **char \*** |  | [optional]
**minCostMicros** | **long** |  | [optional]
**maxCostMicros** | **long** |  | [optional]
**minLatencyMs** | **long** |  | [optional]
**maxLatencyMs** | **long** |  | [optional]
**pageSize** | **int \*** |  | [optional]
**pageToken** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[trace_list_response_t](trace_list_response.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
