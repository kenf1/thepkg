## Feature flags:

|Feature flag|Description|Includes|
|---|---|---|
|default|Default|-|
|all|All|io, qr, webscrape, crypt|
|crypt|Basic string encryption/decryption||
|io|Input/Output|csv, polars/lazy, dotenv|
|qr|QR reader|bardecoder, image, reqwest/blocking|
|webscrape|Webscraping|reqwest/blocking, select|

See [Cargo.toml](../Cargo.toml) for more info
