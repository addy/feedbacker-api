-- name: create-comment
INSERT INTO comment (
  comment_text,
  feedback_id,
  poster_id
) VALUES (
  $1,
  $2,
  $3
);

-- name: update-comment
UPDATE comment SET comment_text = $1;