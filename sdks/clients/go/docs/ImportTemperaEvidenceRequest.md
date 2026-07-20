# ImportTemperaEvidenceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanonicalJson** | **string** | Canonical compact JSON signed by the release/decision key. The endpoint rejects equivalent but non-canonical JSON so the verified bytes are unambiguous across SDKs. |
**PublicKeyPem** | **string** | PEM SubjectPublicKeyInfo for the Ed25519 key whose exact byte digest is pinned inside the signed payload. |
**SignatureBase64** | **string** | Standard-base64 detached Ed25519 signature over &#x60;canonical_json&#x60; bytes. |

## Methods

### NewImportTemperaEvidenceRequest

`func NewImportTemperaEvidenceRequest(canonicalJson string, publicKeyPem string, signatureBase64 string, ) *ImportTemperaEvidenceRequest`

NewImportTemperaEvidenceRequest instantiates a new ImportTemperaEvidenceRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewImportTemperaEvidenceRequestWithDefaults

`func NewImportTemperaEvidenceRequestWithDefaults() *ImportTemperaEvidenceRequest`

NewImportTemperaEvidenceRequestWithDefaults instantiates a new ImportTemperaEvidenceRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanonicalJson

`func (o *ImportTemperaEvidenceRequest) GetCanonicalJson() string`

GetCanonicalJson returns the CanonicalJson field if non-nil, zero value otherwise.

### GetCanonicalJsonOk

`func (o *ImportTemperaEvidenceRequest) GetCanonicalJsonOk() (*string, bool)`

GetCanonicalJsonOk returns a tuple with the CanonicalJson field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalJson

`func (o *ImportTemperaEvidenceRequest) SetCanonicalJson(v string)`

SetCanonicalJson sets CanonicalJson field to given value.


### GetPublicKeyPem

`func (o *ImportTemperaEvidenceRequest) GetPublicKeyPem() string`

GetPublicKeyPem returns the PublicKeyPem field if non-nil, zero value otherwise.

### GetPublicKeyPemOk

`func (o *ImportTemperaEvidenceRequest) GetPublicKeyPemOk() (*string, bool)`

GetPublicKeyPemOk returns a tuple with the PublicKeyPem field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKeyPem

`func (o *ImportTemperaEvidenceRequest) SetPublicKeyPem(v string)`

SetPublicKeyPem sets PublicKeyPem field to given value.


### GetSignatureBase64

`func (o *ImportTemperaEvidenceRequest) GetSignatureBase64() string`

GetSignatureBase64 returns the SignatureBase64 field if non-nil, zero value otherwise.

### GetSignatureBase64Ok

`func (o *ImportTemperaEvidenceRequest) GetSignatureBase64Ok() (*string, bool)`

GetSignatureBase64Ok returns a tuple with the SignatureBase64 field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSignatureBase64

`func (o *ImportTemperaEvidenceRequest) SetSignatureBase64(v string)`

SetSignatureBase64 sets SignatureBase64 field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
