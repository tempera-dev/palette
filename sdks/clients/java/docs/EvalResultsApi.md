# EvalResultsApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**evalResultsGetTemperaEvidence**](EvalResultsApi.md#evalResultsGetTemperaEvidence) | **GET** /v1/eval-results/{tenant_id}/{project_id}/tempera/{kind}/{external_id} |  |
| [**evalResultsGetTemperaEvidenceWithHttpInfo**](EvalResultsApi.md#evalResultsGetTemperaEvidenceWithHttpInfo) | **GET** /v1/eval-results/{tenant_id}/{project_id}/tempera/{kind}/{external_id} |  |
| [**evalResultsImportTemperaBundle**](EvalResultsApi.md#evalResultsImportTemperaBundle) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/bundles |  |
| [**evalResultsImportTemperaBundleWithHttpInfo**](EvalResultsApi.md#evalResultsImportTemperaBundleWithHttpInfo) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/bundles |  |
| [**evalResultsRecordTemperaDecision**](EvalResultsApi.md#evalResultsRecordTemperaDecision) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/decisions |  |
| [**evalResultsRecordTemperaDecisionWithHttpInfo**](EvalResultsApi.md#evalResultsRecordTemperaDecisionWithHttpInfo) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/decisions |  |



## evalResultsGetTemperaEvidence

> TemperaEvidenceReceipt evalResultsGetTemperaEvidence(tenantId, projectId, kind, externalId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.EvalResultsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        EvalResultsApi apiInstance = new EvalResultsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String kind = "kind_example"; // String | result_bundle or ab_decision
        String externalId = "externalId_example"; // String | Bundle or experiment id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            TemperaEvidenceReceipt result = apiInstance.evalResultsGetTemperaEvidence(tenantId, projectId, kind, externalId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling EvalResultsApi#evalResultsGetTemperaEvidence");
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
| **kind** | **String**| result_bundle or ab_decision | |
| **externalId** | **String**| Bundle or experiment id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Read a scoped external evidence receipt |  -  |
| **400** | Invalid evidence kind or identifier |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Evidence not found in this tenant/project |  -  |

## evalResultsGetTemperaEvidenceWithHttpInfo

> ApiResponse<TemperaEvidenceReceipt> evalResultsGetTemperaEvidence evalResultsGetTemperaEvidenceWithHttpInfo(tenantId, projectId, kind, externalId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.EvalResultsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        EvalResultsApi apiInstance = new EvalResultsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String kind = "kind_example"; // String | result_bundle or ab_decision
        String externalId = "externalId_example"; // String | Bundle or experiment id
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<TemperaEvidenceReceipt> response = apiInstance.evalResultsGetTemperaEvidenceWithHttpInfo(tenantId, projectId, kind, externalId, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling EvalResultsApi#evalResultsGetTemperaEvidence");
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
| **kind** | **String**| result_bundle or ab_decision | |
| **externalId** | **String**| Bundle or experiment id | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Read a scoped external evidence receipt |  -  |
| **400** | Invalid evidence kind or identifier |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Evidence not found in this tenant/project |  -  |


## evalResultsImportTemperaBundle

> TemperaEvidenceReceipt evalResultsImportTemperaBundle(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.EvalResultsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        EvalResultsApi apiInstance = new EvalResultsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        ImportTemperaEvidenceRequest importTemperaEvidenceRequest = new ImportTemperaEvidenceRequest(); // ImportTemperaEvidenceRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            TemperaEvidenceReceipt result = apiInstance.evalResultsImportTemperaBundle(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling EvalResultsApi#evalResultsImportTemperaBundle");
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
| **importTemperaEvidenceRequest** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Idempotently store a verified official Tempera result bundle |  -  |
| **400** | Malformed, non-canonical, unsafe, or signature-invalid evidence |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope or the evidence key is not trusted |  -  |
| **409** | The external id already binds different content |  -  |
| **413** | Evidence exceeds the request limit |  -  |
| **422** | Request body does not match the schema |  -  |
| **503** | No Tempera evaluation release-key trust anchor is configured |  -  |

## evalResultsImportTemperaBundleWithHttpInfo

> ApiResponse<TemperaEvidenceReceipt> evalResultsImportTemperaBundle evalResultsImportTemperaBundleWithHttpInfo(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.EvalResultsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        EvalResultsApi apiInstance = new EvalResultsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        ImportTemperaEvidenceRequest importTemperaEvidenceRequest = new ImportTemperaEvidenceRequest(); // ImportTemperaEvidenceRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<TemperaEvidenceReceipt> response = apiInstance.evalResultsImportTemperaBundleWithHttpInfo(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling EvalResultsApi#evalResultsImportTemperaBundle");
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
| **importTemperaEvidenceRequest** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Idempotently store a verified official Tempera result bundle |  -  |
| **400** | Malformed, non-canonical, unsafe, or signature-invalid evidence |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope or the evidence key is not trusted |  -  |
| **409** | The external id already binds different content |  -  |
| **413** | Evidence exceeds the request limit |  -  |
| **422** | Request body does not match the schema |  -  |
| **503** | No Tempera evaluation release-key trust anchor is configured |  -  |


## evalResultsRecordTemperaDecision

> TemperaEvidenceReceipt evalResultsRecordTemperaDecision(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.EvalResultsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        EvalResultsApi apiInstance = new EvalResultsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        ImportTemperaEvidenceRequest importTemperaEvidenceRequest = new ImportTemperaEvidenceRequest(); // ImportTemperaEvidenceRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            TemperaEvidenceReceipt result = apiInstance.evalResultsRecordTemperaDecision(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling EvalResultsApi#evalResultsRecordTemperaDecision");
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
| **importTemperaEvidenceRequest** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Idempotently store a verified preregistered Tempera A/B decision |  -  |
| **400** | Malformed, non-canonical, unsafe, or signature-invalid evidence |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope or the evidence key is not trusted |  -  |
| **409** | The external id already binds different content |  -  |
| **413** | Evidence exceeds the request limit |  -  |
| **422** | Request body does not match the schema |  -  |
| **503** | No Tempera evaluation release-key trust anchor is configured |  -  |

## evalResultsRecordTemperaDecisionWithHttpInfo

> ApiResponse<TemperaEvidenceReceipt> evalResultsRecordTemperaDecision evalResultsRecordTemperaDecisionWithHttpInfo(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.EvalResultsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        EvalResultsApi apiInstance = new EvalResultsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        ImportTemperaEvidenceRequest importTemperaEvidenceRequest = new ImportTemperaEvidenceRequest(); // ImportTemperaEvidenceRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<TemperaEvidenceReceipt> response = apiInstance.evalResultsRecordTemperaDecisionWithHttpInfo(tenantId, projectId, importTemperaEvidenceRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling EvalResultsApi#evalResultsRecordTemperaDecision");
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
| **importTemperaEvidenceRequest** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Idempotently store a verified preregistered Tempera A/B decision |  -  |
| **400** | Malformed, non-canonical, unsafe, or signature-invalid evidence |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope or the evidence key is not trusted |  -  |
| **409** | The external id already binds different content |  -  |
| **413** | Evidence exceeds the request limit |  -  |
| **422** | Request body does not match the schema |  -  |
| **503** | No Tempera evaluation release-key trust anchor is configured |  -  |
