CREATE TABLE users
(
    id            BIGSERIAL PRIMARY KEY,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

INSERT INTO users (username, password_hash)
VALUES ('test1@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$wFbbQLn1SeoUta5S9sxhmw$D4Yih4Gr+bSD8hQxNm/8QmSYNd9x9VuGbjWNI8GQLzU')