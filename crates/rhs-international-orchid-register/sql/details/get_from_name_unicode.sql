SELECT details.*
FROM details
INNER JOIN genera
    ON genera.id = details.genus
WHERE epithet LIKE $2 AND genera.name = $1;
