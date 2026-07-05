# canonical_span_t

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attributes** | **list_t*** |  | 
**cost** | [**money_t**](money.md) \* |  | [optional] 
**end_time** | **char \*** |  | [optional] 
**environment_id** | **char \*** |  | 
**input_ref** | [**artifact_ref_t**](artifact_ref.md) \* |  | [optional] 
**kind** | **char \*** | Canonical agent span kind such as agent.run or llm.call | 
**model** | [**model_ref_t**](model_ref.md) \* |  | [optional] 
**name** | **char \*** |  | 
**normalizer_version** | **char \*** |  | 
**output_ref** | [**artifact_ref_t**](artifact_ref.md) \* |  | [optional] 
**parent_span_id** | **char \*** |  | [optional] 
**project_id** | **char \*** |  | 
**raw_ref** | [**artifact_ref_t**](artifact_ref.md) \* |  | 
**sampling_weight** | **double** | Inverse-probability sampling weight, &#x60;1 / keep_probability&#x60;, stamped on the tail-sampling keep path (§1 #9, §9): &#x60;1.0&#x60; for a span kept with certainty (errors/slow/high-cost/policy keeps) and &#x60;1/p&#x60; for a span kept under probabilistic routine-traffic sampling at rate &#x60;p&#x60;. Roll-ups over a tail-sampled population must weight by this (Horvitz-Thompson) or be labelled biased — never silently averaged. &#x60;None&#x60; on spans ingested before the keep path recorded weights (or by clients that don&#39;t); such a span cannot be de-biased, so any roll-up including it is flagged [&#x60;RollupWeighting::BiasedUnweighted&#x60;]. | [optional] 
**schema_version** | **int** |  | 
**seq** | **long** |  | 
**span_id** | **char \*** |  | 
**start_time** | **char \*** |  | 
**status** | **span_status_t \*** |  | 
**tenant_id** | **char \*** |  | 
**tokens** | [**token_counts_t**](token_counts.md) \* |  | [optional] 
**trace_id** | **char \*** |  | 
**unmapped_attrs** | **any_type_t \*** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


