use rand::Rng;
use std::time::Instant;

// A Point holds (x,y) coordinates for a location on earth
// Usually these would be the latitude and longitude locations
type Point = (f64, f64);

// Boundary defines an enclosed rectangular area.
struct Boundary {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

// Quadtree is a tree where each node in the tree will have exactly 4 children.
// Each node will contain points upto 'MAX_CAPACITY'
// Once the number of points in a node have reached capacity, the node will be subdivided into 4 child nodes and all the points will be distributed to the child nodes
struct Quadtree {
    boundary: Boundary,
    points: Vec<Point>,
    top_left_child: Option<Box<Quadtree>>,
    bottom_left_child: Option<Box<Quadtree>>,
    top_right_child: Option<Box<Quadtree>>,
    bottom_right_child: Option<Box<Quadtree>>,
}

// maximum number of points that can be accomodated in a node before it subdivides into 4 child nodes
const MAX_CAPACITY: usize = 100;

// Inserts a point in the Quadtree
// If the number of points in the node are already at capacity, then the node will be subdivided in 4 child nodes
// Post the sub-division the point will be added to the child node that it fits into
// returns true if the point was inserted into the node or one of its child nodes
fn insert(node: &mut Quadtree, point: Point) -> bool {
    // check if the point is outside the node's boundary, if yes then return false
    if !contains(&node.boundary, point) {
        return false;
    }

    // if node has not reached capacacity and has not been sub-divided, insert the point in this node
    if node.points.len() < MAX_CAPACITY && node.top_left_child.is_none() {
        node.points.push(point);
        return true;
    }

    // if we reach here, that means there could be 2 possibilities
    // 1. the node has already been sub-divided or
    // 2. the node has reached its capacity but has not been sub-divided

    // if node has reached its capacity but has not yet been sub-divided, we need to sub-divide
    if node.top_left_child.is_none() {
        subdivide(node);
    }

    // Insert the point into its correct child node.
    // We can try inserting into all the child nodes
    // The node where the point's position is outside the boundary would
    // return false, until we find the correct child node.

    if insert(node.top_left_child.as_mut().unwrap(), point) {
        return true;
    }
    if insert(node.bottom_left_child.as_mut().unwrap(), point) {
        return true;
    }
    if insert(node.top_right_child.as_mut().unwrap(), point) {
        return true;
    }
    if insert(node.bottom_right_child.as_mut().unwrap(), point) {
        return true;
    }

    //we should not reach here
    false
}

// check if a point is contained within the (x, y) co-ordinates
// of the boundary's top-left and bottom-right corner
fn contains(boundary: &Boundary, point: Point) -> bool {
    point.0 >= boundary.x1
        && point.0 <= boundary.x2
        && point.1 >= boundary.y1
        && point.1 <= boundary.y2
}

// returns true if the 2 boundaries intersect
fn intersects(boundary_1: &Boundary, boundary_2: &Boundary) -> bool {
    boundary_1.x1 <= boundary_2.x2
        && boundary_1.x2 >= boundary_2.x1
        && boundary_1.y1 <= boundary_2.y2
        && boundary_1.y2 >= boundary_2.y1
}

// subdivide splits the node into 4 child nodes and moves the points in the node
// to their correct child nodes
fn subdivide(node: &mut Quadtree) {
    // create 4 child nodes based on the boundary of the current node
    let x1 = node.boundary.x1;
    let x2 = node.boundary.x2;
    let y1 = node.boundary.y1;
    let y2 = node.boundary.y2;
    let mid_x = (x1 + x2) / 2.0;
    let mid_y = (y1 + y2) / 2.0;

    node.top_left_child = Some(Box::new(Quadtree {
        boundary: Boundary {
            x1,
            x2: mid_x,
            y1,
            y2: mid_y,
        },
        points: Vec::new(),
        top_left_child: None,
        bottom_left_child: None,
        top_right_child: None,
        bottom_right_child: None,
    }));

    node.bottom_left_child = Some(Box::new(Quadtree {
        boundary: Boundary {
            x1,
            x2: mid_x,
            y1: mid_y,
            y2,
        },
        points: Vec::new(),
        top_left_child: None,
        bottom_left_child: None,
        top_right_child: None,
        bottom_right_child: None,
    }));

    node.top_right_child = Some(Box::new(Quadtree {
        boundary: Boundary {
            x1: mid_x,
            x2,
            y1,
            y2: mid_y,
        },
        points: Vec::new(),
        top_left_child: None,
        bottom_left_child: None,
        top_right_child: None,
        bottom_right_child: None,
    }));

    node.bottom_right_child = Some(Box::new(Quadtree {
        boundary: Boundary {
            x1: mid_x,
            x2,
            y1: mid_y,
            y2,
        },
        points: Vec::new(),
        top_left_child: None,
        bottom_left_child: None,
        top_right_child: None,
        bottom_right_child: None,
    }));

    // move points in the node to the child nodes that should contain the point.
    // we try inserting each point into all the child nodes.
    // if the position is outside the child node's boundary, insert will return false.
    // if insert returns true that means we have found our correct child node for that point.

    let mut child_nodes = [
        node.top_left_child.as_mut().unwrap(),
        node.bottom_left_child.as_mut().unwrap(),
        node.top_right_child.as_mut().unwrap(),
        node.bottom_right_child.as_mut().unwrap(),
    ];

    for point in &node.points {
        for child_node in &mut child_nodes {
            if insert(child_node, *point) {
                break;
            }
        }
    }

    // no longer need points in the node
    node.points = Vec::new();
}

// search returns all the points within the given boundary
fn search(node: &Quadtree, boundary: &Boundary) -> Vec<Point> {
    // if this node does not interesect with the search boundary
    // we know that the node and all its child nodes do not contain any points
    // that fall in the search boundary
    if !intersects(&node.boundary, boundary) {
        return vec![];
    }

    // If this node has not yet been subdivided, return
    // all the points within the search boundary
    if node.top_left_child.is_none() {
        return node
            .points
            .iter()
            .filter(|&point| contains(boundary, *point))
            .cloned()
            .collect();
    }

    // If the node has been subdivided, search all
    // the child nodes and merge the results
    let mut result: Vec<Point> = Vec::new();
    result.extend(search(node.top_left_child.as_ref().unwrap(), boundary));
    result.extend(search(node.bottom_left_child.as_ref().unwrap(), boundary));
    result.extend(search(node.top_right_child.as_ref().unwrap(), boundary));
    result.extend(search(node.bottom_right_child.as_ref().unwrap(), boundary));

    result
}

// create the root node for the Quadtree
fn create_quad_tree(boundary: Boundary) -> Quadtree {
    Quadtree {
        boundary,
        points: Vec::new(),
        top_left_child: None,
        bottom_left_child: None,
        top_right_child: None,
        bottom_right_child: None,
    }
}

// naive search implementation
// here points correspond to all the locations in our 2 dimnesional space
// boundary represents the rectangular region
// the function returns all the points contained in the rectangular region
fn naive_search(points: &[Point], boundary: &Boundary) -> Vec<Point> {
    points
        .iter()
        .filter(|&point| contains(boundary, *point))
        .cloned()
        .collect()
}

fn main() {
    // total points in our 2 dimensional space
    //let total_points = 1_000_000; // 1 million
    //let total_points = 10_000_000; // 10 million
    let total_points = 100_000_000; // 100 million
    println!(
        "Total number of points in our 2 dimensional space {} ",
        total_points
    );

    // points vector will represent the list of points for our naive search
    let mut points: Vec<Point> = Vec::new();

    // create the root node of the quad tree
    // upper bound for x and y co-ordinates is 100
    // lower bound for x and y co-ordinates is 0
    let mut quadtree = create_quad_tree(Boundary {
        x1: 0.0,
        x2: 100.0,
        y1: 0.0,
        y2: 100.0,
    });

    // initialize thread_rng()
    let mut rng = rand::thread_rng();

    let start_time = Instant::now();
    // generate random points and add them to the points vector and quadtree
    for _ in 0..total_points {
        let x = rng.gen_range(0.0..=100.0);
        let y = rng.gen_range(0.0..=100.0);
        let point = (x, y);

        points.push(point);
        insert(&mut quadtree, point);
    }

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time for populating points and quadtree: {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );

    // search for points within the specified Boundary using Quadtree
    // here we are considering 10.0 and 15.0 as the lower and upper bounds for both x and y co-ordinates
    // ideally this boundary will be defined based on the rectangular region we want to search w.r.t to a point.
    let start_time = Instant::now();
    println!(
        "Quadtree search yielded {} points",
        search(
            &quadtree,
            &Boundary {
                x1: 10.0,
                x2: 15.0,
                y1: 10.0,
                y2: 15.0,
            },
        )
        .len()
    );

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time Quadtree search: {}s {}ms {} us",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis(),
        elapsed_time.subsec_micros(),
    );

    // search for points within the specified Boundary using naive search
    let start_time = Instant::now();
    println!(
        "Naive search yielded {} points",
        naive_search(
            &points,
            &Boundary {
                x1: 10.0,
                x2: 15.0,
                y1: 10.0,
                y2: 15.0,
            },
        )
        .len()
    );
    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time Naive search: {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}
