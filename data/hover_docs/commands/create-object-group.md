```rms
create_object_group GROUP_NAME {
  add_object OBJECT PERCENT
}
```

---

Defines a named pool of objects to randomly select from
When `GROUP_NAME` is used in place of an object constant in [create_object]([URL](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2vz7nxt2afqo)), one object from the pool is chosen at random each time.

- If the group is placed with [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb), the selection is randomized independently for each placement.
- Groups should not mix objects with different resource amounts or gameplay effects when used with [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb), as each placement is independently randomized.
- **Bug**: `PERCENT` currently does not work as documented. All objects in the group are equally likely regardless of the specified value. Entries may be listed multiple times to adjust their probability as a workaround.

**Arguments**:
- `GROUP_NAME`: A name for the group. By convention, use uppercase letters, digits, and underscores.
- `OBJECT`: An [object constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.nvxriamulybh).
- `PERCENT`: Intended weight for the object&nbsp;(0&#8288;&ndash;&#8288;99). Currently bugged and non-functional.

**Attributes**:
- [add_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.9zts2xgopihj)

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

**See more**: [create_object_group](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nx4tooy9xi9)
