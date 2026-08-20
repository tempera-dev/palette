# DatasetsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DatasetsAPI_datasetsCreate**](DatasetsAPI.md#DatasetsAPI_datasetsCreate) | **POST** /v1/datasets/{tenantId}/{projectId} |
[**DatasetsAPI_datasetsCreateVersion**](DatasetsAPI.md#DatasetsAPI_datasetsCreateVersion) | **POST** /v1/datasets/{tenantId}/{projectId}/{datasetId}/versions |
[**DatasetsAPI_datasetsPromoteCaseFromTrace**](DatasetsAPI.md#DatasetsAPI_datasetsPromoteCaseFromTrace) | **POST** /v1/datasets/{tenantId}/{projectId}/{datasetId}/cases/from-trace |


# **DatasetsAPI_datasetsCreate**
```c
dataset_t* DatasetsAPI_datasetsCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_dataset_request_t *create_dataset_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**create_dataset_request** | **[create_dataset_request_t](create_dataset_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_t](dataset.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DatasetsAPI_datasetsCreateVersion**
```c
dataset_version_snapshot_t* DatasetsAPI_datasetsCreateVersion(apiClient_t *apiClient, char *tenantId, char *projectId, char *datasetId, create_dataset_version_request_t *create_dataset_version_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**datasetId** | **char \*** | dataset_id |
**create_dataset_version_request** | **[create_dataset_version_request_t](create_dataset_version_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_version_snapshot_t](dataset_version_snapshot.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DatasetsAPI_datasetsPromoteCaseFromTrace**
```c
dataset_case_t* DatasetsAPI_datasetsPromoteCaseFromTrace(apiClient_t *apiClient, char *tenantId, char *projectId, char *datasetId, promote_trace_case_request_t *promote_trace_case_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**datasetId** | **char \*** | dataset_id |
**promote_trace_case_request** | **[promote_trace_case_request_t](promote_trace_case_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_case_t](dataset_case.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
