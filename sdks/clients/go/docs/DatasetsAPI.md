# \DatasetsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DatasetsCreate**](DatasetsAPI.md#DatasetsCreate) | **Post** /v1/datasets/{tenant_id}/{project_id} |
[**DatasetsCreateVersion**](DatasetsAPI.md#DatasetsCreateVersion) | **Post** /v1/datasets/{tenant_id}/{project_id}/{dataset_id}/versions |
[**DatasetsPromoteCaseFromTrace**](DatasetsAPI.md#DatasetsPromoteCaseFromTrace) | **Post** /v1/datasets/{tenant_id}/{project_id}/{dataset_id}/cases/from-trace |



## DatasetsCreate

> Dataset DatasetsCreate(ctx, tenantId, projectId).CreateDatasetRequest(createDatasetRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/paletteclient"
)

func main() {
	tenantId := "tenantId_example" // string | tenant_id
	projectId := "projectId_example" // string | project_id
	createDatasetRequest := *openapiclient.NewCreateDatasetRequest("Name_example") // CreateDatasetRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DatasetsAPI.DatasetsCreate(context.Background(), tenantId, projectId).CreateDatasetRequest(createDatasetRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DatasetsAPI.DatasetsCreate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DatasetsCreate`: Dataset
	fmt.Fprintf(os.Stdout, "Response from `DatasetsAPI.DatasetsCreate`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiDatasetsCreateRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **createDatasetRequest** | [**CreateDatasetRequest**](CreateDatasetRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**Dataset**](Dataset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DatasetsCreateVersion

> DatasetVersionSnapshot DatasetsCreateVersion(ctx, tenantId, projectId, datasetId).CreateDatasetVersionRequest(createDatasetVersionRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/paletteclient"
)

func main() {
	tenantId := "tenantId_example" // string | tenant_id
	projectId := "projectId_example" // string | project_id
	datasetId := "datasetId_example" // string | dataset_id
	createDatasetVersionRequest := *openapiclient.NewCreateDatasetVersionRequest() // CreateDatasetVersionRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DatasetsAPI.DatasetsCreateVersion(context.Background(), tenantId, projectId, datasetId).CreateDatasetVersionRequest(createDatasetVersionRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DatasetsAPI.DatasetsCreateVersion``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DatasetsCreateVersion`: DatasetVersionSnapshot
	fmt.Fprintf(os.Stdout, "Response from `DatasetsAPI.DatasetsCreateVersion`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**datasetId** | **string** | dataset_id |

### Other Parameters

Other parameters are passed through a pointer to a apiDatasetsCreateVersionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **createDatasetVersionRequest** | [**CreateDatasetVersionRequest**](CreateDatasetVersionRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**DatasetVersionSnapshot**](DatasetVersionSnapshot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DatasetsPromoteCaseFromTrace

> DatasetCase DatasetsPromoteCaseFromTrace(ctx, tenantId, projectId, datasetId).PromoteTraceCaseRequest(promoteTraceCaseRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/paletteclient"
)

func main() {
	tenantId := "tenantId_example" // string | tenant_id
	projectId := "projectId_example" // string | project_id
	datasetId := "datasetId_example" // string | dataset_id
	promoteTraceCaseRequest := *openapiclient.NewPromoteTraceCaseRequest("TraceId_example") // PromoteTraceCaseRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DatasetsAPI.DatasetsPromoteCaseFromTrace(context.Background(), tenantId, projectId, datasetId).PromoteTraceCaseRequest(promoteTraceCaseRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DatasetsAPI.DatasetsPromoteCaseFromTrace``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DatasetsPromoteCaseFromTrace`: DatasetCase
	fmt.Fprintf(os.Stdout, "Response from `DatasetsAPI.DatasetsPromoteCaseFromTrace`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**datasetId** | **string** | dataset_id |

### Other Parameters

Other parameters are passed through a pointer to a apiDatasetsPromoteCaseFromTraceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **promoteTraceCaseRequest** | [**PromoteTraceCaseRequest**](PromoteTraceCaseRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**DatasetCase**](DatasetCase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
