-- name: list-feedback
SELECT
   *
FROM
   feedback f;

-- name: show-feedback
SELECT
   f.*,
   c.*
FROM
   feedback f
LEFT JOIN comment c ON f.feedback_id = c.feedback_id
WHERE feedback_id = $1;

-- name: create-feedback
INSERT INTO feedback (
   feedback_title,
   feedback_text
) VALUES (
   $1,
   $2
);

-- name: update-feedback
UPDATE feedback SET feedback_title = $1, feedback_text = $2, feedback_score = $3;