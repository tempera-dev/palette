# \EvalResultsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**eval_results_period_get_tempera_evidence**](EvalResultsApi.md#eval_results_period_get_tempera_evidence) | **GET** /v1/eval-results/{tenant_id}/{project_id}/tempera/{kind}/{external_id} |
[**eval_results_period_import_tempera_bundle**](EvalResultsApi.md#eval_results_period_import_tempera_bundle) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/bundles |
[**eval_results_period_record_tempera_decision**](EvalResultsApi.md#eval_results_period_record_tempera_decision) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/decisions |



## eval_results_period_get_tempera_evidence

> models::TemperaEvidenceReceipt eval_results_period_get_tempera_evidence(tenant_id, project_id, kind, external_id, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**kind** | **String** | result_bundle or ab_decision | [required] |
**external_id** | **String** | Bundle or experiment id | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_results_period_import_tempera_bundle

> models::TemperaEvidenceReceipt eval_results_period_import_tempera_bundle(tenant_id, project_id, import_tempera_evidence_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**import_tempera_evidence_request** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_results_period_record_tempera_decision

> models::TemperaEvidenceReceipt eval_results_period_record_tempera_decision(tenant_id, project_id, import_tempera_evidence_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**import_tempera_evidence_request** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
