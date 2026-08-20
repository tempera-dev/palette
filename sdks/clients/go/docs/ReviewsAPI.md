# \ReviewsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ReviewsCreateQueue**](ReviewsAPI.md#ReviewsCreateQueue) | **Post** /v1/review-queues/{tenantId}/{projectId} |
[**ReviewsEnqueueTaskFromTrace**](ReviewsAPI.md#ReviewsEnqueueTaskFromTrace) | **Post** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/from-trace |
[**ReviewsListTasks**](ReviewsAPI.md#ReviewsListTasks) | **Get** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks |
[**ReviewsPromoteAnnotation**](ReviewsAPI.md#ReviewsPromoteAnnotation) | **Post** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations/{annotationId}/promote |
[**ReviewsSubmitAnnotation**](ReviewsAPI.md#ReviewsSubmitAnnotation) | **Post** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations |



## ReviewsCreateQueue

> ReviewQueue ReviewsCreateQueue(ctx, tenantId, projectId).CreateReviewQueueHttpRequest(createReviewQueueHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	createReviewQueueHttpRequest := *openapiclient.NewCreateReviewQueueHttpRequest(interface{}(123), "Name_example") // CreateReviewQueueHttpRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ReviewsAPI.ReviewsCreateQueue(context.Background(), tenantId, projectId).CreateReviewQueueHttpRequest(createReviewQueueHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ReviewsAPI.ReviewsCreateQueue``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReviewsCreateQueue`: ReviewQueue
	fmt.Fprintf(os.Stdout, "Response from `ReviewsAPI.ReviewsCreateQueue`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiReviewsCreateQueueRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **createReviewQueueHttpRequest** | [**CreateReviewQueueHttpRequest**](CreateReviewQueueHttpRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**ReviewQueue**](ReviewQueue.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ReviewsEnqueueTaskFromTrace

> ReviewTask ReviewsEnqueueTaskFromTrace(ctx, tenantId, projectId, queueId).EnqueueReviewTaskFromTraceHttpRequest(enqueueReviewTaskFromTraceHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	queueId := "queueId_example" // string | queue_id
	enqueueReviewTaskFromTraceHttpRequest := *openapiclient.NewEnqueueReviewTaskFromTraceHttpRequest("TraceId_example") // EnqueueReviewTaskFromTraceHttpRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ReviewsAPI.ReviewsEnqueueTaskFromTrace(context.Background(), tenantId, projectId, queueId).EnqueueReviewTaskFromTraceHttpRequest(enqueueReviewTaskFromTraceHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ReviewsAPI.ReviewsEnqueueTaskFromTrace``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReviewsEnqueueTaskFromTrace`: ReviewTask
	fmt.Fprintf(os.Stdout, "Response from `ReviewsAPI.ReviewsEnqueueTaskFromTrace`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**queueId** | **string** | queue_id |

### Other Parameters

Other parameters are passed through a pointer to a apiReviewsEnqueueTaskFromTraceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **enqueueReviewTaskFromTraceHttpRequest** | [**EnqueueReviewTaskFromTraceHttpRequest**](EnqueueReviewTaskFromTraceHttpRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**ReviewTask**](ReviewTask.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ReviewsListTasks

> ReviewTaskListResponse ReviewsListTasks(ctx, tenantId, projectId, queueId).State(state).PageSize(pageSize).PageToken(pageToken).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	queueId := "queueId_example" // string | queue_id
	state := openapiclient.ReviewTaskState("open") // ReviewTaskState |  (optional)
	pageSize := int32(56) // int32 | Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum. (optional)
	pageToken := "pageToken_example" // string | Opaque continuation token returned by the preceding list request. (optional)
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ReviewsAPI.ReviewsListTasks(context.Background(), tenantId, projectId, queueId).State(state).PageSize(pageSize).PageToken(pageToken).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ReviewsAPI.ReviewsListTasks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReviewsListTasks`: ReviewTaskListResponse
	fmt.Fprintf(os.Stdout, "Response from `ReviewsAPI.ReviewsListTasks`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**queueId** | **string** | queue_id |

### Other Parameters

Other parameters are passed through a pointer to a apiReviewsListTasksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **state** | [**ReviewTaskState**](ReviewTaskState.md) |  |
 **pageSize** | **int32** | Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum. |
 **pageToken** | **string** | Opaque continuation token returned by the preceding list request. |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**ReviewTaskListResponse**](ReviewTaskListResponse.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ReviewsPromoteAnnotation

> DatasetCase ReviewsPromoteAnnotation(ctx, tenantId, projectId, queueId, taskId, annotationId).PromoteReviewAnnotationHttpRequest(promoteReviewAnnotationHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	queueId := "queueId_example" // string | queue_id
	taskId := "taskId_example" // string | task_id
	annotationId := "annotationId_example" // string | annotation_id
	promoteReviewAnnotationHttpRequest := *openapiclient.NewPromoteReviewAnnotationHttpRequest("DatasetId_example") // PromoteReviewAnnotationHttpRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ReviewsAPI.ReviewsPromoteAnnotation(context.Background(), tenantId, projectId, queueId, taskId, annotationId).PromoteReviewAnnotationHttpRequest(promoteReviewAnnotationHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ReviewsAPI.ReviewsPromoteAnnotation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReviewsPromoteAnnotation`: DatasetCase
	fmt.Fprintf(os.Stdout, "Response from `ReviewsAPI.ReviewsPromoteAnnotation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**queueId** | **string** | queue_id |
**taskId** | **string** | task_id |
**annotationId** | **string** | annotation_id |

### Other Parameters

Other parameters are passed through a pointer to a apiReviewsPromoteAnnotationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------





 **promoteReviewAnnotationHttpRequest** | [**PromoteReviewAnnotationHttpRequest**](PromoteReviewAnnotationHttpRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**DatasetCase**](DatasetCase.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ReviewsSubmitAnnotation

> ReviewAnnotation ReviewsSubmitAnnotation(ctx, tenantId, projectId, queueId, taskId).SubmitReviewAnnotationHttpRequest(submitReviewAnnotationHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	queueId := "queueId_example" // string | queue_id
	taskId := "taskId_example" // string | task_id
	submitReviewAnnotationHttpRequest := *openapiclient.NewSubmitReviewAnnotationHttpRequest(interface{}(123), "ReviewerId_example", openapiclient.ReviewVerdict("pass")) // SubmitReviewAnnotationHttpRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ReviewsAPI.ReviewsSubmitAnnotation(context.Background(), tenantId, projectId, queueId, taskId).SubmitReviewAnnotationHttpRequest(submitReviewAnnotationHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ReviewsAPI.ReviewsSubmitAnnotation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReviewsSubmitAnnotation`: ReviewAnnotation
	fmt.Fprintf(os.Stdout, "Response from `ReviewsAPI.ReviewsSubmitAnnotation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**queueId** | **string** | queue_id |
**taskId** | **string** | task_id |

### Other Parameters

Other parameters are passed through a pointer to a apiReviewsSubmitAnnotationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------




 **submitReviewAnnotationHttpRequest** | [**SubmitReviewAnnotationHttpRequest**](SubmitReviewAnnotationHttpRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**ReviewAnnotation**](ReviewAnnotation.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
