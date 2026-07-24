# \SearchAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**SearchSpans**](SearchAPI.md#SearchSpans) | **Get** /v1/search/{tenantId}/spans |



## SearchSpans

> SearchSpanListResponse SearchSpans(ctx, tenantId).Q(q).ProjectId(projectId).EnvironmentId(environmentId).TraceId(traceId).SpanId(spanId).Kind(kind).Status(status).Model(model).Tool(tool).PageSize(pageSize).PageToken(pageToken).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	q := "q_example" // string |  (optional)
	projectId := "projectId_example" // string |  (optional)
	environmentId := "environmentId_example" // string |  (optional)
	traceId := "traceId_example" // string |  (optional)
	spanId := "spanId_example" // string |  (optional)
	kind := "kind_example" // string |  (optional)
	status := "status_example" // string |  (optional)
	model := "model_example" // string |  (optional)
	tool := "tool_example" // string |  (optional)
	pageSize := int32(56) // int32 |  (optional)
	pageToken := "pageToken_example" // string |  (optional)
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SearchAPI.SearchSpans(context.Background(), tenantId).Q(q).ProjectId(projectId).EnvironmentId(environmentId).TraceId(traceId).SpanId(spanId).Kind(kind).Status(status).Model(model).Tool(tool).PageSize(pageSize).PageToken(pageToken).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SearchAPI.SearchSpans``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SearchSpans`: SearchSpanListResponse
	fmt.Fprintf(os.Stdout, "Response from `SearchAPI.SearchSpans`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |

### Other Parameters

Other parameters are passed through a pointer to a apiSearchSpansRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **q** | **string** |  |
 **projectId** | **string** |  |
 **environmentId** | **string** |  |
 **traceId** | **string** |  |
 **spanId** | **string** |  |
 **kind** | **string** |  |
 **status** | **string** |  |
 **model** | **string** |  |
 **tool** | **string** |  |
 **pageSize** | **int32** |  |
 **pageToken** | **string** |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**SearchSpanListResponse**](SearchSpanListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
