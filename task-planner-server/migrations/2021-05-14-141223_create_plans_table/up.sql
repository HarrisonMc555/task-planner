CREATE TABLE plans (
  id SERIAL PRIMARY KEY,
  task_id SERIAL NOT NULL,
  title TEXT,
  description TEXT,
  due TIMESTAMP WITH TIME ZONE NOT NULL,
  complete BOOL NOT NULL DEFAULT FALSE,
  FOREIGN KEY (task_id) REFERENCES tasks(id)
)
