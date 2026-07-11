# ArchiveApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**archiveArchiveTrace**](ArchiveApi.md#archiveArchiveTrace) | **POST** /v1/archive/{tenant_id}/{project_id}/{trace_id} |  |
| [**archiveArchiveTraceWithHttpInfo**](ArchiveApi.md#archiveArchiveTraceWithHttpInfo) | **POST** /v1/archive/{tenant_id}/{project_id}/{trace_id} |  |
| [**archiveQueryArchiveSpans**](ArchiveApi.md#archiveQueryArchiveSpans) | **GET** /v1/archive/{tenant_id}/{project_id}/spans |  |
| [**archiveQueryArchiveSpansWithHttpInfo**](ArchiveApi.md#archiveQueryArchiveSpansWithHttpInfo) | **GET** /v1/archive/{tenant_id}/{project_id}/spans |  |



## archiveArchiveTrace

> ArchiveManifest archiveArchiveTrace(tenantId, projectId, traceId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ArchiveApi apiInstance = new ArchiveApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String traceId = "traceId_example"; // String | trace_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ArchiveManifest result = apiInstance.archiveArchiveTrace(tenantId, projectId, traceId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ArchiveApi#archiveArchiveTrace");
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
| **traceId** | **String**| trace_id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ArchiveManifest**](ArchiveManifest.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Archive a trace to object storage |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## archiveArchiveTraceWithHttpInfo

> ApiResponse<ArchiveManifest> archiveArchiveTrace archiveArchiveTraceWithHttpInfo(tenantId, projectId, traceId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ArchiveApi apiInstance = new ArchiveApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String traceId = "traceId_example"; // String | trace_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ArchiveManifest> response = apiInstance.archiveArchiveTraceWithHttpInfo(tenantId, projectId, traceId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ArchiveApi#archiveArchiveTrace");
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
| **traceId** | **String**| trace_id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ArchiveManifest**](ArchiveManifest.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Archive a trace to object storage |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |


## archiveQueryArchiveSpans

> ArchiveQueryResponse archiveQueryArchiveSpans(tenantId, projectId, environmentId, traceId, spanId, kind, status, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ArchiveApi apiInstance = new ArchiveApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String environmentId = "environmentId_example"; // String |
        String traceId = "traceId_example"; // String |
        String spanId = "spanId_example"; // String |
        String kind = "kind_example"; // String |
        String status = "status_example"; // String |
        Integer limit = 56; // Integer |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ArchiveQueryResponse result = apiInstance.archiveQueryArchiveSpans(tenantId, projectId, environmentId, traceId, spanId, kind, status, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ArchiveApi#archiveQueryArchiveSpans");
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
| **environmentId** | **String**|  | [optional] |
| **traceId** | **String**|  | [optional] |
| **spanId** | **String**|  | [optional] |
| **kind** | **String**|  | [optional] |
| **status** | **String**|  | [optional] |
| **limit** | **Integer**|  | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ArchiveQueryResponse**](ArchiveQueryResponse.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query archived spans |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## archiveQueryArchiveSpansWithHttpInfo

> ApiResponse<ArchiveQueryResponse> archiveQueryArchiveSpans archiveQueryArchiveSpansWithHttpInfo(tenantId, projectId, environmentId, traceId, spanId, kind, status, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ArchiveApi apiInstance = new ArchiveApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String environmentId = "environmentId_example"; // String |
        String traceId = "traceId_example"; // String |
        String spanId = "spanId_example"; // String |
        String kind = "kind_example"; // String |
        String status = "status_example"; // String |
        Integer limit = 56; // Integer |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ArchiveQueryResponse> response = apiInstance.archiveQueryArchiveSpansWithHttpInfo(tenantId, projectId, environmentId, traceId, spanId, kind, status, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ArchiveApi#archiveQueryArchiveSpans");
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
| **environmentId** | **String**|  | [optional] |
| **traceId** | **String**|  | [optional] |
| **spanId** | **String**|  | [optional] |
| **kind** | **String**|  | [optional] |
| **status** | **String**|  | [optional] |
| **limit** | **Integer**|  | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ArchiveQueryResponse**](ArchiveQueryResponse.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query archived spans |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
