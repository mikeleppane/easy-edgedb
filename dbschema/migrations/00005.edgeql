CREATE MIGRATION m1ws2drajbfxh3apvebvztbqlzm3fd2f77xmw4wwg75zchyrn74ysa
    ONTO m1wllvxdb4syh2tzoxpl2wlmyk2sp3txregdty67rfkb27mpgh467a
{
  CREATE ABSTRACT TYPE default::Place {
      CREATE PROPERTY important_places: array<std::str>;
      CREATE PROPERTY modern_name: std::str;
      CREATE REQUIRED PROPERTY name: std::str;
  };
  ALTER TYPE default::City {
      EXTENDING default::Place LAST;
      ALTER PROPERTY important_places {
          DROP OWNED;
          RESET TYPE;
      };
      ALTER PROPERTY modern_name {
          DROP OWNED;
          RESET TYPE;
      };
      ALTER PROPERTY name {
          RESET OPTIONALITY;
          DROP OWNED;
          RESET TYPE;
      };
  };
  CREATE TYPE default::Country EXTENDING default::Place;
  ALTER TYPE default::Person {
      ALTER LINK places_visited {
          SET TYPE default::Place;
      };
  };
  CREATE TYPE default::Vampire EXTENDING default::Person {
      CREATE PROPERTY age: std::int16;
  };
};
