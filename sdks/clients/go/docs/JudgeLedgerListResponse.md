# JudgeLedgerListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NextPageToken** | Pointer to **NullableString** |  | [optional]
**Records** | [**[]PublicJudgeAuditRecord**](PublicJudgeAuditRecord.md) |  |

## Methods

### NewJudgeLedgerListResponse

`func NewJudgeLedgerListResponse(records []PublicJudgeAuditRecord, ) *JudgeLedgerListResponse`

NewJudgeLedgerListResponse instantiates a new JudgeLedgerListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewJudgeLedgerListResponseWithDefaults

`func NewJudgeLedgerListResponseWithDefaults() *JudgeLedgerListResponse`

NewJudgeLedgerListResponseWithDefaults instantiates a new JudgeLedgerListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNextPageToken

`func (o *JudgeLedgerListResponse) GetNextPageToken() string`

GetNextPageToken returns the NextPageToken field if non-nil, zero value otherwise.

### GetNextPageTokenOk

`func (o *JudgeLedgerListResponse) GetNextPageTokenOk() (*string, bool)`

GetNextPageTokenOk returns a tuple with the NextPageToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextPageToken

`func (o *JudgeLedgerListResponse) SetNextPageToken(v string)`

SetNextPageToken sets NextPageToken field to given value.

### HasNextPageToken

`func (o *JudgeLedgerListResponse) HasNextPageToken() bool`

HasNextPageToken returns a boolean if a field has been set.

### SetNextPageTokenNil

`func (o *JudgeLedgerListResponse) SetNextPageTokenNil(b bool)`

 SetNextPageTokenNil sets the value for NextPageToken to be an explicit nil

### UnsetNextPageToken
`func (o *JudgeLedgerListResponse) UnsetNextPageToken()`

UnsetNextPageToken ensures that no value is present for NextPageToken, not even an explicit nil
### GetRecords

`func (o *JudgeLedgerListResponse) GetRecords() []PublicJudgeAuditRecord`

GetRecords returns the Records field if non-nil, zero value otherwise.

### GetRecordsOk

`func (o *JudgeLedgerListResponse) GetRecordsOk() (*[]PublicJudgeAuditRecord, bool)`

GetRecordsOk returns a tuple with the Records field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecords

`func (o *JudgeLedgerListResponse) SetRecords(v []PublicJudgeAuditRecord)`

SetRecords sets Records field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
