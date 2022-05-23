# Static IP
Issue this command to get the resource group needed to issue a public IP. 

```
$ az aks show --resource-group peerchat --name peerchat-k8s
```

The issuing command might look like this:

```
$ az network public-ip create \
    --resource-group MC_peerchat_peerchat-k8s_germanywestcentral \
    --name peerchatIP \
    --sku Standard \ 
    --allocation-method static \
    --query publicIp.ipAddress
```

# Setup Ingress

Add nginx ingress to your helmet repo and install it

```
$ helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx

$ STATIC_IP=<YOUR_IP>
$ DNS_LABEL=<UNIQUE_AZURE_DOMAIN_NAME>

$ helm install ingress-ngninx ingress-nginx/ingress-nginx \
    --version 4.0.13 \
    --namespace peerchat \
    --set controller.service.loadBalancerIP=$STATIC_IP \
    --set controller.service.annotations."service\.beta\.kubernetes\.io/azure-dns-label-name"=$DNS_LABEL
```

```
$ kubectl label namespace peerchat cert-manager.io/disable-validation=true
```

# Jetstack setup
```
$ helm repo add jetstack https://charts.jetstack.io
$ helm repo update
```

```
$ helm install cert-manager jetstack/cert-manager \
    --namespace peerchat \
    --set installCRDs=true
```