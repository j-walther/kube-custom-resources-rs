/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `cluster.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## cluster.x-k8s.io/v1alpha3
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
## cluster.x-k8s.io/v1alpha4
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
## cluster.x-k8s.io/v1beta1
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
## cluster.x-k8s.io/v1beta2
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
*/
#[cfg(feature = "v1alpha3")]
pub mod v1alpha3;
#[cfg(feature = "v1alpha4")]
pub mod v1alpha4;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
