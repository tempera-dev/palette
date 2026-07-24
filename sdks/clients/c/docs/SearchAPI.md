# SearchAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**SearchAPI_searchSpans**](SearchAPI.md#SearchAPI_searchSpans) | **GET** /v1/search/{tenantId}/spans |


# **SearchAPI_searchSpans**
```c
search_span_list_response_t* SearchAPI_searchSpans(apiClient_t *apiClient, char *tenantId, char *q, char *projectId, char *environmentId, char *traceId, char *spanId, char *kind, char *status, char *model, char *tool, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**q** | **char \*** |  | [optional]
**projectId** | **char \*** |  | [optional]
**environmentId** | **char \*** |  | [optional]
**traceId** | **char \*** |  | [optional]
**spanId** | **char \*** |  | [optional]
**kind** | **char \*** |  | [optional]
**status** | **char \*** |  | [optional]
**model** | **char \*** |  | [optional]
**tool** | **char \*** |  | [optional]
**pageSize** | **int \*** |  | [optional]
**pageToken** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[search_span_list_response_t](search_span_list_response.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
