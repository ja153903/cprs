#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

#[derive(Debug)]
struct MostViewedVideo {
    id: String,
    views: i32,
}

#[derive(Debug)]
struct Metadata {
    total_views: i32,
    most_viewed_video: MostViewedVideo,
}

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut by_creator: HashMap<String, Metadata> = HashMap::new();

        for i in 0..creators.len() {
            let creator = creators[i].clone();
            let id = ids[i].clone();
            let view_count = views[i];

            by_creator
                .entry(creator.clone())
                .and_modify(|metadata| {
                    metadata.total_views += view_count;
                    if metadata.most_viewed_video.views == view_count {
                        if id.clone() < metadata.most_viewed_video.id {
                            metadata.most_viewed_video.id = id.clone();
                        }
                    } else if metadata.most_viewed_video.views < view_count {
                        metadata.most_viewed_video.views = view_count;
                        metadata.most_viewed_video.id = id.clone();
                    }
                })
                .or_insert(Metadata {
                    total_views: view_count,
                    most_viewed_video: MostViewedVideo {
                        id: id.clone(),
                        views: view_count,
                    },
                });
        }

        let mut max_total_views = 0;

        for metadata in by_creator.values() {
            max_total_views = max_total_views.max(metadata.total_views);
        }

        for (creator, metadata) in by_creator.iter() {
            if metadata.total_views == max_total_views {
                result.push(vec![
                    creator.to_string(),
                    metadata.most_viewed_video.id.to_string(),
                ])
            }
        }

        result
    }
}
