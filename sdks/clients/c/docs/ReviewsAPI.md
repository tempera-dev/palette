# ReviewsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ReviewsAPI_reviewsCreateQueue**](ReviewsAPI.md#ReviewsAPI_reviewsCreateQueue) | **POST** /v1/review-queues/{tenantId}/{projectId} |
[**ReviewsAPI_reviewsEnqueueTaskFromTrace**](ReviewsAPI.md#ReviewsAPI_reviewsEnqueueTaskFromTrace) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/from-trace |
[**ReviewsAPI_reviewsListTasks**](ReviewsAPI.md#ReviewsAPI_reviewsListTasks) | **GET** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks |
[**ReviewsAPI_reviewsPromoteAnnotation**](ReviewsAPI.md#ReviewsAPI_reviewsPromoteAnnotation) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations/{annotationId}/promote |
[**ReviewsAPI_reviewsSubmitAnnotation**](ReviewsAPI.md#ReviewsAPI_reviewsSubmitAnnotation) | **POST** /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations |


# **ReviewsAPI_reviewsCreateQueue**
```c
review_queue_t* ReviewsAPI_reviewsCreateQueue(apiClient_t *apiClient, char *tenantId, char *projectId, create_review_queue_http_request_t *create_review_queue_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**create_review_queue_http_request** | **[create_review_queue_http_request_t](create_review_queue_http_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[review_queue_t](review_queue.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ReviewsAPI_reviewsEnqueueTaskFromTrace**
```c
review_task_t* ReviewsAPI_reviewsEnqueueTaskFromTrace(apiClient_t *apiClient, char *tenantId, char *projectId, char *queueId, enqueue_review_task_from_trace_http_request_t *enqueue_review_task_from_trace_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**queueId** | **char \*** | queue_id |
**enqueue_review_task_from_trace_http_request** | **[enqueue_review_task_from_trace_http_request_t](enqueue_review_task_from_trace_http_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[review_task_t](review_task.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ReviewsAPI_reviewsListTasks**
```c
review_task_list_response_t* ReviewsAPI_reviewsListTasks(apiClient_t *apiClient, char *tenantId, char *projectId, char *queueId, review_task_state_e state, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**queueId** | **char \*** | queue_id |
**state** | **review_task_state_e** |  | [optional]
**pageSize** | **int \*** | Maximum number of review tasks to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional]
**pageToken** | **char \*** | Opaque continuation token returned by the preceding list request. | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[review_task_list_response_t](review_task_list_response.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ReviewsAPI_reviewsPromoteAnnotation**
```c
dataset_case_t* ReviewsAPI_reviewsPromoteAnnotation(apiClient_t *apiClient, char *tenantId, char *projectId, char *queueId, char *taskId, char *annotationId, promote_review_annotation_http_request_t *promote_review_annotation_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**queueId** | **char \*** | queue_id |
**taskId** | **char \*** | task_id |
**annotationId** | **char \*** | annotation_id |
**promote_review_annotation_http_request** | **[promote_review_annotation_http_request_t](promote_review_annotation_http_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dataset_case_t](dataset_case.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ReviewsAPI_reviewsSubmitAnnotation**
```c
review_annotation_t* ReviewsAPI_reviewsSubmitAnnotation(apiClient_t *apiClient, char *tenantId, char *projectId, char *queueId, char *taskId, submit_review_annotation_http_request_t *submit_review_annotation_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**queueId** | **char \*** | queue_id |
**taskId** | **char \*** | task_id |
**submit_review_annotation_http_request** | **[submit_review_annotation_http_request_t](submit_review_annotation_http_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[review_annotation_t](review_annotation.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
