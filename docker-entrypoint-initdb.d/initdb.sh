set -e
psql -U admin sampledb <<EOSQL
CREATE TABLE Todos (
  id        SERIAL PRIMARY KEY,
  title     VARCHAR(20) NOT NULL,
  body      TEXT,
  published BOOL NOT NULL DEFAULT('f')
);
EOSQL
