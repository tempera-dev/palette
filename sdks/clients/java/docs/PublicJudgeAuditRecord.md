

# PublicJudgeAuditRecord

Client-facing judge ledger row. The backing `provider`, the `provider_secret_id`, and our raw `provider_cost` are INTERNAL (staff-only) and must never reach a customer — exposing `provider_cost` alongside `charged_cost` would also leak our margin (billing-credits-contract §11). Only customer-facing fields appear here, including `charged_cost` (the amount the customer actually pays).

## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
|**cached** | **Boolean** |  |  |
|**chargedCost** | [**Money**](Money.md) |  |  |
|**createdAt** | **OffsetDateTime** |  |  |
|**evaluatorId** | **String** |  |  |
|**judgeCallId** | **String** |  |  |
|**model** | **String** |  |  |
|**projectId** | **String** |  |  |
|**requestHash** | **String** |  |  |
|**responseHash** | **String** |  |  |
|**score** | **Double** |  |  |
|**tenantId** | **String** |  |  |
