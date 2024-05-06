# `ahocorasick-rs`

A JavaScript binding to [BurntSushi's rust implementation of Aho-Corasick][aho-corasick].

[aho-corasick]: https://github.com/BurntSushi/aho-corasick

> [!CAUTION]
> This package exists to be consumed by Vanilla Extract.
> As a result, it does not have a stable API and may change at any time.
> Additionally, only APIs relevant to Vanilla Extract are exposed.

## Example

```ts
import { AhoCorasic } from "ahocorasick-rs";

const ac = new AhoCorasick(["keyword0", "keyword1", "etc"]);

// Returns tuples of [keywordIndex, start, end]
// End is exclusive
// [[0, 12, 20], [1, 40, 48]]
const results = ac.search(
  "should find keyword0 from position 12 to 20 and keyword1 from position 40 to 48",
);
```
