docker stop aks-simple-api
docker rm aks-simple-api

docker run --env-file ./my_env -p 8888:8888 --name aks-simple-api -d nickmsft/simple-api-aks:latest