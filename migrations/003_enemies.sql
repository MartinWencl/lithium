DROP TABLE IF EXISTS Enemies;

-- TODO: Customize to our specific needs
CREATE TABLE Enemies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    health INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    speed INTEGER NOT NULL,
    type TEXT NOT NULL
);
