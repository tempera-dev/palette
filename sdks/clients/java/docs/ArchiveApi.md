# ArchiveApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**archiveArchiveTrace**](ArchiveApi.md#archiveArchiveTrace) | **POST** /v1/archive/{tenantId}/{projectId}/{traceId} |  |
| [**archiveArchiveTraceWithHttpInfo**](ArchiveApi.md#archiveArchiveTraceWithHttpInfo) | **POST** /v1/archive/{tenantId}/{projectId}/{traceId} |  |
| [**archiveQuerySpans**](ArchiveApi.md#archiveQuerySpans) | **GET** /v1/archive/{tenantId}/{projectId}/spans |  |
| [**archiveQuerySpansWithHttpInfo**](ArchiveApi.md#archiveQuerySpansWithHttpInfo) | **GET** /v1/archive/{tenantId}/{projectId}/spans |  |



## archiveArchiveTrace

> ArchiveManifest archiveArchiveTrace(tenantId, projectId, traceId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        // Configure OAuth2 access token for authorization: tempera_oauth
        OAuth tempera_oauth = (OAuth) defaultClient.getAuthentication("tempera_oauth");
        tempera_oauth.setAccessToken("YOUR ACCESS TOKEN");

        // Configure API key authorization: palette_api_key
        ApiKeyAuth palette_api_key = (ApiKeyAuth) defaultClient.getAuthentication("palette_api_key");
        palette_api_key.setApiKey("YOUR API KEY");
        // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
        //palette_api_key.setApiKeyPrefix("Token");

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

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

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
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        // Configure OAuth2 access token for authorization: tempera_oauth
        OAuth tempera_oauth = (OAuth) defaultClient.getAuthentication("tempera_oauth");
        tempera_oauth.setAccessToken("YOUR ACCESS TOKEN");

        // Configure API key authorization: palette_api_key
        ApiKeyAuth palette_api_key = (ApiKeyAuth) defaultClient.getAuthentication("palette_api_key");
        palette_api_key.setApiKey("YOUR API KEY");
        // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
        //palette_api_key.setApiKeyPrefix("Token");

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

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

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


## archiveQuerySpans

> ArchiveQueryResponse archiveQuerySpans(tenantId, projectId, environmentId, traceId, spanId, kind, status, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        // Configure OAuth2 access token for authorization: tempera_oauth
        OAuth tempera_oauth = (OAuth) defaultClient.getAuthentication("tempera_oauth");
        tempera_oauth.setAccessToken("YOUR ACCESS TOKEN");

        // Configure API key authorization: palette_api_key
        ApiKeyAuth palette_api_key = (ApiKeyAuth) defaultClient.getAuthentication("palette_api_key");
        palette_api_key.setApiKey("YOUR API KEY");
        // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
        //palette_api_key.setApiKeyPrefix("Token");

        ArchiveApi apiInstance = new ArchiveApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String environmentId = "environmentId_example"; // String |
        String traceId = "traceId_example"; // String |
        String spanId = "spanId_example"; // String |
        String kind = "kind_example"; // String |
        String status = "status_example"; // String |
        Integer pageSize = 56; // Integer |
        String pageToken = "pageToken_example"; // String |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ArchiveQueryResponse result = apiInstance.archiveQuerySpans(tenantId, projectId, environmentId, traceId, spanId, kind, status, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ArchiveApi#archiveQuerySpans");
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
| **pageSize** | **Integer**|  | [optional] |
| **pageToken** | **String**|  | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ArchiveQueryResponse**](ArchiveQueryResponse.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

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

## archiveQuerySpansWithHttpInfo

> ApiResponse<ArchiveQueryResponse> archiveQuerySpans archiveQuerySpansWithHttpInfo(tenantId, projectId, environmentId, traceId, spanId, kind, status, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ArchiveApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        // Configure OAuth2 access token for authorization: tempera_oauth
        OAuth tempera_oauth = (OAuth) defaultClient.getAuthentication("tempera_oauth");
        tempera_oauth.setAccessToken("YOUR ACCESS TOKEN");

        // Configure API key authorization: palette_api_key
        ApiKeyAuth palette_api_key = (ApiKeyAuth) defaultClient.getAuthentication("palette_api_key");
        palette_api_key.setApiKey("YOUR API KEY");
        // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
        //palette_api_key.setApiKeyPrefix("Token");

        ArchiveApi apiInstance = new ArchiveApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String environmentId = "environmentId_example"; // String |
        String traceId = "traceId_example"; // String |
        String spanId = "spanId_example"; // String |
        String kind = "kind_example"; // String |
        String status = "status_example"; // String |
        Integer pageSize = 56; // Integer |
        String pageToken = "pageToken_example"; // String |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ArchiveQueryResponse> response = apiInstance.archiveQuerySpansWithHttpInfo(tenantId, projectId, environmentId, traceId, spanId, kind, status, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ArchiveApi#archiveQuerySpans");
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
| **pageSize** | **Integer**|  | [optional] |
| **pageToken** | **String**|  | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ArchiveQueryResponse**](ArchiveQueryResponse.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

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
