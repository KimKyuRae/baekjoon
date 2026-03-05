use std::io::{stdin, Read};

const INF: i64 = 1e18 as i64;

struct SegmentTree {
    n: usize,
    min_val: Vec<i64>,
    max_val: Vec<i64>,
    min_dp: Vec<i64>,
    min_active: Vec<i64>,  
    max_active: Vec<i64>,
    min_inactive: Vec<i64>,
    max_inactive: Vec<i64>,
    min_total_dp: Vec<i64>,
    lazy_add: Vec<i64>,
    lazy_state: Vec<u8>
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let size = 4 * n + 10;
        Self {
            n,
            min_val: vec![INF; size],
            max_val: vec![-INF; size],
            min_dp: vec![INF; size],
            min_active: vec![INF; size],
            max_active: vec![-INF; size],
            min_inactive: vec![INF; size],
            max_inactive: vec![-INF; size],
            min_total_dp: vec![INF; size],
            lazy_add: vec![0; size],
            lazy_state: vec![0; size],
        }
    }

    fn apply_add(&mut self, node: usize, delta: i64) {
        self.min_val[node] += delta;
        self.max_val[node] += delta;
        
        if self.min_active[node] != INF {
            self.min_active[node] += delta;
            self.max_active[node] += delta;
        }
        if self.min_inactive[node] != INF {
            self.min_inactive[node] += delta;
            self.max_inactive[node] += delta;
        }
        self.lazy_add[node] += delta;
    }

    fn apply_state(&mut self, node: usize, state: u8) {
        if state == 1 {
            self.min_active[node] = self.min_val[node];
            self.max_active[node] = self.max_val[node];
            self.min_inactive[node] = INF;
            self.max_inactive[node] = -INF;
            self.min_total_dp[node] = self.min_dp[node];
        } else {
            self.min_inactive[node] = self.min_val[node];
            self.max_inactive[node] = self.max_val[node];
            self.min_active[node] = INF;
            self.max_active[node] = -INF;
            self.min_total_dp[node] = INF;
        }
        self.lazy_state[node] = state;
    }
    
    fn push_down(&mut self, node: usize) {
        let left = node << 1;
        let right = node << 1 | 1;

        if self.lazy_add[node] != 0 {
            self.apply_add(left, self.lazy_add[node]);
            self.apply_add(right, self.lazy_add[node]);
            self.lazy_add[node] = 0;
        }
        if self.lazy_state[node] > 0 {
            self.apply_state(left, self.lazy_state[node]);
            self.apply_state(right, self.lazy_state[node]);
            self.lazy_state[node] = 0;
        }
    }
    
    fn push_up(&mut self, node: usize) {
        let left = node << 1;
        let right = node << 1 | 1;

        self.min_val[node] = self.min_val[left].min(self.min_val[right]);
        self.max_val[node] = self.max_val[left].max(self.max_val[right]);
        self.min_dp[node] = self.min_dp[left].min(self.min_dp[right]);
        self.min_active[node] = self.min_active[left].min(self.min_active[right]);
        self.max_active[node] = self.max_active[left].max(self.max_active[right]);
        self.min_inactive[node] = self.min_inactive[left].min(self.min_inactive[right]);
        self.max_inactive[node] = self.max_inactive[left].max(self.max_inactive[right]);
        self.min_total_dp[node] = self.min_total_dp[left].min(self.min_total_dp[right]);
    }
    
    fn insert(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64, dp: i64) {
        if start == end {
            self.min_val[node] = val;
            self.max_val[node] = val;
            self.min_dp[node] = dp;
            self.min_active[node] = val;
            self.max_active[node] = val;
            self.min_inactive[node] = INF;
            self.max_inactive[node] = -INF;
            self.min_total_dp[node] = dp;
            return;
        }
        self.push_down(node);
        let mid = (start + end) / 2;
        if idx <= mid {
            self.insert(node << 1, start, mid, idx, val, dp);
        } else {
            self.insert(node << 1 | 1, mid + 1, end, idx, val, dp);
        }
        self.push_up(node);
    }

    fn update_status(&mut self, node: usize, start: usize, end: usize, threshold: i64, is_active_check: bool) {
        if is_active_check {
            if self.max_active[node] <= threshold { return; }
            if self.min_active[node] > threshold { return self.apply_state(node, 2); }
        } else {
            if self.min_inactive[node] > threshold { return; }
            if self.max_inactive[node] <= threshold { return self.apply_state(node, 1); }
        }
        self.push_down(node);
        let mid = (start + end) / 2;
        self.update_status(node << 1, start, mid, threshold, is_active_check);
        self.update_status(node << 1 | 1, mid + 1, end, threshold, is_active_check);
        self.push_up(node);
    }

    fn range_add(&mut self, node: usize, start: usize, end: usize, q_left: usize, q_right: usize, delta: i64, threshold: i64) {
        if q_left <= start && end <= q_right {
            self.apply_add(node, delta);
            return self.update_status(node, start, end, threshold, true);
        }
        self.push_down(node);
        let mid = (start + end) / 2;
        if q_left <= mid {
            self.range_add(node << 1, start, mid, q_left, q_right, delta, threshold);
        }
        if q_right > mid {
            self.range_add(node << 1 | 1, mid + 1, end, q_left, q_right, delta, threshold);
        }
        self.push_up(node);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).ok();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let n = match iter.next() {
        Some(val) => val as usize,
        None => return,
    };

    let arr: Vec<i64> = iter.collect();
    let mut tree = SegmentTree::new(n);
    let mut dp = vec![0i64; n + 1];
    
    let mut max_stack: Vec<(i64, usize, usize)> = Vec::new();
    let mut min_stack: Vec<(i64, usize, usize)> = Vec::new();

    for i in 1..=n {
        let val = arr[i - 1];
        let current_idx = i as i64;
        
        tree.insert(1, 0, n - 1, i - 1, current_idx - 1, dp[i - 1]);
        
        let mut left_boundary = i - 1;
        while max_stack.last().map_or(false, |&(v, _, _)| v < val) {
            let (old_val, l, r) = max_stack.pop().unwrap();
            tree.range_add(1, 0, n - 1, l, r, val - old_val, current_idx);
            left_boundary = l;
        }
        max_stack.push((val, left_boundary, i - 1));

        left_boundary = i - 1;
        while min_stack.last().map_or(false, |&(v, _, _)| v > val) {
            let (old_val, l, r) = min_stack.pop().unwrap();
            tree.range_add(1, 0, n - 1, l, r, old_val - val, current_idx);
            left_boundary = l;
        }
        min_stack.push((val, left_boundary, i - 1));

        tree.update_status(1, 0, n - 1, current_idx, false);
        dp[i] = tree.min_total_dp[1] + 1;
    }

    println!("{}", dp[n]);
}