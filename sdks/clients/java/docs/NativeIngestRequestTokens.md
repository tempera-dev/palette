

# NativeIngestRequestTokens

## oneOf schemas
* [TokenCounts](TokenCounts.md)

NOTE: this class is nullable.

## Example
```java
// Import classes:
import ai.palette.client.model.NativeIngestRequestTokens;
import ai.palette.client.model.TokenCounts;

public class Example {
    public static void main(String[] args) {
        NativeIngestRequestTokens exampleNativeIngestRequestTokens = new NativeIngestRequestTokens();

        // create a new TokenCounts
        TokenCounts exampleTokenCounts = new TokenCounts();
        // set NativeIngestRequestTokens to TokenCounts
        exampleNativeIngestRequestTokens.setActualInstance(exampleTokenCounts);
        // to get back the TokenCounts set earlier
        TokenCounts testTokenCounts = (TokenCounts) exampleNativeIngestRequestTokens.getActualInstance();
    }
}
```
