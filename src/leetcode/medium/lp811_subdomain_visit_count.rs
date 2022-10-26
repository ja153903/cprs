#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        // we can split by space to get the count and the website
        // then we can split the domain into smaller pieces
        let mut result: Vec<String> = Vec::new();
        let mut visits: HashMap<String, i32> = HashMap::new();

        for cpdomain in cpdomains.iter() {
            let mut as_visit_and_domain = cpdomain.split_whitespace();
            let visit: i32 = if let Some(v) = as_visit_and_domain.next() {
                v.parse::<i32>().unwrap()
            } else {
                0
            };

            let domain: Vec<String> = if let Some(d) = as_visit_and_domain.next() {
                d.split('.')
                    .map(|dm| dm.to_string())
                    .collect::<Vec<String>>()
            } else {
                Vec::new()
            };

            for i in 0..domain.len() {
                let mut current_domain = Vec::new();

                for j in i..domain.len() {
                    current_domain.push(domain[j].clone());
                }

                let current_domain_as_string = current_domain.join(".");

                visits
                    .entry(current_domain_as_string)
                    .and_modify(|count| *count += visit)
                    .or_insert(visit);
            }
        }

        for (domain, visit) in visits.iter() {
            result.push(format!("{} {}", visit, domain));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let cpdomains = vec![String::from("9001 discuss.leetcode.com")];

        let mut result = Solution::subdomain_visits(cpdomains);
        result.sort();
        let expected = vec![
            String::from("9001 com"),
            String::from("9001 discuss.leetcode.com"),
            String::from("9001 leetcode.com"),
        ];

        assert_eq!(expected, result);
    }
}
