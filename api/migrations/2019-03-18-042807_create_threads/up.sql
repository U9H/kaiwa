CREATE TABLE threads (
  id SERIAL PRIMARY KEY,
  page_id INTEGER NOT NULL REFERENCES pages (id),
  -- thread ID can be recursive, meaning we can nest comments arbitrarily.
  -- If the thread is null, we assume this to be a top-level comment.
  thread_id INTEGER REFERENCES threads (id),
  body VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
); 