

# StatusError


## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
|**code** | **Integer** | The HTTP status code, repeated in the body so the envelope is self-describing. |  |
|**details** | **List&lt;Map&lt;String, Object&gt;&gt;** | Structured error details. Present but empty when there are none. |  [optional] |
|**message** | **String** | A developer-facing description that is safe to log and safe to show. |  |
|**requestId** | **String** | The server-assigned request identifier, mirrored from the x-request-id response header. |  [optional] |
|**status** | [**StatusEnum**](#StatusEnum) | The canonical google.rpc.Code name, for example INVALID_ARGUMENT. |  |



## Enum: StatusEnum

| Name | Value |
|---- | -----|
| CANCELLED | &quot;CANCELLED&quot; |
| UNKNOWN | &quot;UNKNOWN&quot; |
| INVALID_ARGUMENT | &quot;INVALID_ARGUMENT&quot; |
| DEADLINE_EXCEEDED | &quot;DEADLINE_EXCEEDED&quot; |
| NOT_FOUND | &quot;NOT_FOUND&quot; |
| ALREADY_EXISTS | &quot;ALREADY_EXISTS&quot; |
| PERMISSION_DENIED | &quot;PERMISSION_DENIED&quot; |
| UNAUTHENTICATED | &quot;UNAUTHENTICATED&quot; |
| RESOURCE_EXHAUSTED | &quot;RESOURCE_EXHAUSTED&quot; |
| FAILED_PRECONDITION | &quot;FAILED_PRECONDITION&quot; |
| ABORTED | &quot;ABORTED&quot; |
| OUT_OF_RANGE | &quot;OUT_OF_RANGE&quot; |
| UNIMPLEMENTED | &quot;UNIMPLEMENTED&quot; |
| INTERNAL | &quot;INTERNAL&quot; |
| UNAVAILABLE | &quot;UNAVAILABLE&quot; |
| DATA_LOSS | &quot;DATA_LOSS&quot; |
