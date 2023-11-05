--! select_user_by_repository_and_permissions
SELECT
    "username"
FROM
    "APIBackend".public."BorsRepositoryPermissions"
WHERE
    "repository" = :repository AND
    "permissions" && :permission_array;
