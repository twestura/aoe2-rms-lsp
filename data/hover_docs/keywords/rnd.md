```rms
rnd(min,max)
```

---

Generates a random number between `min` and `max` (inclusive).

**Arguments**:
- `min`: Inclusive minimum, must be an integer.
- `max`: Inclusive maximum, must be an integer.

Requires `min < max`.

Ensure there are **no spaces**; `rnd(5, 7)` is invalid because of the space.

**Example**: Place 5, 6, or 7&nbsp;gold piles.

```rms
<OBJECTS_GENERATION>
create_object GOLD {
  number_of_objects rnd(5,7)
}
```

**See more**: [rnd](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ml72cdygzrfv)
