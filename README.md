# your_miyajestys_yoga
Yoga app for Miya

Allowed REST API requests:

curl --location --request POST '127.0.0.1:8088/api/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "user_id": 1
}'

curl --location --request GET '127.0.0.1:8088/api/users/1'