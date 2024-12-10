INSERT INTO
  details (
    registar_id,
    genus,
    epithet,
    synonym,
    synonym_genus,
    synonym_epithet,
    registrant_name,
    originator_name,
    date_of_registration,
    seed_parent,
    pollen_parent
  )
VALUES
  (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11
  )
RETURNING *;
