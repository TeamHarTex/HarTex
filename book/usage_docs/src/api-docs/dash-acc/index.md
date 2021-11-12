# Dashboard Access

This model configures the access level of the dashboard for a various number of users.

Two square brackets are required to wrap the name, because this is an array of tables.

### Dashboard Access object

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
