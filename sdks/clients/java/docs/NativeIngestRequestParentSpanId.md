

# NativeIngestRequestParentSpanId

## oneOf schemas
* [String](String.md)

NOTE: this class is nullable.

## Example
```java
// Import classes:
import ai.palette.client.model.NativeIngestRequestParentSpanId;
import ai.palette.client.model.String;

public class Example {
    public static void main(String[] args) {
        NativeIngestRequestParentSpanId exampleNativeIngestRequestParentSpanId = new NativeIngestRequestParentSpanId();

        // create a new String
        String exampleString = new String();
        // set NativeIngestRequestParentSpanId to String
        exampleNativeIngestRequestParentSpanId.setActualInstance(exampleString);
        // to get back the String set earlier
        String testString = (String) exampleNativeIngestRequestParentSpanId.getActualInstance();
    }
}
```
