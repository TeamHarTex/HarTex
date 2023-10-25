-- Users
CREATE ROLE apibackend WITH LOGIN NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE NOREPLICATION;
CREATE ROLE hartex WITH LOGIN NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE NOREPLICATION;
CREATE ROLE hartexnightly with LOGIN NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE NOREPLICATION;

-- APIBackend Database
SELECT 'CREATE DATABASE "APIBackend"' WHERE NOT EXISTS(SELECT FROM pg_database WHERE datname = 'APIBackend')\gexec

-- DROP TABLE IF EXISTS "APIBackend".public."StartTimestamps";
CREATE TABLE IF NOT EXISTS "APIBackend".public."StartTimestamps" (
    "component" TEXT COLLATE pg_catalog."default" NOT NULL,
    "timestamp" TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT "StartTimestamps_pkey" PRIMARY KEY ("component")
);

ALTER TABLE IF EXISTS "APIBackend".public."StartTimestamps" SET TABLESPACE pg_default;
ALTER TABLE IF EXISTS "APIBackend".public."StartTimestamps" OWNER TO postrges;

REVOKE ALL ON TABLE "APIBackend".public."StartTimestamps" FROM apibackend;
GRANT INSERT, SELECT, UPDATE ON TABLE "APIBackend".public."StartTimestamps" TO apibackend;
GRANT ALL ON TABLE "APIBackend".public."StartTimestamps" to postgres;
