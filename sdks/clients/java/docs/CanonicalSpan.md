

# CanonicalSpan


## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
|**attributes** | **Map&lt;String, Object&gt;** |  |  |
|**cost** | [**Money**](Money.md) |  |  [optional] |
|**endTime** | **OffsetDateTime** |  |  [optional] |
|**environmentId** | **String** |  |  |
|**inputRef** | [**ArtifactRef**](ArtifactRef.md) |  |  [optional] |
|**kind** | **String** | Canonical agent span kind such as agent.run or llm.call |  |
|**model** | [**ModelRef**](ModelRef.md) |  |  [optional] |
|**name** | **String** |  |  |
|**normalizerVersion** | **String** |  |  |
|**outputRef** | [**ArtifactRef**](ArtifactRef.md) |  |  [optional] |
|**parentSpanId** | **String** |  |  [optional] |
|**projectId** | **String** |  |  |
|**rawRef** | [**ArtifactRef**](ArtifactRef.md) |  |  |
|**samplingWeight** | **Double** | Inverse-probability sampling weight, &#x60;1 / keep_probability&#x60;, stamped on the tail-sampling keep path (§1 #9, §9): &#x60;1.0&#x60; for a span kept with certainty (errors/slow/high-cost/policy keeps) and &#x60;1/p&#x60; for a span kept under probabilistic routine-traffic sampling at rate &#x60;p&#x60;. Roll-ups over a tail-sampled population must weight by this (Horvitz-Thompson) or be labelled biased — never silently averaged. &#x60;None&#x60; on spans ingested before the keep path recorded weights (or by clients that don&#39;t); such a span cannot be de-biased, so any roll-up including it is flagged [&#x60;RollupWeighting::BiasedUnweighted&#x60;]. |  [optional] |
|**schemaVersion** | **Integer** |  |  |
|**seq** | **Long** |  |  |
|**spanId** | **String** |  |  |
|**startTime** | **OffsetDateTime** |  |  |
|**status** | **SpanStatus** |  |  |
|**tenantId** | **String** |  |  |
|**tokens** | [**TokenCounts**](TokenCounts.md) |  |  [optional] |
|**traceId** | **String** |  |  |
|**unmappedAttrs** | **Object** |  |  |



