# \JudgeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**judge_period_evaluate**](JudgeApi.md#judge_period_evaluate) | **POST** /v1/judge/{tenant_id}/{project_id}/evaluate |
[**judge_period_list_ledger**](JudgeApi.md#judge_period_list_ledger) | **GET** /v1/judge/{tenant_id}/{project_id}/ledger |



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

> Vec<models::PublicJudgeAuditRecord> judge_period_list_ledger(tenant_id, project_id, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**Vec<models::PublicJudgeAuditRecord>**](PublicJudgeAuditRecord.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
