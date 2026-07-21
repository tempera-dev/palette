# TemperaEvidenceReceipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Created** | **bool** |  |
**DeclaredContentSha256** | **string** |  |
**ExternalId** | **string** |  |
**Kind** | [**ExternalEvalEvidenceKind**](ExternalEvalEvidenceKind.md) |  |
**ProjectId** | **string** |  |
**PublicKeySha256** | **string** |  |
**SchemaVersion** | **string** |  |
**SignatureSha256** | **string** |  |
**SignedPayloadSha256** | **string** |  |
**SourceSchemaVersion** | **string** |  |
**StoredAt** | **time.Time** |  |
**Summary** | [**TemperaEvidenceSummary**](TemperaEvidenceSummary.md) |  |
**TenantId** | **string** |  |

## Methods

### NewTemperaEvidenceReceipt

`func NewTemperaEvidenceReceipt(created bool, declaredContentSha256 string, externalId string, kind ExternalEvalEvidenceKind, projectId string, publicKeySha256 string, schemaVersion string, signatureSha256 string, signedPayloadSha256 string, sourceSchemaVersion string, storedAt time.Time, summary TemperaEvidenceSummary, tenantId string, ) *TemperaEvidenceReceipt`

NewTemperaEvidenceReceipt instantiates a new TemperaEvidenceReceipt object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTemperaEvidenceReceiptWithDefaults

`func NewTemperaEvidenceReceiptWithDefaults() *TemperaEvidenceReceipt`

NewTemperaEvidenceReceiptWithDefaults instantiates a new TemperaEvidenceReceipt object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreated

`func (o *TemperaEvidenceReceipt) GetCreated() bool`

GetCreated returns the Created field if non-nil, zero value otherwise.

### GetCreatedOk

`func (o *TemperaEvidenceReceipt) GetCreatedOk() (*bool, bool)`

GetCreatedOk returns a tuple with the Created field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreated

`func (o *TemperaEvidenceReceipt) SetCreated(v bool)`

SetCreated sets Created field to given value.


### GetDeclaredContentSha256

`func (o *TemperaEvidenceReceipt) GetDeclaredContentSha256() string`

GetDeclaredContentSha256 returns the DeclaredContentSha256 field if non-nil, zero value otherwise.

### GetDeclaredContentSha256Ok

`func (o *TemperaEvidenceReceipt) GetDeclaredContentSha256Ok() (*string, bool)`

GetDeclaredContentSha256Ok returns a tuple with the DeclaredContentSha256 field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeclaredContentSha256

`func (o *TemperaEvidenceReceipt) SetDeclaredContentSha256(v string)`

SetDeclaredContentSha256 sets DeclaredContentSha256 field to given value.


### GetExternalId

`func (o *TemperaEvidenceReceipt) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *TemperaEvidenceReceipt) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *TemperaEvidenceReceipt) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.


### GetKind

`func (o *TemperaEvidenceReceipt) GetKind() ExternalEvalEvidenceKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *TemperaEvidenceReceipt) GetKindOk() (*ExternalEvalEvidenceKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *TemperaEvidenceReceipt) SetKind(v ExternalEvalEvidenceKind)`

SetKind sets Kind field to given value.


### GetProjectId

`func (o *TemperaEvidenceReceipt) GetProjectId() string`

GetProjectId returns the ProjectId field if non-nil, zero value otherwise.

### GetProjectIdOk

`func (o *TemperaEvidenceReceipt) GetProjectIdOk() (*string, bool)`

GetProjectIdOk returns a tuple with the ProjectId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProjectId

`func (o *TemperaEvidenceReceipt) SetProjectId(v string)`

SetProjectId sets ProjectId field to given value.


### GetPublicKeySha256

`func (o *TemperaEvidenceReceipt) GetPublicKeySha256() string`

GetPublicKeySha256 returns the PublicKeySha256 field if non-nil, zero value otherwise.

### GetPublicKeySha256Ok

`func (o *TemperaEvidenceReceipt) GetPublicKeySha256Ok() (*string, bool)`

GetPublicKeySha256Ok returns a tuple with the PublicKeySha256 field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKeySha256

`func (o *TemperaEvidenceReceipt) SetPublicKeySha256(v string)`

SetPublicKeySha256 sets PublicKeySha256 field to given value.


### GetSchemaVersion

`func (o *TemperaEvidenceReceipt) GetSchemaVersion() string`

GetSchemaVersion returns the SchemaVersion field if non-nil, zero value otherwise.

### GetSchemaVersionOk

`func (o *TemperaEvidenceReceipt) GetSchemaVersionOk() (*string, bool)`

GetSchemaVersionOk returns a tuple with the SchemaVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchemaVersion

`func (o *TemperaEvidenceReceipt) SetSchemaVersion(v string)`

SetSchemaVersion sets SchemaVersion field to given value.


### GetSignatureSha256

`func (o *TemperaEvidenceReceipt) GetSignatureSha256() string`

GetSignatureSha256 returns the SignatureSha256 field if non-nil, zero value otherwise.

### GetSignatureSha256Ok

`func (o *TemperaEvidenceReceipt) GetSignatureSha256Ok() (*string, bool)`

GetSignatureSha256Ok returns a tuple with the SignatureSha256 field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSignatureSha256

`func (o *TemperaEvidenceReceipt) SetSignatureSha256(v string)`

SetSignatureSha256 sets SignatureSha256 field to given value.


### GetSignedPayloadSha256

`func (o *TemperaEvidenceReceipt) GetSignedPayloadSha256() string`

GetSignedPayloadSha256 returns the SignedPayloadSha256 field if non-nil, zero value otherwise.

### GetSignedPayloadSha256Ok

`func (o *TemperaEvidenceReceipt) GetSignedPayloadSha256Ok() (*string, bool)`

GetSignedPayloadSha256Ok returns a tuple with the SignedPayloadSha256 field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSignedPayloadSha256

`func (o *TemperaEvidenceReceipt) SetSignedPayloadSha256(v string)`

SetSignedPayloadSha256 sets SignedPayloadSha256 field to given value.


### GetSourceSchemaVersion

`func (o *TemperaEvidenceReceipt) GetSourceSchemaVersion() string`

GetSourceSchemaVersion returns the SourceSchemaVersion field if non-nil, zero value otherwise.

### GetSourceSchemaVersionOk

`func (o *TemperaEvidenceReceipt) GetSourceSchemaVersionOk() (*string, bool)`

GetSourceSchemaVersionOk returns a tuple with the SourceSchemaVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceSchemaVersion

`func (o *TemperaEvidenceReceipt) SetSourceSchemaVersion(v string)`

SetSourceSchemaVersion sets SourceSchemaVersion field to given value.


### GetStoredAt

`func (o *TemperaEvidenceReceipt) GetStoredAt() time.Time`

GetStoredAt returns the StoredAt field if non-nil, zero value otherwise.

### GetStoredAtOk

`func (o *TemperaEvidenceReceipt) GetStoredAtOk() (*time.Time, bool)`

GetStoredAtOk returns a tuple with the StoredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStoredAt

`func (o *TemperaEvidenceReceipt) SetStoredAt(v time.Time)`

SetStoredAt sets StoredAt field to given value.


### GetSummary

`func (o *TemperaEvidenceReceipt) GetSummary() TemperaEvidenceSummary`

GetSummary returns the Summary field if non-nil, zero value otherwise.

### GetSummaryOk

`func (o *TemperaEvidenceReceipt) GetSummaryOk() (*TemperaEvidenceSummary, bool)`

GetSummaryOk returns a tuple with the Summary field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSummary

`func (o *TemperaEvidenceReceipt) SetSummary(v TemperaEvidenceSummary)`

SetSummary sets Summary field to given value.


### GetTenantId

`func (o *TemperaEvidenceReceipt) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *TemperaEvidenceReceipt) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *TemperaEvidenceReceipt) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
