```rms
set_gaia_unconvertible
```

---

Makes a gaia object unrescuable by players and hostile toward them.

- Must be used after [set_gaia_object_only](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bnzkfqaopnv9).
- Gaia military units act as if on defensive stance — attacking anything that enters their search radius and retreating if the target moves away.
- Does not work when testing from the scenario editor.
- Unrescuable status does not apply to [second_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cr71z3stu8pd).
- Certain objects are always convertible (e.g. monuments) or behave unexpectedly (Town Centers, Gates).
- Gaia Markets lose functionality and cannot be traded with. Use object&nbsp;1646 to give Gaia an indestructible Market for players to trade with.
- Villagers repair gaia buildings and cannot attack them.

**Mutually exclusive with**: [set_building_capturable](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.svu8loj25dpl)

**Example**: Decorate the map with unrescuable gaia pyramids.
```rms
<OBJECTS_GENERATION>
create_object PYRAMID {
  number_of_objects 3
  set_gaia_object_only
  set_gaia_unconvertible
  make_indestructible
}
```

**See more**: [set_gaia_unconvertible](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.g4mzdyb4izbo)
