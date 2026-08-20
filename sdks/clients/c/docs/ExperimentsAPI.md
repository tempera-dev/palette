# ExperimentsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ExperimentsAPI_experimentsRunDeterministic**](ExperimentsAPI.md#ExperimentsAPI_experimentsRunDeterministic) | **POST** /v1/experiments/{tenantId}/{projectId}/{datasetId}/versions/{versionId}/deterministic |
[**ExperimentsAPI_experimentsRunJudge**](ExperimentsAPI.md#ExperimentsAPI_experimentsRunJudge) | **POST** /v1/experiments/{tenantId}/{projectId}/{datasetId}/versions/{versionId}/judge |


# **ExperimentsAPI_experimentsRunDeterministic**
```c
experiment_run_report_t* ExperimentsAPI_experimentsRunDeterministic(apiClient_t *apiClient, char *tenantId, char *projectId, char *datasetId, char *versionId, run_experiment_request_t *run_experiment_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**datasetId** | **char \*** | dataset_id |
**versionId** | **char \*** | version_id |
**run_experiment_request** | **[run_experiment_request_t](run_experiment_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[experiment_run_report_t](experiment_run_report.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ExperimentsAPI_experimentsRunJudge**
```c
experiment_run_report_t* ExperimentsAPI_experimentsRunJudge(apiClient_t *apiClient, char *tenantId, char *projectId, char *datasetId, char *versionId, run_judge_experiment_request_t *run_judge_experiment_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**datasetId** | **char \*** | dataset_id |
**versionId** | **char \*** | version_id |
**run_judge_experiment_request** | **[run_judge_experiment_request_t](run_judge_experiment_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[experiment_run_report_t](experiment_run_report.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
