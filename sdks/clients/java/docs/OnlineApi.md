# OnlineApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**onlineDecideSampling**](OnlineApi.md#onlineDecideSampling) | **POST** /v1/online/{tenantId}/{projectId}/traces/{traceId}/sampling |  |
| [**onlineDecideSamplingWithHttpInfo**](OnlineApi.md#onlineDecideSamplingWithHttpInfo) | **POST** /v1/online/{tenantId}/{projectId}/traces/{traceId}/sampling |  |



## onlineDecideSampling

> SamplingDecision onlineDecideSampling(tenantId, projectId, traceId, onlineSamplingPolicy, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.OnlineApi;

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

        OnlineApi apiInstance = new OnlineApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String traceId = "traceId_example"; // String | trace_id
        OnlineSamplingPolicy onlineSamplingPolicy = new OnlineSamplingPolicy(); // OnlineSamplingPolicy |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            SamplingDecision result = apiInstance.onlineDecideSampling(tenantId, projectId, traceId, onlineSamplingPolicy, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling OnlineApi#onlineDecideSampling");
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
| **onlineSamplingPolicy** | [**OnlineSamplingPolicy**](OnlineSamplingPolicy.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**SamplingDecision**](SamplingDecision.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Decide online sampling for a trace |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## onlineDecideSamplingWithHttpInfo

> ApiResponse<SamplingDecision> onlineDecideSampling onlineDecideSamplingWithHttpInfo(tenantId, projectId, traceId, onlineSamplingPolicy, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.OnlineApi;

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

        OnlineApi apiInstance = new OnlineApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String traceId = "traceId_example"; // String | trace_id
        OnlineSamplingPolicy onlineSamplingPolicy = new OnlineSamplingPolicy(); // OnlineSamplingPolicy |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<SamplingDecision> response = apiInstance.onlineDecideSamplingWithHttpInfo(tenantId, projectId, traceId, onlineSamplingPolicy, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling OnlineApi#onlineDecideSampling");
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
| **onlineSamplingPolicy** | [**OnlineSamplingPolicy**](OnlineSamplingPolicy.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**SamplingDecision**](SamplingDecision.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Decide online sampling for a trace |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
