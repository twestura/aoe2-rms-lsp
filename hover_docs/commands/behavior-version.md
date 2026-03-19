```rms
behavior_version VERSION_NUMBER
```

---

Used for versioning changes that might affect how existing maps generate.

**Arguments**:
- `VERSION_NUMBER`: 0, 1, or 2 (default 0).
  - 0 is classic behavior
  - 1 is new behavior
  - 2 the same as 1, as well as supposedly changing the behavior of object placement for per-player lands

The `behavior_version` changes land generation such that when specifying [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.bzvr6x5i10na) or [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ipg3ruf70nn4), the square land origin covered by the [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.b9qafcmyygh) is included in the total, rather than being additive.
The new behavior also fixes a bug where land order would influence the generation.

- This command may be used anywhere, but by convention is used in player setup.
- May be used in the future to gate off compatibility-breaking changes.
- To update to a new version, increase [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.bzvr6x5i10na) by (2 * [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.b9qafcmyygh) + 1)². The [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ipg3ruf70nn4) must also be changed by a percentage that varies with map size.

**Example**: Activate new land generation behavior and observe that lands become smaller.

```rms
<PLAYER_SETUP>
behavior_version 2

<LAND_GENERATION>
create_player_lands {
  terrain_type DESERT
  number_of_tiles 250
  base_size 12
}
```

**See more**: [behavior_version](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.seeuqpcozayb)
