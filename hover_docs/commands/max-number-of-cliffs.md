```rms
max_number_of_cliffs NUMBER
```

---

Maximum number of distinct cliffs to generate.
The actual count is chosen randomly between [min_number_of_cliffs](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3j0rxjupzp29) (inclusive) and `max_number_of_cliffs` (exclusive).

- Does not scale with [map size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qannz915qgy5). Use conditionals to scale manually.
- Must not be less than [min_number_of_cliffs](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3j0rxjupzp29).

**Arguments**:
- `NUMBER`: The minimum number of cliffs, exclusive. Can be an integer, a constant, a [random expression](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ml72cdygzrfv), or a [math expression](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bqp5f3wcm40e).

**Example**: Generate 5, 6, or 7 cliffs.
```rms
<CLIFF_GENERATION>
min_number_of_cliffs 5
max_number_of_cliffs 8
```

**See more**: [max_number_of_cliffs](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fd5g85qqk5wj)
