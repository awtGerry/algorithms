/*
   K-means clustering algorithm
   this is a simple implementation of the k-means clustering algorithm, it will
   be implemented in rust using opengl later on.
*/

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Cluster {
    points: Vec<Point>,
    centroid: Point,
}

fn main() {
    let mut points: Vec<Point> = Vec::new();
    let mut clusters: Vec<Cluster> = Vec::new();

    // generate random points
    for _ in 0..100 {
        points.push(Point {
            x: rand::random::<f32>() * 100.0,
            y: rand::random::<f32>() * 100.0,
        });
    }

    // generate random clusters
    for _ in 0..10 {
        clusters.push(Cluster {
            points: Vec::new(),
            centroid: Point {
                x: rand::random::<f32>() * 100.0,
                y: rand::random::<f32>() * 100.0,
            },
        });
    }

    // assign points to clusters
    for point in points.iter() {
        let mut min_distance = std::f32::MAX;
        let mut cluster_index = 0;
        for (index, cluster) in clusters.iter().enumerate() {
            let distance = (point.x - cluster.centroid.x).powi(2) + (point.y - cluster.centroid.y).powi(2);
            if distance < min_distance {
                min_distance = distance;
                cluster_index = index;
            }
        }
        clusters[cluster_index].points.push(point.clone());
    }

    // calculate new centroids
    for cluster in clusters.iter_mut() {
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        for point in cluster.points.iter() {
            x_sum += point.x;
            y_sum += point.y;
        }
        cluster.centroid.x = x_sum / cluster.points.len() as f32;
        cluster.centroid.y = y_sum / cluster.points.len() as f32;
    }

    // print clusters
    for cluster in clusters.iter() {
        println!("Cluster: {:?}", cluster);
    }
}
