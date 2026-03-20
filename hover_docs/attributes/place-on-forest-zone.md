```rms
place_on_forest_zone
```

---

Places objects only on, and directly next to, tiles with trees on them (including straggler trees and scenario editor trees).

**Mutually exclusive with**: [avoid_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ym7v1j9vbnle)

**Example**: Place sheep all along the edge of forests.
```rms
<OBJECTS_GENERATION>
create_object SHEEP {
  number_of_objects 99999
  place_on_forest_zone
}
```

**See more**: [place_on_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.38vodsu87lbp)
