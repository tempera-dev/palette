# ArchiveAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ArchiveAPI_archiveArchiveTrace**](ArchiveAPI.md#ArchiveAPI_archiveArchiveTrace) | **POST** /v1/archive/{tenantId}/{projectId}/{traceId} |
[**ArchiveAPI_archiveQuerySpans**](ArchiveAPI.md#ArchiveAPI_archiveQuerySpans) | **GET** /v1/archive/{tenantId}/{projectId}/spans |


# **ArchiveAPI_archiveArchiveTrace**
```c
archive_manifest_t* ArchiveAPI_archiveArchiveTrace(apiClient_t *apiClient, char *tenantId, char *projectId, char *traceId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**traceId** | **char \*** | trace_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[archive_manifest_t](archive_manifest.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ArchiveAPI_archiveQuerySpans**
```c
archive_query_response_t* ArchiveAPI_archiveQuerySpans(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, char *traceId, char *spanId, char *kind, char *status, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**environmentId** | **char \*** |  | [optional]
**traceId** | **char \*** |  | [optional]
**spanId** | **char \*** |  | [optional]
**kind** | **char \*** |  | [optional]
**status** | **char \*** |  | [optional]
**pageSize** | **int \*** |  | [optional]
**pageToken** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[archive_query_response_t](archive_query_response.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
