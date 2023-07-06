INSERT INTO public."StartTimestamps" ("component", "timestamp") VALUES ($1, $2)
ON CONFLICT ("component")
DO UPDATE SET "timestamp" = $2;