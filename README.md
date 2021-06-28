# devisa_api
## Part of the Devisa Core API

---
### News
- No news!


---
### About
- Responsible for operations falling under the broad umbrella of real-time network communications


---
### Notes


---
### Links
- [dv_api Root] **(You are here!)**:
    - [api-srv](api-srv/README.md)
        - [api-actix](api-srv/api-actix/README.md)
        - [api-tide](api-srv/api-tide/README.md)
        - [api-warp](api-srv/api-warp/README.md)
    - [api-redis](api-redis/README.md)

---
### To run
- ^[06/26/21]^ ^[22:28]^ To run from api server:
```
sudo podman run --name di-redis -d docker.io/redis:latest
sudo podman run --name di-api
    \ -p "80:80"
    \ --env-file .env
    \ quay.io/devisa/devisa-api:latest
```


---
### Todo
