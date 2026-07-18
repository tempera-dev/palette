# ArchiveAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ArchiveAPI_archiveArchiveTrace**](ArchiveAPI.md#ArchiveAPI_archiveArchiveTrace) | **POST** /v1/archive/{tenant_id}/{project_id}/{trace_id} |
[**ArchiveAPI_archiveQuerySpans**](ArchiveAPI.md#ArchiveAPI_archiveQuerySpans) | **GET** /v1/archive/{tenant_id}/{project_id}/spans |


# **ArchiveAPI_archiveArchiveTrace**
```c
archive_manifest_t* ArchiveAPI_archiveArchiveTrace(apiClient_t *apiClient, char *tenant_id, char *project_id, char *trace_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**trace_id** | **char \*** | trace_id |
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
archive_query_response_t* ArchiveAPI_archiveQuerySpans(apiClient_t *apiClient, char *tenant_id, char *project_id, char *environment_id, char *trace_id, char *span_id, char *kind, char *status, int *limit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**environment_id** | **char \*** |  | [optional]
**trace_id** | **char \*** |  | [optional]
**span_id** | **char \*** |  | [optional]
**kind** | **char \*** |  | [optional]
**status** | **char \*** |  | [optional]
**limit** | **int \*** |  | [optional]
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
