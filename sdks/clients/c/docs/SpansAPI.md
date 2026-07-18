# SpansAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**SpansAPI_spansGet**](SpansAPI.md#SpansAPI_spansGet) | **GET** /v1/spans/{tenant_id}/{trace_id}/{span_id} |
[**SpansAPI_spansGetIo**](SpansAPI.md#SpansAPI_spansGetIo) | **GET** /v1/spans/{tenant_id}/{trace_id}/{span_id}/io |


# **SpansAPI_spansGet**
```c
canonical_span_t* SpansAPI_spansGet(apiClient_t *apiClient, char *tenant_id, char *trace_id, char *span_id, int *unmask, char *reason, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**trace_id** | **char \*** | trace_id |
**span_id** | **char \*** | span_id |
**unmask** | **int \*** |  | [optional]
**reason** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[canonical_span_t](canonical_span.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **SpansAPI_spansGetIo**
```c
span_io_response_t* SpansAPI_spansGetIo(apiClient_t *apiClient, char *tenant_id, char *trace_id, char *span_id, int *unmask, char *reason, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**trace_id** | **char \*** | trace_id |
**span_id** | **char \*** | span_id |
**unmask** | **int \*** |  | [optional]
**reason** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[span_io_response_t](span_io_response.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
