# JudgeApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**judgeEvaluate**](JudgeApi.md#judgeEvaluate) | **POST** /v1/judge/{tenantId}/{projectId}/evaluate |  |
| [**judgeEvaluateWithHttpInfo**](JudgeApi.md#judgeEvaluateWithHttpInfo) | **POST** /v1/judge/{tenantId}/{projectId}/evaluate |  |
| [**judgeListLedger**](JudgeApi.md#judgeListLedger) | **GET** /v1/judge/{tenantId}/{projectId}/ledger |  |
| [**judgeListLedgerWithHttpInfo**](JudgeApi.md#judgeListLedgerWithHttpInfo) | **GET** /v1/judge/{tenantId}/{projectId}/ledger |  |



## judgeEvaluate

> JudgeBrokerOutcome judgeEvaluate(tenantId, projectId, runJudgeEvalHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.JudgeApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        JudgeApi apiInstance = new JudgeApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        RunJudgeEvalHttpRequest runJudgeEvalHttpRequest = new RunJudgeEvalHttpRequest(); // RunJudgeEvalHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            JudgeBrokerOutcome result = apiInstance.judgeEvaluate(tenantId, projectId, runJudgeEvalHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling JudgeApi#judgeEvaluate");
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
| **runJudgeEvalHttpRequest** | [**RunJudgeEvalHttpRequest**](RunJudgeEvalHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**JudgeBrokerOutcome**](JudgeBrokerOutcome.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Run an ad-hoc judge evaluation |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |

## judgeEvaluateWithHttpInfo

> ApiResponse<JudgeBrokerOutcome> judgeEvaluate judgeEvaluateWithHttpInfo(tenantId, projectId, runJudgeEvalHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.JudgeApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        JudgeApi apiInstance = new JudgeApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        RunJudgeEvalHttpRequest runJudgeEvalHttpRequest = new RunJudgeEvalHttpRequest(); // RunJudgeEvalHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<JudgeBrokerOutcome> response = apiInstance.judgeEvaluateWithHttpInfo(tenantId, projectId, runJudgeEvalHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling JudgeApi#judgeEvaluate");
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
| **runJudgeEvalHttpRequest** | [**RunJudgeEvalHttpRequest**](RunJudgeEvalHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**JudgeBrokerOutcome**](JudgeBrokerOutcome.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Run an ad-hoc judge evaluation |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |


## judgeListLedger

> JudgeLedgerListResponse judgeListLedger(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.JudgeApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        JudgeApi apiInstance = new JudgeApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        Integer pageSize = 56; // Integer | Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum.
        String pageToken = "pageToken_example"; // String | Opaque continuation token returned by the preceding list request.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            JudgeLedgerListResponse result = apiInstance.judgeListLedger(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling JudgeApi#judgeListLedger");
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
| **pageSize** | **Integer**| Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional] |
| **pageToken** | **String**| Opaque continuation token returned by the preceding list request. | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**JudgeLedgerListResponse**](JudgeLedgerListResponse.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List judge ledger audit records |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |

## judgeListLedgerWithHttpInfo

> ApiResponse<JudgeLedgerListResponse> judgeListLedger judgeListLedgerWithHttpInfo(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.JudgeApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        JudgeApi apiInstance = new JudgeApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        Integer pageSize = 56; // Integer | Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum.
        String pageToken = "pageToken_example"; // String | Opaque continuation token returned by the preceding list request.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<JudgeLedgerListResponse> response = apiInstance.judgeListLedgerWithHttpInfo(tenantId, projectId, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling JudgeApi#judgeListLedger");
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
| **pageSize** | **Integer**| Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional] |
| **pageToken** | **String**| Opaque continuation token returned by the preceding list request. | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**JudgeLedgerListResponse**](JudgeLedgerListResponse.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List judge ledger audit records |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |
