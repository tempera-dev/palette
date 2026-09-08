

# NativeIngestRequestModel

## oneOf schemas
* [ModelRef](ModelRef.md)

NOTE: this class is nullable.

## Example
```java
// Import classes:
import ai.palette.client.model.NativeIngestRequestModel;
import ai.palette.client.model.ModelRef;

public class Example {
    public static void main(String[] args) {
        NativeIngestRequestModel exampleNativeIngestRequestModel = new NativeIngestRequestModel();

        // create a new ModelRef
        ModelRef exampleModelRef = new ModelRef();
        // set NativeIngestRequestModel to ModelRef
        exampleNativeIngestRequestModel.setActualInstance(exampleModelRef);
        // to get back the ModelRef set earlier
        ModelRef testModelRef = (ModelRef) exampleNativeIngestRequestModel.getActualInstance();
    }
}
```
