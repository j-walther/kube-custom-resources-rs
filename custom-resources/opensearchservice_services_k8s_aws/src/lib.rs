/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `opensearchservice.services.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## opensearchservice.services.k8s.aws/v1alpha1
- `Domain`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
