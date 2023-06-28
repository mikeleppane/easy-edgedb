CREATE MIGRATION m1wllvxdb4syh2tzoxpl2wlmyk2sp3txregdty67rfkb27mpgh467a
    ONTO m1acewuowvsna5o4f7fvnsmgixgsvabjp5l2vylrp4gyfs2mm33cha
{
  ALTER TYPE default::City {
      CREATE PROPERTY important_places: array<std::str>;
  };
  CREATE ABSTRACT TYPE default::Person {
      CREATE MULTI LINK places_visited: default::City;
      CREATE REQUIRED PROPERTY name: std::str;
  };
  ALTER TYPE default::NPC {
      EXTENDING default::Person LAST;
      ALTER LINK places_visited {
          RESET CARDINALITY;
          DROP OWNED;
          RESET TYPE;
      };
      ALTER PROPERTY name {
          RESET OPTIONALITY;
          DROP OWNED;
          RESET TYPE;
      };
  };
  CREATE SCALAR TYPE default::Class EXTENDING enum<Rogue, Mystic, Merchant>;
  CREATE TYPE default::PC EXTENDING default::Person {
      CREATE REQUIRED PROPERTY class: default::Class;
  };
};
