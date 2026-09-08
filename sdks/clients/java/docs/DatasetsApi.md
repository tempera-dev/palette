# DatasetsApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**datasetsCreate**](DatasetsApi.md#datasetsCreate) | **POST** /v1/datasets/{tenantId}/{projectId} |  |
| [**datasetsCreateWithHttpInfo**](DatasetsApi.md#datasetsCreateWithHttpInfo) | **POST** /v1/datasets/{tenantId}/{projectId} |  |
| [**datasetsCreateVersion**](DatasetsApi.md#datasetsCreateVersion) | **POST** /v1/datasets/{tenantId}/{projectId}/{datasetId}/versions |  |
| [**datasetsCreateVersionWithHttpInfo**](DatasetsApi.md#datasetsCreateVersionWithHttpInfo) | **POST** /v1/datasets/{tenantId}/{projectId}/{datasetId}/versions |  |
| [**datasetsPromoteCaseFromTrace**](DatasetsApi.md#datasetsPromoteCaseFromTrace) | **POST** /v1/datasets/{tenantId}/{projectId}/{datasetId}/cases/from-trace |  |
| [**datasetsPromoteCaseFromTraceWithHttpInfo**](DatasetsApi.md#datasetsPromoteCaseFromTraceWithHttpInfo) | **POST** /v1/datasets/{tenantId}/{projectId}/{datasetId}/cases/from-trace |  |



## datasetsCreate

> Dataset datasetsCreate(tenantId, projectId, createDatasetRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.DatasetsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        DatasetsApi apiInstance = new DatasetsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateDatasetRequest createDatasetRequest = new CreateDatasetRequest(); // CreateDatasetRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            Dataset result = apiInstance.datasetsCreate(tenantId, projectId, createDatasetRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling DatasetsApi#datasetsCreate");
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
| **createDatasetRequest** | [**CreateDatasetRequest**](CreateDatasetRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**Dataset**](Dataset.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a dataset |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |

## datasetsCreateWithHttpInfo

> ApiResponse<Dataset> datasetsCreate datasetsCreateWithHttpInfo(tenantId, projectId, createDatasetRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.DatasetsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        DatasetsApi apiInstance = new DatasetsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        CreateDatasetRequest createDatasetRequest = new CreateDatasetRequest(); // CreateDatasetRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<Dataset> response = apiInstance.datasetsCreateWithHttpInfo(tenantId, projectId, createDatasetRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling DatasetsApi#datasetsCreate");
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
| **createDatasetRequest** | [**CreateDatasetRequest**](CreateDatasetRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**Dataset**](Dataset.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a dataset |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |


## datasetsCreateVersion

> DatasetVersionSnapshot datasetsCreateVersion(tenantId, projectId, datasetId, createDatasetVersionRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.DatasetsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        DatasetsApi apiInstance = new DatasetsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String datasetId = "datasetId_example"; // String | dataset_id
        CreateDatasetVersionRequest createDatasetVersionRequest = new CreateDatasetVersionRequest(); // CreateDatasetVersionRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            DatasetVersionSnapshot result = apiInstance.datasetsCreateVersion(tenantId, projectId, datasetId, createDatasetVersionRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling DatasetsApi#datasetsCreateVersion");
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
| **datasetId** | **String**| dataset_id | |
| **createDatasetVersionRequest** | [**CreateDatasetVersionRequest**](CreateDatasetVersionRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**DatasetVersionSnapshot**](DatasetVersionSnapshot.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a dataset version snapshot |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **404** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |

## datasetsCreateVersionWithHttpInfo

> ApiResponse<DatasetVersionSnapshot> datasetsCreateVersion datasetsCreateVersionWithHttpInfo(tenantId, projectId, datasetId, createDatasetVersionRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.DatasetsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        DatasetsApi apiInstance = new DatasetsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String datasetId = "datasetId_example"; // String | dataset_id
        CreateDatasetVersionRequest createDatasetVersionRequest = new CreateDatasetVersionRequest(); // CreateDatasetVersionRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<DatasetVersionSnapshot> response = apiInstance.datasetsCreateVersionWithHttpInfo(tenantId, projectId, datasetId, createDatasetVersionRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling DatasetsApi#datasetsCreateVersion");
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
| **datasetId** | **String**| dataset_id | |
| **createDatasetVersionRequest** | [**CreateDatasetVersionRequest**](CreateDatasetVersionRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**DatasetVersionSnapshot**](DatasetVersionSnapshot.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a dataset version snapshot |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **404** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |


## datasetsPromoteCaseFromTrace

> DatasetCase datasetsPromoteCaseFromTrace(tenantId, projectId, datasetId, promoteTraceCaseRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.DatasetsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        DatasetsApi apiInstance = new DatasetsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String datasetId = "datasetId_example"; // String | dataset_id
        PromoteTraceCaseRequest promoteTraceCaseRequest = new PromoteTraceCaseRequest(); // PromoteTraceCaseRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            DatasetCase result = apiInstance.datasetsPromoteCaseFromTrace(tenantId, projectId, datasetId, promoteTraceCaseRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling DatasetsApi#datasetsPromoteCaseFromTrace");
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
| **datasetId** | **String**| dataset_id | |
| **promoteTraceCaseRequest** | [**PromoteTraceCaseRequest**](PromoteTraceCaseRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

[**DatasetCase**](DatasetCase.md)


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Promote a trace span to a dataset case |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **404** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |

## datasetsPromoteCaseFromTraceWithHttpInfo

> ApiResponse<DatasetCase> datasetsPromoteCaseFromTrace datasetsPromoteCaseFromTraceWithHttpInfo(tenantId, projectId, datasetId, promoteTraceCaseRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId)



### Example

```java
// Import classes:
import ai.palette.client.ApiClient;
import ai.palette.client.ApiException;
import ai.palette.client.ApiResponse;
import ai.palette.client.Configuration;
import ai.palette.client.models.*;
import ai.palette.client.api.DatasetsApi;

public class Example {
    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost");

        DatasetsApi apiInstance = new DatasetsApi(defaultClient);
        String tenantId = "tenantId_example"; // String | tenant_id
        String projectId = "projectId_example"; // String | project_id
        String datasetId = "datasetId_example"; // String | dataset_id
        PromoteTraceCaseRequest promoteTraceCaseRequest = new PromoteTraceCaseRequest(); // PromoteTraceCaseRequest |
        String authorization = "authorization_example"; // String | Bearer API token for strict auth
        String xPaletteApiKey = "xPaletteApiKey_example"; // String | API key alternative for strict auth
        String xPaletteProjectId = "xPaletteProjectId_example"; // String | Strict-auth project scope
        String xPaletteEnvironmentId = "xPaletteEnvironmentId_example"; // String | Strict-auth environment scope
        try {
            ApiResponse<DatasetCase> response = apiInstance.datasetsPromoteCaseFromTraceWithHttpInfo(tenantId, projectId, datasetId, promoteTraceCaseRequest, authorization, xPaletteApiKey, xPaletteProjectId, xPaletteEnvironmentId);
            System.out.println("Status code: " + response.getStatusCode());
            System.out.println("Response headers: " + response.getHeaders());
            System.out.println("Response body: " + response.getData());
        } catch (ApiException e) {
            System.err.println("Exception when calling DatasetsApi#datasetsPromoteCaseFromTrace");
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
| **datasetId** | **String**| dataset_id | |
| **promoteTraceCaseRequest** | [**PromoteTraceCaseRequest**](PromoteTraceCaseRequest.md)|  | |
| **authorization** | **String**| Bearer API token for strict auth | [optional] |
| **xPaletteApiKey** | **String**| API key alternative for strict auth | [optional] |
| **xPaletteProjectId** | **String**| Strict-auth project scope | [optional] |
| **xPaletteEnvironmentId** | **String**| Strict-auth environment scope | [optional] |

### Return type

ApiResponse<[**DatasetCase**](DatasetCase.md)>


### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Promote a trace span to a dataset case |  -  |
| **400** | A google.rpc.Status error envelope. |  -  |
| **401** | A google.rpc.Status error envelope. |  -  |
| **403** | A google.rpc.Status error envelope. |  -  |
| **404** | A google.rpc.Status error envelope. |  -  |
| **0** | A google.rpc.Status error envelope. |  -  |
