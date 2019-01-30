CREATE ROLE injection_role LOGIN PASSWORD 'injection_password';
CREATE DATABASE injection OWNER injection_role;
