# JudgeAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**JudgeAPI_judgeEvaluate**](JudgeAPI.md#JudgeAPI_judgeEvaluate) | **POST** /v1/judge/{tenantId}/{projectId}/evaluate |
[**JudgeAPI_judgeListLedger**](JudgeAPI.md#JudgeAPI_judgeListLedger) | **GET** /v1/judge/{tenantId}/{projectId}/ledger |


# **JudgeAPI_judgeEvaluate**
```c
judge_broker_outcome_t* JudgeAPI_judgeEvaluate(apiClient_t *apiClient, char *tenantId, char *projectId, run_judge_eval_http_request_t *run_judge_eval_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**run_judge_eval_http_request** | **[run_judge_eval_http_request_t](run_judge_eval_http_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[judge_broker_outcome_t](judge_broker_outcome.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **JudgeAPI_judgeListLedger**
```c
judge_ledger_list_response_t* JudgeAPI_judgeListLedger(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**pageSize** | **int \*** | Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional]
**pageToken** | **char \*** | Opaque continuation token returned by the preceding list request. | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[judge_ledger_list_response_t](judge_ledger_list_response.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
