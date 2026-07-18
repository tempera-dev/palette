# DatasetsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DatasetsAPI_datasetsCreate**](DatasetsAPI.md#DatasetsAPI_datasetsCreate) | **POST** /v1/datasets/{tenant_id}/{project_id} |
[**DatasetsAPI_datasetsCreateVersion**](DatasetsAPI.md#DatasetsAPI_datasetsCreateVersion) | **POST** /v1/datasets/{tenant_id}/{project_id}/{dataset_id}/versions |
[**DatasetsAPI_datasetsPromoteCaseFromTrace**](DatasetsAPI.md#DatasetsAPI_datasetsPromoteCaseFromTrace) | **POST** /v1/datasets/{tenant_id}/{project_id}/{dataset_id}/cases/from-trace |


# **DatasetsAPI_datasetsCreate**
```c
dataset_t* DatasetsAPI_datasetsCreate(apiClient_t *apiClient, char *tenant_id, char *project_id, create_dataset_request_t *create_dataset_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**create_dataset_request** | **[create_dataset_request_t](create_dataset_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_t](dataset.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DatasetsAPI_datasetsCreateVersion**
```c
dataset_version_snapshot_t* DatasetsAPI_datasetsCreateVersion(apiClient_t *apiClient, char *tenant_id, char *project_id, char *dataset_id, create_dataset_version_request_t *create_dataset_version_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**dataset_id** | **char \*** | dataset_id |
**create_dataset_version_request** | **[create_dataset_version_request_t](create_dataset_version_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_version_snapshot_t](dataset_version_snapshot.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DatasetsAPI_datasetsPromoteCaseFromTrace**
```c
dataset_case_t* DatasetsAPI_datasetsPromoteCaseFromTrace(apiClient_t *apiClient, char *tenant_id, char *project_id, char *dataset_id, promote_trace_case_request_t *promote_trace_case_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**dataset_id** | **char \*** | dataset_id |
**promote_trace_case_request** | **[promote_trace_case_request_t](promote_trace_case_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_case_t](dataset_case.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
