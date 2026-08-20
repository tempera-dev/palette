# GatesApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**gatesCreate**](GatesApi.md#gatesCreate) | **POST** /v1/gates/{tenantId}/{projectId} |  |
| [**gatesCreateWithHttpInfo**](GatesApi.md#gatesCreateWithHttpInfo) | **POST** /v1/gates/{tenantId}/{projectId} |  |
| [**gatesRun**](GatesApi.md#gatesRun) | **POST** /v1/gates/{tenantId}/{projectId}/{gateId}/run |  |
| [**gatesRunWithHttpInfo**](GatesApi.md#gatesRunWithHttpInfo) | **POST** /v1/gates/{tenantId}/{projectId}/{gateId}/run |  |



## gatesCreate

> GateDefinition gatesCreate(tenantId, projectId, createGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.GatesApi;

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

        GatesApi apiInstance = new GatesApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateGateRequest createGateRequest = new CreateGateRequest(); // CreateGateRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            GateDefinition result = apiInstance.gatesCreate(tenantId, projectId, createGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling GatesApi#gatesCreate");
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
| **createGateRequest** | [**CreateGateRequest**](CreateGateRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**GateDefinition**](GateDefinition.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a release gate |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## gatesCreateWithHttpInfo

> ApiResponse<GateDefinition> gatesCreate gatesCreateWithHttpInfo(tenantId, projectId, createGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.GatesApi;

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

        GatesApi apiInstance = new GatesApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateGateRequest createGateRequest = new CreateGateRequest(); // CreateGateRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<GateDefinition> response = apiInstance.gatesCreateWithHttpInfo(tenantId, projectId, createGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling GatesApi#gatesCreate");
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
| **createGateRequest** | [**CreateGateRequest**](CreateGateRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**GateDefinition**](GateDefinition.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a release gate |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |


## gatesRun

> GateRunReport gatesRun(tenantId, projectId, gateId, runGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.GatesApi;

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

        GatesApi apiInstance = new GatesApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String gateId = "gateId_example"; // String | gate_id
        RunGateRequest runGateRequest = new RunGateRequest(); // RunGateRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            GateRunReport result = apiInstance.gatesRun(tenantId, projectId, gateId, runGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling GatesApi#gatesRun");
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
| **gateId** | **String**| gate_id | |
| **runGateRequest** | [**RunGateRequest**](RunGateRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**GateRunReport**](GateRunReport.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Run a gate against an experiment |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## gatesRunWithHttpInfo

> ApiResponse<GateRunReport> gatesRun gatesRunWithHttpInfo(tenantId, projectId, gateId, runGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.GatesApi;

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

        GatesApi apiInstance = new GatesApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String gateId = "gateId_example"; // String | gate_id
        RunGateRequest runGateRequest = new RunGateRequest(); // RunGateRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<GateRunReport> response = apiInstance.gatesRunWithHttpInfo(tenantId, projectId, gateId, runGateRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling GatesApi#gatesRun");
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
| **gateId** | **String**| gate_id | |
| **runGateRequest** | [**RunGateRequest**](RunGateRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**GateRunReport**](GateRunReport.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Run a gate against an experiment |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |
