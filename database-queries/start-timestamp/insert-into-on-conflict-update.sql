INSERT INTO public."StartTimestamps" ("component", "timestamp") VALUES ($1, $2)
ON CONFLICT ("timestamp")
DO UPDATE SET "timestamp" = $2;