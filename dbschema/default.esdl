module default {
  scalar type Class extending enum<Rogue, Mystic, Merchant>;
  abstract type Person {
    required name: str;
    multi places_visited: Place;
  }

  type PC extending Person {
    required class: Class;
  }

  type NPC extending Person {
  }

  type Vampire extending Person {
    age: int16;
  }

  abstract type Place {
    required name: str;
    modern_name: str;
    important_places: array<str>;
  }

  type City extending Place;

  type Country extending Place;

}
