

# NativeIngestRequest


## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
|**attributes** | **Map&lt;String, Object&gt;** |  |  |
|**authContext** | [**AuthContext**](AuthContext.md) |  |  [optional] |
|**cost** | [**NativeIngestRequestCost**](NativeIngestRequestCost.md) |  |  [optional] |
|**endTime** | **OffsetDateTime** |  |  [optional] |
|**idempotencyKey** | **String** |  |  [optional] |
|**input** | **Object** |  |  [optional] |
|**kind** | **String** | Canonical agent span kind such as agent.run or llm.call |  |
|**model** | [**NativeIngestRequestModel**](NativeIngestRequestModel.md) |  |  [optional] |
|**name** | **String** |  |  |
|**output** | **Object** |  |  [optional] |
|**parentSpanId** | [**NativeIngestRequestParentSpanId**](NativeIngestRequestParentSpanId.md) |  |  [optional] |
|**redactionClass** | **RedactionClass** |  |  |
|**scope** | [**TenantScope**](TenantScope.md) |  |  |
|**seq** | **Long** |  |  |
|**spanId** | **String** |  |  |
|**startTime** | **OffsetDateTime** |  |  [optional] |
|**status** | **SpanStatus** |  |  |
|**tokens** | [**NativeIngestRequestTokens**](NativeIngestRequestTokens.md) |  |  [optional] |
|**traceId** | **String** |  |  |
