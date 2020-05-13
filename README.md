## Simple Times Table Rust Service

### Local Setup

#### Native

```
cargo install && target/release/hellorust
```

#### Docker

```
docker build -t jcrisp/hello_rust hello_rust
docker run -p 8080:8080 jcrisp/hello_rust
```

#### Hit the service

http://localhost:8080/?table=3


### K8 Setup on Google Cloud
Based on: https://cloud.google.com/kubernetes-engine/docs/quickstarts/deploying-a-language-specific-app

```
gcloud builds submit --tag gcr.io/hellorust/hellorust-gke .
gcloud container clusters create hellorust-gke --num-nodes 1 --enable-basic-auth --issue-client-certificate
kubectl apply -f k8/deployment.yaml
kubectl apply -f k8/service.yaml # setup load balancer
kubectl get services # to find the IP of load balancer
```
