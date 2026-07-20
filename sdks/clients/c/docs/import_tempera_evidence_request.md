# import_tempera_evidence_request_t

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_json** | **char \*** | Canonical compact JSON signed by the release/decision key. The endpoint rejects equivalent but non-canonical JSON so the verified bytes are unambiguous across SDKs. |
**public_key_pem** | **char \*** | PEM SubjectPublicKeyInfo for the Ed25519 key whose exact byte digest is pinned inside the signed payload. |
**signature_base64** | **char \*** | Standard-base64 detached Ed25519 signature over &#x60;canonical_json&#x60; bytes. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
