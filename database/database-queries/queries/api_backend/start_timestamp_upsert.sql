--! start_timestamp_upsert
INSERT INTO
    "APIBackend".public."StartTimestamps" ("component", "timestamp")
VALUES (:component, :timestamp)
ON CONFLICT ("component") DO UPDATE
    SET
        "timestamp" = :timestamp;
