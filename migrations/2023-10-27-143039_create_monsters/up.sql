-- Your SQL goes here
CREATE TABLE monsters (
    id varchar PRIMARY KEY,
    image_url varchar NOT NULL,
    name varchar NOT NULL,
    attack INT NOT NULL,
    defense INT NOT NULL,
    hp INT NOT NULL,
    speed INT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
SELECT diesel_manage_created_at('monsters');
SELECT diesel_manage_updated_at('monsters');