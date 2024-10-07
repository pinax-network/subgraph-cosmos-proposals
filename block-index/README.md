## Cosmos Block Index

### Available query fields

These are the expressions that can be used in queries:

- `event.type:<type>` - event type
- `event.type:<type>:<attribute>` - event type by attribute key
- `event.type:<type>:<attribute>:<value>` - event type by attribute key/value
- `event.attribute:<key>` - event attribute key
- `event.attribute:<key>:<value>` - event attribute key/value

Queries can include `&&` and `||` logical operands, as well as `(` and `)` parenthesis.
