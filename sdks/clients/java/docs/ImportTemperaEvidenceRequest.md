

# ImportTemperaEvidenceRequest


## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
|**canonicalJson** | **String** | Canonical compact JSON signed by the release/decision key. The endpoint rejects equivalent but non-canonical JSON so the verified bytes are unambiguous across SDKs. |  |
|**publicKeyPem** | **String** | PEM SubjectPublicKeyInfo for the Ed25519 key whose exact byte digest is pinned inside the signed payload. |  |
|**signatureBase64** | **String** | Standard-base64 detached Ed25519 signature over &#x60;canonical_json&#x60; bytes. |  |
