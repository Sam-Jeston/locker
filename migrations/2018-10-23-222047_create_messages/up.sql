CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  channel_id INTEGER REFERENCES channels(id),
  message TEXT,
  nonce VARCHAR(60),
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
);
