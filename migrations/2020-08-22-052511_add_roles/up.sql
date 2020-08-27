ALTER TABLE project_users
    ADD view_role TINYINT(1) NOT NULL,
    ADD modify_role TINYINT(1) NOT NULL,
    ADD admin_role TINYINT(1) NOT NULL;
