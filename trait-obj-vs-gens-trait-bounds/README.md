## Description
Example how to use trait objects and generics with trait bounds.
App has user_store which can be HashMap or Postgres table. 

## Thoughts
Using trait objects leads to simpler code. However, this approach introduces slight overhead at runtime due to dynamic dispatch. It also requires that we only use object-safe traits.
An alternative approach is to use generics with trait bounds. This approach doesn’t incur any runtime overhead and allows us to use any traits we’d like.The downside is more complex code. 