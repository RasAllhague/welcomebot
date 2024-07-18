# database

## guild

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | name | varchar | 50 | N | |
| UQ | guild_id | bigint | | N | |
| | welcome_message | varchar | 255 | Y | |
| | welcome_channel | bigint | | Y | |
| FK | back_banner | integer | | N | | 
| FK | front_banner | integer | | N | |
| | create_user_id | bigint | | N | |
| | create_date | timestamp | | N | |
| | modify_user_id | bigint | | Y | |
| | modify_date | timestamp | | Y | |

## image

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | original_name | varchar | 255 | N | |
| | server_name | varchar | 255 | N | |
| | path | varchar | 255 | N | |
| | width | integer | | N | |
| | height | integer | | N | |
| | size | bigint | | N | |
| | create_user_id | bigint | | N | |
| | create_date | bigint | | N | |
