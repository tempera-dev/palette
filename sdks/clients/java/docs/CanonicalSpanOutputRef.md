

# CanonicalSpanOutputRef

## oneOf schemas
* [ArtifactRef](ArtifactRef.md)

NOTE: this class is nullable.

## Example
```java
// Import classes:
import ai.palette.client.model.CanonicalSpanOutputRef;
import ai.palette.client.model.ArtifactRef;

public class Example {
    public static void main(String[] args) {
        CanonicalSpanOutputRef exampleCanonicalSpanOutputRef = new CanonicalSpanOutputRef();

        // create a new ArtifactRef
        ArtifactRef exampleArtifactRef = new ArtifactRef();
        // set CanonicalSpanOutputRef to ArtifactRef
        exampleCanonicalSpanOutputRef.setActualInstance(exampleArtifactRef);
        // to get back the ArtifactRef set earlier
        ArtifactRef testArtifactRef = (ArtifactRef) exampleCanonicalSpanOutputRef.getActualInstance();
    }
}
```
