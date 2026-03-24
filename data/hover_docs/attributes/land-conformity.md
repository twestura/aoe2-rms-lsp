```rms
land_conformity PERCENT
```

---

Intended as a more potent version of [clumping_factor](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7e3knrokkcni), with higher values conforming more to the shape of the land origin set by [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh), and negative values conforming less.

**This attribute is currently buggy. It is advised to avoid using it.**

**Arguments**:
- `PERCENT`: A number -1&ndash;100.
  - 0 is the same as not having this attribute. Any negative number behaves the same.
  - 100 causes this attribute to override [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh) unless [set_circular_base](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xl67z7pma1vl) is specified.
  - Useful ranges without [set_circular_base](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xl67z7pma1vl): 0&ndash;15, 25&ndash;35, 45&ndash;99, 100.
  - Useful ranges with [set_circular_base](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xl67z7pma1vl): 0&ndash;10, 20&ndash;30, 40&ndash;99, 100.

**See more**: [land_conformity](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.4ox4rtue3tq0)
