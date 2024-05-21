DROP TABLE IF EXISTS Weapons;

CREATE TABLE Weapons(
  ID INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  damage_low_bound REAL NOT NULL,
  damage_up_bound REAL NOT NULL,
  crit_chance REAL NOT NULL,
  crit_multiplier REAL NOT NULL,
  range REAL NOT NULL,
  accuracy REAL NOT NULL,
  shrink_rate REAL NOT NULL
);
