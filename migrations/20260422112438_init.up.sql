CREATE TABLE city (
    id SERIAL PRIMARY KEY,
    department_code VARCHAR NOT NULL,
    insee_code VARCHAR,
    zip_code VARCHAR,
    name VARCHAR NOT NULL,
    lat FLOAT NOT NULL,
    lon FLOAT NOT NULL
);
