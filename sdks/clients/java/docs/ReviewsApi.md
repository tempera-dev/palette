# ReviewsApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**reviewsCreateQueue**](ReviewsApi.md#reviewsCreateQueue) | **POST** /v1/review-queues/{tenantId}/{projectId} |  |
| [**reviewsCreateQueueWithHttpInfo**](ReviewsApi.md#reviewsCreateQueueWithHttpInfo) | **POST** /v1/review-queues/{tenantId}/{projectId} |  |
| [**reviewsEnqueueTaskFromTrace**](ReviewsApi.md#reviewsEnqueueTaskFromTrace) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/from-trace |  |
| [**reviewsEnqueueTaskFromTraceWithHttpInfo**](ReviewsApi.md#reviewsEnqueueTaskFromTraceWithHttpInfo) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/from-trace |  |
| [**reviewsListTasks**](ReviewsApi.md#reviewsListTasks) | **GET** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks |  |
| [**reviewsListTasksWithHttpInfo**](ReviewsApi.md#reviewsListTasksWithHttpInfo) | **GET** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks |  |
| [**reviewsPromoteAnnotation**](ReviewsApi.md#reviewsPromoteAnnotation) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations/{annotationId}/promote |  |
| [**reviewsPromoteAnnotationWithHttpInfo**](ReviewsApi.md#reviewsPromoteAnnotationWithHttpInfo) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations/{annotationId}/promote |  |
| [**reviewsSubmitAnnotation**](ReviewsApi.md#reviewsSubmitAnnotation) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations |  |
| [**reviewsSubmitAnnotationWithHttpInfo**](ReviewsApi.md#reviewsSubmitAnnotationWithHttpInfo) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations |  |



## reviewsCreateQueue

> ReviewQueue reviewsCreateQueue(tenantId, projectId, createReviewQueueHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateReviewQueueHttpRequest createReviewQueueHttpRequest = new CreateReviewQueueHttpRequest(); // CreateReviewQueueHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ReviewQueue result = apiInstance.reviewsCreateQueue(tenantId, projectId, createReviewQueueHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsCreateQueue");
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
| **createReviewQueueHttpRequest** | [**CreateReviewQueueHttpRequest**](CreateReviewQueueHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ReviewQueue**](ReviewQueue.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a human review queue |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |

## reviewsCreateQueueWithHttpInfo

> ApiResponse<ReviewQueue> reviewsCreateQueue reviewsCreateQueueWithHttpInfo(tenantId, projectId, createReviewQueueHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateReviewQueueHttpRequest createReviewQueueHttpRequest = new CreateReviewQueueHttpRequest(); // CreateReviewQueueHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ReviewQueue> response = apiInstance.reviewsCreateQueueWithHttpInfo(tenantId, projectId, createReviewQueueHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsCreateQueue");
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
| **createReviewQueueHttpRequest** | [**CreateReviewQueueHttpRequest**](CreateReviewQueueHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ReviewQueue**](ReviewQueue.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a human review queue |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |


## reviewsEnqueueTaskFromTrace

> ReviewTask reviewsEnqueueTaskFromTrace(tenantId, projectId, queueId, enqueueReviewTaskFromTraceHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        EnqueueReviewTaskFromTraceHttpRequest enqueueReviewTaskFromTraceHttpRequest = new EnqueueReviewTaskFromTraceHttpRequest(); // EnqueueReviewTaskFromTraceHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ReviewTask result = apiInstance.reviewsEnqueueTaskFromTrace(tenantId, projectId, queueId, enqueueReviewTaskFromTraceHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsEnqueueTaskFromTrace");
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
| **queueId** | **String**| queue_id | |
| **enqueueReviewTaskFromTraceHttpRequest** | [**EnqueueReviewTaskFromTraceHttpRequest**](EnqueueReviewTaskFromTraceHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ReviewTask**](ReviewTask.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Enqueue a review task from a trace |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## reviewsEnqueueTaskFromTraceWithHttpInfo

> ApiResponse<ReviewTask> reviewsEnqueueTaskFromTrace reviewsEnqueueTaskFromTraceWithHttpInfo(tenantId, projectId, queueId, enqueueReviewTaskFromTraceHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        EnqueueReviewTaskFromTraceHttpRequest enqueueReviewTaskFromTraceHttpRequest = new EnqueueReviewTaskFromTraceHttpRequest(); // EnqueueReviewTaskFromTraceHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ReviewTask> response = apiInstance.reviewsEnqueueTaskFromTraceWithHttpInfo(tenantId, projectId, queueId, enqueueReviewTaskFromTraceHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsEnqueueTaskFromTrace");
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
| **queueId** | **String**| queue_id | |
| **enqueueReviewTaskFromTraceHttpRequest** | [**EnqueueReviewTaskFromTraceHttpRequest**](EnqueueReviewTaskFromTraceHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ReviewTask**](ReviewTask.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Enqueue a review task from a trace |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |


## reviewsListTasks

> ReviewTaskListResponse reviewsListTasks(tenantId, projectId, queueId, state, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        ReviewTaskState state = ReviewTaskState.fromValue("open"); // ReviewTaskState |
        Integer pageSize = 56; // Integer | Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum.
        String pageToken = "pageToken_example"; // String | Opaque continuation token returned by the preceding list request.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ReviewTaskListResponse result = apiInstance.reviewsListTasks(tenantId, projectId, queueId, state, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsListTasks");
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
| **queueId** | **String**| queue_id | |
| **state** | [**ReviewTaskState**](.md)|  | [optional] [enum: open, submitted, cancelled] |
| **pageSize** | **Integer**| Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional] |
| **pageToken** | **String**| Opaque continuation token returned by the preceding list request. | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ReviewTaskListResponse**](ReviewTaskListResponse.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List review tasks |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## reviewsListTasksWithHttpInfo

> ApiResponse<ReviewTaskListResponse> reviewsListTasks reviewsListTasksWithHttpInfo(tenantId, projectId, queueId, state, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        ReviewTaskState state = ReviewTaskState.fromValue("open"); // ReviewTaskState |
        Integer pageSize = 56; // Integer | Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum.
        String pageToken = "pageToken_example"; // String | Opaque continuation token returned by the preceding list request.
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ReviewTaskListResponse> response = apiInstance.reviewsListTasksWithHttpInfo(tenantId, projectId, queueId, state, pageSize, pageToken, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsListTasks");
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
| **queueId** | **String**| queue_id | |
| **state** | [**ReviewTaskState**](.md)|  | [optional] [enum: open, submitted, cancelled] |
| **pageSize** | **Integer**| Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional] |
| **pageToken** | **String**| Opaque continuation token returned by the preceding list request. | [optional] |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ReviewTaskListResponse**](ReviewTaskListResponse.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | List review tasks |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |


## reviewsPromoteAnnotation

> DatasetCase reviewsPromoteAnnotation(tenantId, projectId, queueId, taskId, annotationId, promoteReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        String taskId = "taskId_example"; // String | task_id
        String annotationId = "annotationId_example"; // String | annotation_id
        PromoteReviewAnnotationHttpRequest promoteReviewAnnotationHttpRequest = new PromoteReviewAnnotationHttpRequest(); // PromoteReviewAnnotationHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            DatasetCase result = apiInstance.reviewsPromoteAnnotation(tenantId, projectId, queueId, taskId, annotationId, promoteReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsPromoteAnnotation");
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
| **queueId** | **String**| queue_id | |
| **taskId** | **String**| task_id | |
| **annotationId** | **String**| annotation_id | |
| **promoteReviewAnnotationHttpRequest** | [**PromoteReviewAnnotationHttpRequest**](PromoteReviewAnnotationHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**DatasetCase**](DatasetCase.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Promote a review annotation to a dataset case |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## reviewsPromoteAnnotationWithHttpInfo

> ApiResponse<DatasetCase> reviewsPromoteAnnotation reviewsPromoteAnnotationWithHttpInfo(tenantId, projectId, queueId, taskId, annotationId, promoteReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        String taskId = "taskId_example"; // String | task_id
        String annotationId = "annotationId_example"; // String | annotation_id
        PromoteReviewAnnotationHttpRequest promoteReviewAnnotationHttpRequest = new PromoteReviewAnnotationHttpRequest(); // PromoteReviewAnnotationHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<DatasetCase> response = apiInstance.reviewsPromoteAnnotationWithHttpInfo(tenantId, projectId, queueId, taskId, annotationId, promoteReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsPromoteAnnotation");
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
| **queueId** | **String**| queue_id | |
| **taskId** | **String**| task_id | |
| **annotationId** | **String**| annotation_id | |
| **promoteReviewAnnotationHttpRequest** | [**PromoteReviewAnnotationHttpRequest**](PromoteReviewAnnotationHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**DatasetCase**](DatasetCase.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Promote a review annotation to a dataset case |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |


## reviewsSubmitAnnotation

> ReviewAnnotation reviewsSubmitAnnotation(tenantId, projectId, queueId, taskId, submitReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        String taskId = "taskId_example"; // String | task_id
        SubmitReviewAnnotationHttpRequest submitReviewAnnotationHttpRequest = new SubmitReviewAnnotationHttpRequest(); // SubmitReviewAnnotationHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ReviewAnnotation result = apiInstance.reviewsSubmitAnnotation(tenantId, projectId, queueId, taskId, submitReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsSubmitAnnotation");
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
| **queueId** | **String**| queue_id | |
| **taskId** | **String**| task_id | |
| **submitReviewAnnotationHttpRequest** | [**SubmitReviewAnnotationHttpRequest**](SubmitReviewAnnotationHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**ReviewAnnotation**](ReviewAnnotation.md)


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Submit a review annotation |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |

## reviewsSubmitAnnotationWithHttpInfo

> ApiResponse<ReviewAnnotation> reviewsSubmitAnnotation reviewsSubmitAnnotationWithHttpInfo(tenantId, projectId, queueId, taskId, submitReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.auth.*;
import ai.palette.client.models.*;
import ai.palette.client.api.ReviewsApi;

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

        ReviewsApi apiInstance = new ReviewsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String queueId = "queueId_example"; // String | queue_id
        String taskId = "taskId_example"; // String | task_id
        SubmitReviewAnnotationHttpRequest submitReviewAnnotationHttpRequest = new SubmitReviewAnnotationHttpRequest(); // SubmitReviewAnnotationHttpRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<ReviewAnnotation> response = apiInstance.reviewsSubmitAnnotationWithHttpInfo(tenantId, projectId, queueId, taskId, submitReviewAnnotationHttpRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling ReviewsApi#reviewsSubmitAnnotation");
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
| **queueId** | **String**| queue_id | |
| **taskId** | **String**| task_id | |
| **submitReviewAnnotationHttpRequest** | [**SubmitReviewAnnotationHttpRequest**](SubmitReviewAnnotationHttpRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**ReviewAnnotation**](ReviewAnnotation.md)>


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Submit a review annotation |  -  |
| **400** | Invalid request, scope, or filter |  -  |
| **401** | Missing or invalid credentials |  -  |
| **403** | Credentials lack the required scope |  -  |
| **404** | Resource not found |  -  |
