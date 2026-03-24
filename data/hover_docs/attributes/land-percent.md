```rms
land_percent PERCENT
```

---

Percentage of the total size of the map.

- In [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr) and [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx): the size that the land grows to cover. For [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr), the percentage is divided equally among all players. If land growth is constrained by borders or other lands, lands may be smaller than specified.
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): percentage of the total map painted by this command. If [number_of_clumps](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1tzwe1brcw80) is specified, this value is divided equally among the clumps. Terrain is replaced only where the appropriate [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ptxp1ht2fh0p) or [base_layer](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nlwld8oca536) is present. Terrain only replaces the specified number of clumps and may not fill 100% of the map if set to 100.

**Mutually exclusive with**: [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bzvr6x5i10na)

**Arguments**:
- `PERCENT`: A number 0&ndash;100 (default 100 in land generation; [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qdz0o9mtb2hi) in terrain generation).

**Example**: Allocate 20% of the map to player lands.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DIRT
  land_percent 20
}
```

**See more**: [land_percent (create_player_lands / create_land)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ipg3ruf70nn4), [land_percent (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.tzpfzbf2ze3w)
