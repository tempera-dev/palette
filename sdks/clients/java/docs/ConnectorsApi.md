# ConnectorsApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**connectorsConnect**](ConnectorsApi.md#connectorsConnect) | **POST** /v1/connectors/{tenant_id}/{project_id}/connect |  |
| [**connectorsConnectWithHttpInfo**](ConnectorsApi.md#connectorsConnectWithHttpInfo) | **POST** /v1/connectors/{tenant_id}/{project_id}/connect |  |
| [**connectorsGetSkills**](ConnectorsApi.md#connectorsGetSkills) | **GET** /v1/connectors/{tenant_id}/{project_id}/skills |  |
| [**connectorsGetSkillsWithHttpInfo**](ConnectorsApi.md#connectorsGetSkillsWithHttpInfo) | **GET** /v1/connectors/{tenant_id}/{project_id}/skills |  |
| [**connectorsInvokeTool**](ConnectorsApi.md#connectorsInvokeTool) | **POST** /v1/connectors/{tenant_id}/{project_id}/invoke |  |
| [**connectorsInvokeToolWithHttpInfo**](ConnectorsApi.md#connectorsInvokeToolWithHttpInfo) | **POST** /v1/connectors/{tenant_id}/{project_id}/invoke |  |
| [**connectorsList**](ConnectorsApi.md#connectorsList) | **GET** /v1/connectors/{tenant_id}/{project_id} |  |
| [**connectorsListWithHttpInfo**](ConnectorsApi.md#connectorsListWithHttpInfo) | **GET** /v1/connectors/{tenant_id}/{project_id} |  |
| [**connectorsListTools**](ConnectorsApi.md#connectorsListTools) | **GET** /v1/connectors/{tenant_id}/{project_id}/tools |  |
| [**connectorsListToolsWithHttpInfo**](ConnectorsApi.md#connectorsListToolsWithHttpInfo) | **GET** /v1/connectors/{tenant_id}/{project_id}/tools |  |
| [**connectorsStatus**](ConnectorsApi.md#connectorsStatus) | **GET** /v1/connectors/{tenant_id}/{project_id}/status |  |
| [**connectorsStatusWithHttpInfo**](ConnectorsApi.md#connectorsStatusWithHttpInfo) | **GET** /v1/connectors/{tenant_id}/{project_id}/status |  |



## connectorsConnect

> ConnectionLink connectorsConnect(tenantId, projectId, connectConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        ConnectConnectorRequest connectConnectorRequest = new ConnectConnectorRequest(); // ConnectConnectorRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ConnectionLink result = apiInstance.connectorsConnect(tenantId, projectId, connectConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsConnect");
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
| **connectConnectorRequest** | [**ConnectConnectorRequest**](ConnectConnectorRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ConnectionLink**](ConnectionLink.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | One-time login link to authorize the app |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |

## connectorsConnectWithHttpInfo

> ApiResponse<ConnectionLink> connectorsConnect connectorsConnectWithHttpInfo(tenantId, projectId, connectConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        ConnectConnectorRequest connectConnectorRequest = new ConnectConnectorRequest(); // ConnectConnectorRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ConnectionLink> response = apiInstance.connectorsConnectWithHttpInfo(tenantId, projectId, connectConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsConnect");
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
| **connectConnectorRequest** | [**ConnectConnectorRequest**](ConnectConnectorRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ConnectionLink**](ConnectionLink.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | One-time login link to authorize the app |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |


## connectorsGetSkills

> ConnectorSkillsResponse connectorsGetSkills(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String toolkit = "toolkit_example"; // String | Toolkit slug to scope the request to.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ConnectorSkillsResponse result = apiInstance.connectorsGetSkills(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsGetSkills");
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
| **toolkit** | **String**| Toolkit slug to scope the request to. | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ConnectorSkillsResponse**](ConnectorSkillsResponse.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Generated prompting scaffold (skill cards) for a toolkit |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |

## connectorsGetSkillsWithHttpInfo

> ApiResponse<ConnectorSkillsResponse> connectorsGetSkills connectorsGetSkillsWithHttpInfo(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String toolkit = "toolkit_example"; // String | Toolkit slug to scope the request to.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ConnectorSkillsResponse> response = apiInstance.connectorsGetSkillsWithHttpInfo(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsGetSkills");
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
| **toolkit** | **String**| Toolkit slug to scope the request to. | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ConnectorSkillsResponse**](ConnectorSkillsResponse.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Generated prompting scaffold (skill cards) for a toolkit |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |


## connectorsInvokeTool

> ToolExecution connectorsInvokeTool(tenantId, projectId, invokeConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        InvokeConnectorRequest invokeConnectorRequest = new InvokeConnectorRequest(); // InvokeConnectorRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ToolExecution result = apiInstance.connectorsInvokeTool(tenantId, projectId, invokeConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsInvokeTool");
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
| **invokeConnectorRequest** | [**InvokeConnectorRequest**](InvokeConnectorRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ToolExecution**](ToolExecution.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Execute a connector tool and return its result envelope |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |

## connectorsInvokeToolWithHttpInfo

> ApiResponse<ToolExecution> connectorsInvokeTool connectorsInvokeToolWithHttpInfo(tenantId, projectId, invokeConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        InvokeConnectorRequest invokeConnectorRequest = new InvokeConnectorRequest(); // InvokeConnectorRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ToolExecution> response = apiInstance.connectorsInvokeToolWithHttpInfo(tenantId, projectId, invokeConnectorRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsInvokeTool");
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
| **invokeConnectorRequest** | [**InvokeConnectorRequest**](InvokeConnectorRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ToolExecution**](ToolExecution.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Execute a connector tool and return its result envelope |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |


## connectorsList

> List<Toolkit> connectorsList(tenantId, projectId, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        Integer limit = 56; // Integer | Maximum number of apps to return (page size).
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            List<Toolkit> result = apiInstance.connectorsList(tenantId, projectId, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsList");
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
| **limit** | **Integer**| Maximum number of apps to return (page size). | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**List&lt;Toolkit&gt;**](Toolkit.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List connectable third-party apps (catalog) |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |

## connectorsListWithHttpInfo

> ApiResponse<List<Toolkit>> connectorsList connectorsListWithHttpInfo(tenantId, projectId, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        Integer limit = 56; // Integer | Maximum number of apps to return (page size).
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<List<Toolkit>> response = apiInstance.connectorsListWithHttpInfo(tenantId, projectId, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsList");
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
| **limit** | **Integer**| Maximum number of apps to return (page size). | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**List&lt;Toolkit&gt;**](Toolkit.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List connectable third-party apps (catalog) |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |


## connectorsListTools

> List<ConnectorTool> connectorsListTools(tenantId, projectId, toolkit, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String toolkit = "toolkit_example"; // String | Toolkit slug to list tools for.
        Integer limit = 56; // Integer | Maximum number of tools to return (page size).
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            List<ConnectorTool> result = apiInstance.connectorsListTools(tenantId, projectId, toolkit, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsListTools");
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
| **toolkit** | **String**| Toolkit slug to list tools for. | |
| **limit** | **Integer**| Maximum number of tools to return (page size). | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**List&lt;ConnectorTool&gt;**](ConnectorTool.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List a toolkit&#39;s executable tools with input schemas |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |

## connectorsListToolsWithHttpInfo

> ApiResponse<List<ConnectorTool>> connectorsListTools connectorsListToolsWithHttpInfo(tenantId, projectId, toolkit, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String toolkit = "toolkit_example"; // String | Toolkit slug to list tools for.
        Integer limit = 56; // Integer | Maximum number of tools to return (page size).
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<List<ConnectorTool>> response = apiInstance.connectorsListToolsWithHttpInfo(tenantId, projectId, toolkit, limit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsListTools");
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
| **toolkit** | **String**| Toolkit slug to list tools for. | |
| **limit** | **Integer**| Maximum number of tools to return (page size). | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**List&lt;ConnectorTool&gt;**](ConnectorTool.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List a toolkit&#39;s executable tools with input schemas |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |


## connectorsStatus

> ConnectionStatus connectorsStatus(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String toolkit = "toolkit_example"; // String | Toolkit slug to scope the request to.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ConnectionStatus result = apiInstance.connectorsStatus(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsStatus");
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
| **toolkit** | **String**| Toolkit slug to scope the request to. | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ConnectionStatus**](ConnectionStatus.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Connection status of a toolkit for this project |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |

## connectorsStatusWithHttpInfo

> ApiResponse<ConnectionStatus> connectorsStatus connectorsStatusWithHttpInfo(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.ConnectorsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        ConnectorsApi apiInstance = new ConnectorsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String toolkit = "toolkit_example"; // String | Toolkit slug to scope the request to.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ConnectionStatus> response = apiInstance.connectorsStatusWithHttpInfo(tenantId, projectId, toolkit, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ConnectorsApi#connectorsStatus");
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
| **toolkit** | **String**| Toolkit slug to scope the request to. | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ConnectionStatus**](ConnectionStatus.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Connection status of a toolkit for this project |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **501** | Connector provider not configured |  -  |
