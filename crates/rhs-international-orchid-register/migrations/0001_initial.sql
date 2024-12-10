CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE genera (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE details (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  registar_id INT NOT NULL,
  genus UUID REFERENCES genera(id) NOT NULL,
  epithet TEXT NOT NULL,
  synonym BOOLEAN NOT NULL DEFAULT 'false',
  synonym_genus UUID REFERENCES genera(id),
  synonym_epithet UUID REFERENCES details(id),
  registrant_name TEXT,
  originator_name TEXT,
  date_of_registration DATE,
  seed_parent UUID REFERENCES details(id),
  pollen_parent UUID REFERENCES details(id),
  UNIQUE ( genus, epithet )
);
