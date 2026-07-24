# \JudgeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**judge_period_evaluate**](JudgeApi.md#judge_period_evaluate) | **POST** /v1/judge/{tenantId}/{projectId}/evaluate |
[**judge_period_list_ledger**](JudgeApi.md#judge_period_list_ledger) | **GET** /v1/judge/{tenantId}/{projectId}/ledger |



## judge_period_evaluate

> models::JudgeBrokerOutcome judge_period_evaluate(tenant_id, project_id, run_judge_eval_http_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**run_judge_eval_http_request** | [**RunJudgeEvalHttpRequest**](RunJudgeEvalHttpRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::JudgeBrokerOutcome**](JudgeBrokerOutcome.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## judge_period_list_ledger

> models::JudgeLedgerListResponse judge_period_list_ledger(tenant_id, project_id, page_size, page_token, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**page_size** | Option<**i32**> | Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum. |  |
**page_token** | Option<**String**> | Opaque continuation token returned by the preceding list request. |  |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::JudgeLedgerListResponse**](JudgeLedgerListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
