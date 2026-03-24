```rms
cliff_curliness PERCENT
```

---

The chance that a cliff changes direction at each segment.
Low values produce straight cliffs, high values produce curly cliffs.

**Arguments**:
- `PERCENT`: A number in 0&#8288;&ndash;&#8288;100 (default 36).

**Example**: Generate notably curly cliffs.
```rms
<CLIFF_GENERATION>
min_number_of_cliffs 5
max_number_of_cliffs 5
min_length_of_cliff 10
max_length_of_cliff 10
cliff_curliness 50
```

**See more**: [cliff_curliness](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6ns6xzfo7h7c)
