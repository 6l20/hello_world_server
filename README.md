# hello_world_server
Simple rust web server


## Add a user

curl -X POST http://127.0.0.1:8080/users \
-H "Content-Type: application/json" \
-d '{"name": "Sylvain", "email": "sylvain@example.com"}'

## Get all users

curl http://127.0.0.1:8080/users \ 
-H "Content-Type: application/json"
