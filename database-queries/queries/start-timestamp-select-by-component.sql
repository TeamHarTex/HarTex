--! select_start_timestamp_by_component
SELECT
    *
FROM
    public."StartTimestamps"
WHERE
    "component" = :component;
