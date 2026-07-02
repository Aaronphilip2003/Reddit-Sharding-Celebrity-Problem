-- Schema for one shard database.
-- Usage: psql <shard_db_url> -v shard_count=4 -v start_value=<shard_index + shard_count> -f schema_shard.sql

DROP TABLE IF EXISTS votes;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- author_id has no FK: the comment's author can live on any shard, not
-- necessarily this one, and foreign keys can't reach across databases.
CREATE TABLE comments (
    id BIGSERIAL PRIMARY KEY,
    post_id BIGINT NOT NULL REFERENCES posts(id),
    author_id BIGINT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- user_id has no FK, for the same cross-shard reason as comments.author_id.
CREATE TABLE votes (
    id BIGSERIAL PRIMARY KEY,
    post_id BIGINT NOT NULL REFERENCES posts(id),
    user_id BIGINT NOT NULL,
    value SMALLINT NOT NULL CHECK (value IN (-1, 1)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (post_id, user_id)
);

-- Encode the shard index into every generated id: id % :shard_count always equals this shard's index.
ALTER SEQUENCE users_id_seq INCREMENT BY :shard_count RESTART WITH :start_value;
ALTER SEQUENCE posts_id_seq INCREMENT BY :shard_count RESTART WITH :start_value;
