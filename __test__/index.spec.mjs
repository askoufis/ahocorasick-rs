import test from "ava";

import { AhoCorasick } from "../index.js";

test("should find matches", (t) => {
  const ac = new AhoCorasick(["keyword0", "keyword1", "etc"]);

  const results = ac.search(
    "should find keyword0 from position 12 to 20 and keyword1 from position 40 to 48",
  );

  t.deepEqual(results, [
    [0, 12, 20],
    [1, 40, 48],
  ]);
});
