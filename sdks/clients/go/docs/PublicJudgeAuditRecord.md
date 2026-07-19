# PublicJudgeAuditRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Cached** | **bool** |  |
**ChargedCost** | [**Money**](Money.md) |  |
**CreatedAt** | **time.Time** |  |
**EvaluatorId** | **string** |  |
**JudgeCallId** | **string** |  |
**Model** | **string** |  |
**ProjectId** | **string** |  |
**RequestHash** | **string** |  |
**ResponseHash** | **string** |  |
**Score** | **float64** |  |
**TenantId** | **string** |  |

## Methods

### NewPublicJudgeAuditRecord

`func NewPublicJudgeAuditRecord(cached bool, chargedCost Money, createdAt time.Time, evaluatorId string, judgeCallId string, model string, projectId string, requestHash string, responseHash string, score float64, tenantId string, ) *PublicJudgeAuditRecord`

NewPublicJudgeAuditRecord instantiates a new PublicJudgeAuditRecord object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicJudgeAuditRecordWithDefaults

`func NewPublicJudgeAuditRecordWithDefaults() *PublicJudgeAuditRecord`

NewPublicJudgeAuditRecordWithDefaults instantiates a new PublicJudgeAuditRecord object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCached

`func (o *PublicJudgeAuditRecord) GetCached() bool`

GetCached returns the Cached field if non-nil, zero value otherwise.

### GetCachedOk

`func (o *PublicJudgeAuditRecord) GetCachedOk() (*bool, bool)`

GetCachedOk returns a tuple with the Cached field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCached

`func (o *PublicJudgeAuditRecord) SetCached(v bool)`

SetCached sets Cached field to given value.


### GetChargedCost

`func (o *PublicJudgeAuditRecord) GetChargedCost() Money`

GetChargedCost returns the ChargedCost field if non-nil, zero value otherwise.

### GetChargedCostOk

`func (o *PublicJudgeAuditRecord) GetChargedCostOk() (*Money, bool)`

GetChargedCostOk returns a tuple with the ChargedCost field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChargedCost

`func (o *PublicJudgeAuditRecord) SetChargedCost(v Money)`

SetChargedCost sets ChargedCost field to given value.


### GetCreatedAt

`func (o *PublicJudgeAuditRecord) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PublicJudgeAuditRecord) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PublicJudgeAuditRecord) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEvaluatorId

`func (o *PublicJudgeAuditRecord) GetEvaluatorId() string`

GetEvaluatorId returns the EvaluatorId field if non-nil, zero value otherwise.

### GetEvaluatorIdOk

`func (o *PublicJudgeAuditRecord) GetEvaluatorIdOk() (*string, bool)`

GetEvaluatorIdOk returns a tuple with the EvaluatorId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvaluatorId

`func (o *PublicJudgeAuditRecord) SetEvaluatorId(v string)`

SetEvaluatorId sets EvaluatorId field to given value.


### GetJudgeCallId

`func (o *PublicJudgeAuditRecord) GetJudgeCallId() string`

GetJudgeCallId returns the JudgeCallId field if non-nil, zero value otherwise.

### GetJudgeCallIdOk

`func (o *PublicJudgeAuditRecord) GetJudgeCallIdOk() (*string, bool)`

GetJudgeCallIdOk returns a tuple with the JudgeCallId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJudgeCallId

`func (o *PublicJudgeAuditRecord) SetJudgeCallId(v string)`

SetJudgeCallId sets JudgeCallId field to given value.


### GetModel

`func (o *PublicJudgeAuditRecord) GetModel() string`

GetModel returns the Model field if non-nil, zero value otherwise.

### GetModelOk

`func (o *PublicJudgeAuditRecord) GetModelOk() (*string, bool)`

GetModelOk returns a tuple with the Model field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetModel

`func (o *PublicJudgeAuditRecord) SetModel(v string)`

SetModel sets Model field to given value.


### GetProjectId

`func (o *PublicJudgeAuditRecord) GetProjectId() string`

GetProjectId returns the ProjectId field if non-nil, zero value otherwise.

### GetProjectIdOk

`func (o *PublicJudgeAuditRecord) GetProjectIdOk() (*string, bool)`

GetProjectIdOk returns a tuple with the ProjectId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProjectId

`func (o *PublicJudgeAuditRecord) SetProjectId(v string)`

SetProjectId sets ProjectId field to given value.


### GetRequestHash

`func (o *PublicJudgeAuditRecord) GetRequestHash() string`

GetRequestHash returns the RequestHash field if non-nil, zero value otherwise.

### GetRequestHashOk

`func (o *PublicJudgeAuditRecord) GetRequestHashOk() (*string, bool)`

GetRequestHashOk returns a tuple with the RequestHash field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequestHash

`func (o *PublicJudgeAuditRecord) SetRequestHash(v string)`

SetRequestHash sets RequestHash field to given value.


### GetResponseHash

`func (o *PublicJudgeAuditRecord) GetResponseHash() string`

GetResponseHash returns the ResponseHash field if non-nil, zero value otherwise.

### GetResponseHashOk

`func (o *PublicJudgeAuditRecord) GetResponseHashOk() (*string, bool)`

GetResponseHashOk returns a tuple with the ResponseHash field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResponseHash

`func (o *PublicJudgeAuditRecord) SetResponseHash(v string)`

SetResponseHash sets ResponseHash field to given value.


### GetScore

`func (o *PublicJudgeAuditRecord) GetScore() float64`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *PublicJudgeAuditRecord) GetScoreOk() (*float64, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *PublicJudgeAuditRecord) SetScore(v float64)`

SetScore sets Score field to given value.


### GetTenantId

`func (o *PublicJudgeAuditRecord) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *PublicJudgeAuditRecord) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *PublicJudgeAuditRecord) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
