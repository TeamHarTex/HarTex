# Permission Levels

This object is for configuring permission levels for roles and users.

### Permission Levels Object

This object is a collection of two key-value pairs, `roles` and `users` respectively, of permission levels, 
from `0` to `100`, where `100` is the highest permission level, and `0` is the lowest.

#### Permission Levels Structure

| FIELD | TYPE | DESCRIPTION                                               |
|-------|------|-----------------------------------------------------------|
| roles | map? | a key-value pair representing permission levels for roles |
| users | map? | a key-value pair representing permission levels for users |

#### Example Permission Levels Object
```toml
[PermissionLevels.roles]
1234567887654321 = 100
2345678998765432 = 90
3456789009876543 = 80
9876543223456789 = 50
8765432112345678 = 10

[PermissionLevels.users]
1000000000000001 = 100
2000000000000002 = 90
```
