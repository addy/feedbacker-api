CREATE SEQUENCE feedback_id_seq;
CREATE SEQUENCE comment_id_seq;
CREATE SEQUENCE poster_id_seq;

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE feedback (
   feedback_id bigint PRIMARY KEY NOT NULL DEFAULT nextval('feedback_id_seq'),
   feedback_title varchar(256),
   feedback_text varchar(2048),
   feedback_score integer DEFAULT 0,
   updt_dt_tm TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   create_dt_tm TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE comment (
   comment_id bigint PRIMARY KEY NOT NULL DEFAULT nextval('comment_id_seq'),
   comment_text varchar(2048),
   feedback_id bigint NOT NULL,
   poster_id bigint NOT NULL,
   updt_dt_tm TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   create_dt_tm TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE poster (
   poster_id bigint PRIMARY KEY NOT NULL DEFAULT nextval('poster_id_seq'),
   poster_name varchar(256),
   pass_hash varchar(256),
   updt_dt_tm TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   create_dt_tm TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON feedback
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON comment
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON poster
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();