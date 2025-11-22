
# Data

## ref.1

Prompt (using t3.chat Premium, selected Model Grok 4)

START-PROMPT
I have the following Rust enum and I want to deserialize it, but serde complains it does not support tagged enums which are tuples, is there a trick to to it or do I have to write my own deserialize?

```
    #[derive(Deserialize)]
    #[serde(tag = "type")]
    pub enum Type {
        Deposit(Metadata, Decimal),
        Withdrawal(Metadata, Decimal),
        Dispute(Metadata),
        Resolve(Metadata),
        Chargeback(Metadata),
    }
```

The data is CSV and looks like this:

```
type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
dispute, 2, 5,
```
END-PROMPT
