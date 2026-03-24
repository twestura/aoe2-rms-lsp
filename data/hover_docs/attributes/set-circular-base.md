```rms
set_circular_base
```

---

Changes the square land origin into a circle of the size that would be exactly inscribed by the square. DE only.

- Land origins with a [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh) of 3 or smaller are still a perfect square. Larger bases are more obviously circular.
- Produces a perfect circle when combined with [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ipg3ruf70nn4) 0 or [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bzvr6x5i10na) 0.
- **Bug**: when combined with [base_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ppqecxdcopxb), the whole square will be elevated. Prevented by adding [land_conformity](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.4ox4rtue3tq0) 99.

**Example**: Create a circular desert for each player.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DESERT
  land_percent 0
  base_size 12
  set_circular_base
}
```

**See more**: [set_circular_base](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xl67z7pma1vl)
