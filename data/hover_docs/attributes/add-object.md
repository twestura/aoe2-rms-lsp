```rms
add_object OBJECT PERCENT
```

---

Adds an object to a [create_object_group](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nx4tooy9xi9) pool. When the group is used, one object is chosen at random from all entries.

- **Bug**: `PERCENT` currently does not work as documented. All objects in the group are equally likely regardless of the specified value. Entries may be listed multiple times to adjust their probability as a workaround.

**Arguments**:
- `OBJECT`: An [`object constant`](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nvxriamulybh).
- `PERCENT`: Intended weight for the object (0&ndash;99). Currently bugged and non-functional.

**Example**: Create a pool of cow variants and generate groups of mixed cows.
```rms
<OBJECTS_GENERATION>
create_object_group HERDABLE_A {
  add_object DLC_COW 25
  add_object DLC_COW_B 25
  add_object DLC_COW_C 25
  add_object DLC_COW_D 25
}
create_object HERDABLE_A {
  number_of_objects 6
  number_of_groups 24
}
```

**See more**: [add_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.9zts2xgopihj)
