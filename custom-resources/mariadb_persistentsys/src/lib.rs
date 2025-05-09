/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `mariadb.persistentsys` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## mariadb.persistentsys/v1alpha1
- `Backup`
- `MariaDB`
- `Monitor`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
