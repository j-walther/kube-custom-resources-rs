/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `kom.kaiso.github.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## kom.kaiso.github.io/v1alpha1
- `Microservice`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
