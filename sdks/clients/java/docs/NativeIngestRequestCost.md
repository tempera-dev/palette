

# NativeIngestRequestCost

## oneOf schemas
* [Money](Money.md)

NOTE: this class is nullable.

## Example
```java
// Import classes:
import ai.palette.client.model.NativeIngestRequestCost;
import ai.palette.client.model.Money;

public class Example {
    public static void main(String[] args) {
        NativeIngestRequestCost exampleNativeIngestRequestCost = new NativeIngestRequestCost();

        // create a new Money
        Money exampleMoney = new Money();
        // set NativeIngestRequestCost to Money
        exampleNativeIngestRequestCost.setActualInstance(exampleMoney);
        // to get back the Money set earlier
        Money testMoney = (Money) exampleNativeIngestRequestCost.getActualInstance();
    }
}
```
