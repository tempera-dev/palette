# AuditApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**auditList**](AuditApi.md#auditList) | **GET** /v1/audit/{tenant_id}/{project_id} |  |
| [**auditListWithHttpInfo**](AuditApi.md#auditListWithHttpInfo) | **GET** /v1/audit/{tenant_id}/{project_id} |  |



## auditList

> List<AuditEvent> auditList(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.AuditApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        AuditApi apiInstance = new AuditApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            List<AuditEvent> result = apiInstance.auditList(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling AuditApi#auditList");
            System.err.println("Status code: " + e.getCode());
            System.err.println("Reason: " + e.getResponseBody());
            System.err.println("Response headers: " + e.getResponseHeaders());
            e.printStackTrace();
        }
    }
}
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **tenantId** | **String**| tenant_id | |
| **projectId** | **String**| project_id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**List&lt;AuditEvent&gt;**](AuditEvent.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List audit events |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## auditListWithHttpInfo

> ApiResponse<List<AuditEvent>> auditList auditListWithHttpInfo(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.AuditApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        AuditApi apiInstance = new AuditApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<List<AuditEvent>> response = apiInstance.auditListWithHttpInfo(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling AuditApi#auditList");
            System.err.println("Status code: " + e.getCode());
            System.err.println("Response headers: " + e.getResponseHeaders());
            System.err.println("Reason: " + e.getResponseBody());
            e.printStackTrace();
        }
    }
}
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **tenantId** | **String**| tenant_id | |
| **projectId** | **String**| project_id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**List&lt;AuditEvent&gt;**](AuditEvent.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List audit events |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
