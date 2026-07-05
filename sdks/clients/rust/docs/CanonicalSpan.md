# CanonicalSpan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attributes** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**cost** | Option<[**models::Money**](Money.md)> |  | [optional]
**end_time** | Option<**String**> |  | [optional]
**environment_id** | **String** |  | 
**input_ref** | Option<[**models::ArtifactRef**](ArtifactRef.md)> |  | [optional]
**kind** | **String** | Canonical agent span kind such as agent.run or llm.call | 
**model** | Option<[**models::ModelRef**](ModelRef.md)> |  | [optional]
**name** | **String** |  | 
**normalizer_version** | **String** |  | 
**output_ref** | Option<[**models::ArtifactRef**](ArtifactRef.md)> |  | [optional]
**parent_span_id** | Option<**String**> |  | [optional]
**project_id** | **String** |  | 
**raw_ref** | [**models::ArtifactRef**](ArtifactRef.md) |  | 
**sampling_weight** | Option<**f64**> | Inverse-probability sampling weight, `1 / keep_probability`, stamped on the tail-sampling keep path (§1 #9, §9): `1.0` for a span kept with certainty (errors/slow/high-cost/policy keeps) and `1/p` for a span kept under probabilistic routine-traffic sampling at rate `p`. Roll-ups over a tail-sampled population must weight by this (Horvitz-Thompson) or be labelled biased — never silently averaged. `None` on spans ingested before the keep path recorded weights (or by clients that don't); such a span cannot be de-biased, so any roll-up including it is flagged [`RollupWeighting::BiasedUnweighted`]. | [optional]
**schema_version** | **i32** |  | 
**seq** | **i64** |  | 
**span_id** | **String** |  | 
**start_time** | **String** |  | 
**status** | [**models::SpanStatus**](SpanStatus.md) |  | 
**tenant_id** | **String** |  | 
**tokens** | Option<[**models::TokenCounts**](TokenCounts.md)> |  | [optional]
**trace_id** | **String** |  | 
**unmapped_attrs** | Option<[**serde_json::Value**](.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


