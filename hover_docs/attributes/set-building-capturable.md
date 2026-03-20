```rms
set_building_capturable
```

---

Makes a building switch control to whoever most recently has units nearby.

- Has no effect on units or other non-building objects.
- Can be used for buildings that start under gaia control or under player control.
- Capturable buildings cannot be deleted.
- Capturable buildings can be destroyed (unless [make_indestructible](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.a9ken9hkekd6) is used).

**Mutually exclusive with**: [set_gaia_unconvertible](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.g4mzdyb4izbo)

**Example**: Place an outpost that converts to whoever is nearby.
```rms
<OBJECTS_GENERATION>
create_object OUTPOST {
  set_gaia_object_only
  make_indestructible
  set_building_capturable
}
```

**See more**: [set_building_capturable](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.svu8loj25dpl)
