CREATE TABLE channels (
  id SERIAL PRIMARY KEY,
  creator VARCHAR(48) NOT NULL,
  member VARCHAR(48) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX creator_index
ON channels (creator);

CREATE INDEX member_index
ON channels (member);
