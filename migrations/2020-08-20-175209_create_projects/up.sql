CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);
CREATE TABLE project_users (
  id SERIAL PRIMARY KEY,
  user_id BIGINT(20) UNSIGNED NOT NULL,
  CONSTRAINT fk_user_id
    FOREIGN KEY (user_id) REFERENCES users (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  project_id BIGINT(20) UNSIGNED NOT NULL,
  CONSTRAINT fk_project_id
    FOREIGN KEY (project_id) REFERENCES projects (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT uc_project_user UNIQUE (user_id , project_id)
);