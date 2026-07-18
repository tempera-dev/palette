# UsageApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**usageGetSummary**](UsageApi.md#usageGetSummary) | **GET** /v1/usage/{tenant_id}/{project_id} |  |
| [**usageGetSummaryWithHttpInfo**](UsageApi.md#usageGetSummaryWithHttpInfo) | **GET** /v1/usage/{tenant_id}/{project_id} |  |



## usageGetSummary

> UsageSummary usageGetSummary(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.UsageApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        UsageApi apiInstance = new UsageApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            UsageSummary result = apiInstance.usageGetSummary(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling UsageApi#usageGetSummary");
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

[**UsageSummary**](UsageSummary.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get usage summary |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## usageGetSummaryWithHttpInfo

> ApiResponse<UsageSummary> usageGetSummary usageGetSummaryWithHttpInfo(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.UsageApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        UsageApi apiInstance = new UsageApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<UsageSummary> response = apiInstance.usageGetSummaryWithHttpInfo(tenantId, projectId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling UsageApi#usageGetSummary");
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

ApiResponse<[**UsageSummary**](UsageSummary.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get usage summary |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
