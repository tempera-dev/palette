# AuditAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AuditAPI_auditList**](AuditAPI.md#AuditAPI_auditList) | **GET** /v1/audit/{tenant_id}/{project_id} |


# **AuditAPI_auditList**
```c
audit_event_list_response_t* AuditAPI_auditList(apiClient_t *apiClient, char *tenant_id, char *project_id, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**pageSize** | **int \*** | Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional]
**pageToken** | **char \*** | Opaque continuation token returned by the preceding list request. | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[audit_event_list_response_t](audit_event_list_response.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
