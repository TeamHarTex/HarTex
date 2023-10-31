--! select_start_timestamp_by_component
SELECT
    *
FROM
    "APIBackend".public."StartTimestamps"
WHERE
    "component" = :component;
