# AlertsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AlertsAPI_alertsEvaluate**](AlertsAPI.md#AlertsAPI_alertsEvaluate) | **POST** /v1/alerts/{tenantId}/{projectId}/traces/{traceId}/webhook |


# **AlertsAPI_alertsEvaluate**
```c
alert_decision_t* AlertsAPI_alertsEvaluate(apiClient_t *apiClient, char *tenantId, char *projectId, char *traceId, evaluate_alert_request_t *evaluate_alert_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**traceId** | **char \*** | trace_id |
**evaluate_alert_request** | **[evaluate_alert_request_t](evaluate_alert_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[alert_decision_t](alert_decision.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
