DROP TABLE IF EXISTS nodes;
CREATE TABLE nodes {
    id SERIAL PRIMARY KEY,
    public_key VARCHAR(66) NOT NULL,
    alias VARCHAR(256) NOT NULL,
    capacity FLOAT NOT NULL,
    first_seen DateTime<Utc>
}
