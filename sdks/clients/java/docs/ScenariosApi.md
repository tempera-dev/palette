# ScenariosApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**scenariosCreate**](ScenariosApi.md#scenariosCreate) | **POST** /v1/scenarios/{tenantId}/{projectId} |  |
| [**scenariosCreateWithHttpInfo**](ScenariosApi.md#scenariosCreateWithHttpInfo) | **POST** /v1/scenarios/{tenantId}/{projectId} |  |
| [**scenariosGet**](ScenariosApi.md#scenariosGet) | **GET** /v1/scenarios/{tenantId}/{projectId}/{scenarioId} |  |
| [**scenariosGetWithHttpInfo**](ScenariosApi.md#scenariosGetWithHttpInfo) | **GET** /v1/scenarios/{tenantId}/{projectId}/{scenarioId} |  |
| [**scenariosList**](ScenariosApi.md#scenariosList) | **GET** /v1/scenarios/{tenantId}/{projectId} |  |
| [**scenariosListWithHttpInfo**](ScenariosApi.md#scenariosListWithHttpInfo) | **GET** /v1/scenarios/{tenantId}/{projectId} |  |
| [**scenariosMine**](ScenariosApi.md#scenariosMine) | **POST** /v1/scenarios/{tenantId}/{projectId}/mine |  |
| [**scenariosMineWithHttpInfo**](ScenariosApi.md#scenariosMineWithHttpInfo) | **POST** /v1/scenarios/{tenantId}/{projectId}/mine |  |



## scenariosCreate

> Scenario scenariosCreate(tenantId, projectId, createScenarioRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateScenarioRequest createScenarioRequest = new CreateScenarioRequest(); // CreateScenarioRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            Scenario result = apiInstance.scenariosCreate(tenantId, projectId, createScenarioRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosCreate");
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
| **createScenarioRequest** | [**CreateScenarioRequest**](CreateScenarioRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**Scenario**](Scenario.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a scenario |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## scenariosCreateWithHttpInfo

> ApiResponse<Scenario> scenariosCreate scenariosCreateWithHttpInfo(tenantId, projectId, createScenarioRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateScenarioRequest createScenarioRequest = new CreateScenarioRequest(); // CreateScenarioRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<Scenario> response = apiInstance.scenariosCreateWithHttpInfo(tenantId, projectId, createScenarioRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosCreate");
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
| **createScenarioRequest** | [**CreateScenarioRequest**](CreateScenarioRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**Scenario**](Scenario.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a scenario |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |


## scenariosGet

> Scenario scenariosGet(tenantId, projectId, scenarioId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String scenarioId = "scenarioId_example"; // String | scenario_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            Scenario result = apiInstance.scenariosGet(tenantId, projectId, scenarioId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosGet");
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
| **scenarioId** | **String**| scenario_id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**Scenario**](Scenario.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get a scenario |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## scenariosGetWithHttpInfo

> ApiResponse<Scenario> scenariosGet scenariosGetWithHttpInfo(tenantId, projectId, scenarioId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String scenarioId = "scenarioId_example"; // String | scenario_id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<Scenario> response = apiInstance.scenariosGetWithHttpInfo(tenantId, projectId, scenarioId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosGet");
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
| **scenarioId** | **String**| scenario_id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**Scenario**](Scenario.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get a scenario |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |


## scenariosList

> ListScenariosResponse scenariosList(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        Integer pageSize = 56; // Integer | Maximum number of scenarios to return. Zero selects the server default; values above the service maximum are coerced to that maximum.
        String pageToken = "pageToken_example"; // String | Opaque continuation token returned by the preceding list request.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ListScenariosResponse result = apiInstance.scenariosList(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosList");
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
| **pageSize** | **Integer**| Maximum number of scenarios to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional] |
| **pageToken** | **String**| Opaque continuation token returned by the preceding list request. | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ListScenariosResponse**](ListScenariosResponse.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List scenarios |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## scenariosListWithHttpInfo

> ApiResponse<ListScenariosResponse> scenariosList scenariosListWithHttpInfo(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        Integer pageSize = 56; // Integer | Maximum number of scenarios to return. Zero selects the server default; values above the service maximum are coerced to that maximum.
        String pageToken = "pageToken_example"; // String | Opaque continuation token returned by the preceding list request.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ListScenariosResponse> response = apiInstance.scenariosListWithHttpInfo(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosList");
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
| **pageSize** | **Integer**| Maximum number of scenarios to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional] |
| **pageToken** | **String**| Opaque continuation token returned by the preceding list request. | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ListScenariosResponse**](ListScenariosResponse.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List scenarios |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |


## scenariosMine

> MineScenariosResponse scenariosMine(tenantId, projectId, mineScenariosRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        MineScenariosRequest mineScenariosRequest = new MineScenariosRequest(); // MineScenariosRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            MineScenariosResponse result = apiInstance.scenariosMine(tenantId, projectId, mineScenariosRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosMine");
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
| **mineScenariosRequest** | [**MineScenariosRequest**](MineScenariosRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**MineScenariosResponse**](MineScenariosResponse.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Mine scenario clusters from traces |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## scenariosMineWithHttpInfo

> ApiResponse<MineScenariosResponse> scenariosMine scenariosMineWithHttpInfo(tenantId, projectId, mineScenariosRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ScenariosApi;

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

        ScenariosApi apiInstance = new ScenariosApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        MineScenariosRequest mineScenariosRequest = new MineScenariosRequest(); // MineScenariosRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<MineScenariosResponse> response = apiInstance.scenariosMineWithHttpInfo(tenantId, projectId, mineScenariosRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ScenariosApi#scenariosMine");
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
| **mineScenariosRequest** | [**MineScenariosRequest**](MineScenariosRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**MineScenariosResponse**](MineScenariosResponse.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Mine scenario clusters from traces |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |
