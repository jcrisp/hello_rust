## Simple Times Table Rust Service

### Setup

#### Local

```
cargo build && target/debug/hellorust       # debug

cargo install && target/release/hellorust   # release
```

### Hit the service

http://localhost:8080/?table=3


#### Docker

```
docker build -t jcrisp/hello_rust .           
# show docker file meanwhile

docker run -p 8080:8080 jcrisp/hello_rust

# show running on localhost
```

#### K8
Based on: https://cloud.google.com/kubernetes-engine/docs/quickstarts/deploying-a-language-specific-app

```
gcloud builds submit --tag gcr.io/hellorust/hellorust-gke .

gcloud container clusters create hellorust-gke --num-nodes 1 --enable-basic-auth --issue-client-certificate

# show console
# show deployment.yaml, service.yaml

kubectl apply -f k8/deployment.yaml

# check status
kubectl get deployments
kubectl get pods

kubectl apply -f k8/service.yaml
kubectl get services # to find the IP of load balancer

# clean up
gcloud container clusters delete hellorust-gke
gcloud container images delete gcr.io/hellorust/helloworld-gke
```
