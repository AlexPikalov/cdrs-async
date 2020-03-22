pub struct LoadBalancerState {
  current_node_index: usize,
}

pub struct LoadBalancer<Next> {
  state: LoadBalancerState,
  next_fn: Next,
}
