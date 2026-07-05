# CanonicalSpan


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attributes** | **Dict[str, object]** |  | 
**cost** | [**Money**](Money.md) |  | [optional] 
**end_time** | **datetime** |  | [optional] 
**environment_id** | **str** |  | 
**input_ref** | [**ArtifactRef**](ArtifactRef.md) |  | [optional] 
**kind** | **str** | Canonical agent span kind such as agent.run or llm.call | 
**model** | [**ModelRef**](ModelRef.md) |  | [optional] 
**name** | **str** |  | 
**normalizer_version** | **str** |  | 
**output_ref** | [**ArtifactRef**](ArtifactRef.md) |  | [optional] 
**parent_span_id** | **str** |  | [optional] 
**project_id** | **str** |  | 
**raw_ref** | [**ArtifactRef**](ArtifactRef.md) |  | 
**sampling_weight** | **float** | Inverse-probability sampling weight, &#x60;1 / keep_probability&#x60;, stamped on the tail-sampling keep path (§1 #9, §9): &#x60;1.0&#x60; for a span kept with certainty (errors/slow/high-cost/policy keeps) and &#x60;1/p&#x60; for a span kept under probabilistic routine-traffic sampling at rate &#x60;p&#x60;. Roll-ups over a tail-sampled population must weight by this (Horvitz-Thompson) or be labelled biased — never silently averaged. &#x60;None&#x60; on spans ingested before the keep path recorded weights (or by clients that don&#39;t); such a span cannot be de-biased, so any roll-up including it is flagged [&#x60;RollupWeighting::BiasedUnweighted&#x60;]. | [optional] 
**schema_version** | **int** |  | 
**seq** | **int** |  | 
**span_id** | **str** |  | 
**start_time** | **datetime** |  | 
**status** | [**SpanStatus**](SpanStatus.md) |  | 
**tenant_id** | **str** |  | 
**tokens** | [**TokenCounts**](TokenCounts.md) |  | [optional] 
**trace_id** | **str** |  | 
**unmapped_attrs** | **object** |  | 

## Example

```python
from beater_client.models.canonical_span import CanonicalSpan

# TODO update the JSON string below
json = "{}"
# create an instance of CanonicalSpan from a JSON string
canonical_span_instance = CanonicalSpan.from_json(json)
# print the JSON string representation of the object
print(CanonicalSpan.to_json())

# convert the object into a dict
canonical_span_dict = canonical_span_instance.to_dict()
# create an instance of CanonicalSpan from a dict
canonical_span_from_dict = CanonicalSpan.from_dict(canonical_span_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


