# ImportTemperaEvidenceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_json** | **String** | Canonical compact JSON signed by the release/decision key. The endpoint rejects equivalent but non-canonical JSON so the verified bytes are unambiguous across SDKs. |
**public_key_pem** | **String** | PEM SubjectPublicKeyInfo for the Ed25519 key whose exact byte digest is pinned inside the signed payload. |
**signature_base64** | **String** | Standard-base64 detached Ed25519 signature over `canonical_json` bytes. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
