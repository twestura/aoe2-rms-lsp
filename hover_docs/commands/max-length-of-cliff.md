```rms
max_length_of_cliff LENGTH
```

---

Maximum cliff length in cliff segments.
Cliff lengths are chosen randomly between [min_length_of_cliff](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.58mxeqmbhw5c) and `max_length_of_cliff` (both inclusive).

- The unit is cliff segments, not tiles. Each segment is 3&nbsp;tiles: length&nbsp;3 is 12&nbsp;tiles, length&nbsp;4 is 15&nbsp;tiles, etc.
- Cliffs may end up shorter than specified if they run out of space.
- Must not be less than [min_length_of_cliff](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.58mxeqmbhw5c), otherwise the game will crash.

**Arguments**:
- `LENGTH`: The number of cliff segments, must be at least&nbsp;3 (default 5).

**Example**: Generate cliffs of 12&#8288;&ndash;&#8288;18 tiles in length.
```rms
<CLIFF_GENERATION>
min_length_of_cliff 3 /* 3 * 4 == 12 */
max_length_of_cliff 5 /* 3 * 6 == 18 */
```

**See more**: [max_length_of_cliff](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1ee8y599eivc)
