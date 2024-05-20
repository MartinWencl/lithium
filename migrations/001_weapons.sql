CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS Weapons;

CREATE TABLE Weapons (
  id UUID DEFAULT uuid_generate_v4 (),
  name VARCHAR NOT NULL,

  PRIMARY KEY (user_id)
);
