# HealthApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**healthCheck**](HealthApi.md#healthCheck) | **GET** /health |  |
| [**healthCheckWithHttpInfo**](HealthApi.md#healthCheckWithHttpInfo) | **GET** /health |  |



## healthCheck

> HealthResponse healthCheck()



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.HealthApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        HealthApi apiInstance = new HealthApi(defaultClient);
        try {
            HealthResponse result = apiInstance.healthCheck();
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling HealthApi#healthCheck");
            System.err.println("Status code: " + e.getCode());
            System.err.println("Reason: " + e.getResponseBody());
            System.err.println("Response headers: " + e.getResponseHeaders());
            e.printStackTrace();
        }
    }
}
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**HealthResponse**](HealthResponse.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Runtime is accepting requests |  -  |

## healthCheckWithHttpInfo

> ApiResponse<HealthResponse> healthCheck healthCheckWithHttpInfo()



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.HealthApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        HealthApi apiInstance = new HealthApi(defaultClient);
        try {
            ApiResponse<HealthResponse> response = apiInstance.healthCheckWithHttpInfo();
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling HealthApi#healthCheck");
            System.err.println("Status code: " + e.getCode());
            System.err.println("Response headers: " + e.getResponseHeaders());
            System.err.println("Reason: " + e.getResponseBody());
            e.printStackTrace();
        }
    }
}
```

### Parameters

This endpoint does not need any parameter.

### Return type

ApiResponse<[**HealthResponse**](HealthResponse.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Runtime is accepting requests |  -  |
