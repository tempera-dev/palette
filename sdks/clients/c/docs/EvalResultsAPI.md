# EvalResultsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**EvalResultsAPI_evalResultsGetTemperaEvidence**](EvalResultsAPI.md#EvalResultsAPI_evalResultsGetTemperaEvidence) | **GET** /v1/eval-results/{tenant_id}/{project_id}/tempera/{kind}/{external_id} |
[**EvalResultsAPI_evalResultsImportTemperaBundle**](EvalResultsAPI.md#EvalResultsAPI_evalResultsImportTemperaBundle) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/bundles |
[**EvalResultsAPI_evalResultsRecordTemperaDecision**](EvalResultsAPI.md#EvalResultsAPI_evalResultsRecordTemperaDecision) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/decisions |


# **EvalResultsAPI_evalResultsGetTemperaEvidence**
```c
tempera_evidence_receipt_t* EvalResultsAPI_evalResultsGetTemperaEvidence(apiClient_t *apiClient, char *tenant_id, char *project_id, char *kind, char *external_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**kind** | **char \*** | result_bundle or ab_decision |
**external_id** | **char \*** | Bundle or experiment id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[tempera_evidence_receipt_t](tempera_evidence_receipt.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **EvalResultsAPI_evalResultsImportTemperaBundle**
```c
tempera_evidence_receipt_t* EvalResultsAPI_evalResultsImportTemperaBundle(apiClient_t *apiClient, char *tenant_id, char *project_id, import_tempera_evidence_request_t *import_tempera_evidence_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**import_tempera_evidence_request** | **[import_tempera_evidence_request_t](import_tempera_evidence_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[tempera_evidence_receipt_t](tempera_evidence_receipt.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **EvalResultsAPI_evalResultsRecordTemperaDecision**
```c
tempera_evidence_receipt_t* EvalResultsAPI_evalResultsRecordTemperaDecision(apiClient_t *apiClient, char *tenant_id, char *project_id, import_tempera_evidence_request_t *import_tempera_evidence_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**import_tempera_evidence_request** | **[import_tempera_evidence_request_t](import_tempera_evidence_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[tempera_evidence_receipt_t](tempera_evidence_receipt.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
