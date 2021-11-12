# Dashboard Access

This object configures the access level of the dashboard for a various number of users.

Two square brackets are required to wrap the name, because this is an array of tables.

### Dashboard Access Object

#### Dashboard Access Structure

| FIELD       | TYPE    | DESCRIPTION                                   |
| ----------- | ------- | --------------------------------------------- |
| userId      | integer | the user ID of the user                       |
| accessLevel | integer | the access level of the user in the dashboard |

#### Dashboard Access Level

| ACCESS LEVEL | INTEGER REPRESENTATION | MEANING                                                               |
| ------------ | ---------------------- | --------------------------------------------------------------------- |
| ADMIN        | 3                      | full access; can add other users to the dashboard                     |
| EDITOR       | 2                      | configuration editor; can edit the configuration but cannot add users |
| VIEWER       | 1                      | viewer; cannot edit configuration whatsoever                          |

#### Example Dashboard Access Object(s)

```toml
[[DashboardAccess]]
userId = 1234567887654321
accessLevel = 3

[[DashboardAccess]]
userId = 2345678998765432
accessLevel = 2

[[DashboardAccess]]
userId = 3456789009876543
accessLevel = 1
```

<div class="info">
    <h5>
        <span class="span">
            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M11 7h2v2h-2zm0 4h2v6h-2zm1-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/></svg>
        </span>
        <span class="span2">
            INFO
        </span>
    </h5>
    <p>All guild owners of guilds with enabled configurations will have the <code>HarTex Guild Owner</code> assigned to them in the support guild automatically. For the array of users, that are specified in the <code>DashboardAccess</code> array, will be granted the <code>HarTex User</code> role in the suppport guild.</p>
</div>
