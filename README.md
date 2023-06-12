# Quadtree : Proximity Search

This repository contains a Rust implementation of a quadtree data structure. A quadtree is a tree where each node has exactly four children. It is commonly used for spatial indexing and efficient proximity searches.

## Usage

To use the quadtree implementation in your Rust project, follow these steps:

1. Add the `rand` crate to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8"
```

2. Copy the quadtree code from the provided example and integrate it into your project.

## Quadtree Structure

The quadtree implementation consists of the following components:

### Structs

#### `Point`

- A `Point` represents a location on Earth and consists of `(x, y)` coordinates.
- Typically, these coordinates correspond to latitude and longitude locations.

#### `Boundary`

- The `Boundary` struct defines an enclosed rectangular area.
- It holds the `(x1, x2, y1, y2)` coordinates of the top-left and bottom-right corners of the boundary.

#### `Quadtree`

- The `Quadtree` struct represents a node in the quadtree.
- It contains the following fields:
  - `boundary`: A `Boundary` representing the boundary of the current node.
  - `points`: A vector of `Point` objects that are stored in the current node.
  - `top_left_child`, `bottom_left_child`, `top_right_child`, `bottom_right_child`: Optional boxes that store the child nodes resulting from the subdivision.

### Methods

The quadtree implementation provides the following methods:

#### `create_quad_tree`
```rust
fn create_quad_tree(boundary: Boundary) -> Quadtree
```
- Creates and returns the root node of the quadtree with the specified `boundary`.

#### `insert`
```rust
fn insert(node: &mut Quadtree, point: Point) -> bool
```
- Inserts a point into the quadtree.
- If the number of points in the node reaches the MAX_CAPACITY, the node is subdivided into four child nodes, and the points are distributed among them.
- Returns true if the point was inserted into the node or one of its child nodes.

#### `search`
```rust
fn search(node: &Quadtree, boundary: &Boundary) -> Vec<Point>
```
- Performs a proximity search in the quadtree for all points within the specified `boundary`.
- Returns a vector containing the points found within the boundary.

#### `contains`
```rust
fn contains(boundary: &Boundary, point: Point) -> bool
```
- Checks if a `point` is contained within the `boundary` of the quadtree node.
- Returns `true` if the point is inside the `boundary`; otherwise, returns `false`.

#### `intersects`
```rust
fn intersects(boundary_1: &Boundary, boundary_2: &Boundary) -> bool
```
- Checks if two boundaries `boundary_1` and `boundary_2` intersect.
- Returns `true` if the boundaries intersect; otherwise, returns `false`.

#### `subdivide`
```rust
fn subdivide(node: &mut Quadtree)
```
- Subdivides the `node` into four child nodes and moves the points to their corresponding child nodes.

#### `naive_search`
```rust
fn naive_search(points: &[Point], boundary: &Boundary) -> Vec<Point>
```
- Performs a naive search for all points within the specified `boundary`.
- Returns a vector containing the points found within the `boundary` using a brute-force approach.

## Proximity Search Example
The provided example demonstrates a proximity search using both the quadtree implementation and a naive search algorithm for comparison. It generates a specified number of random points in a 2-dimensional space and populates the quadtree with those points. Then, it performs a search for points within a specified boundary using both the quadtree and the naive search.

To run the example, execute the following command:
```rust
cargo run
```
The example will output the total number of points generated, the elapsed time for populating the points and quadtree, the number of points found using the quadtree search, and the elapsed time for the quadtree search. It will also output the number of points found using the naive search and the elapsed time for the naive search.

Timing both the solutions for searching a provided search boundary across 10 runs using a set of 1 million, 10 million and 100 million points yielded the following results. The results clearly demonstrate that the Quadtree implementation is 20 to 30 times faster than the naive search solution. Note that the results may vary based on the configuration of the machine used to time the solutions. 

| Points      | Quadtree | Naive search | Speedup |
|-------------|----------|--------------|---------|
| 1 million   | 19 us    | 4 ms         | 20      |
| 10 million  | 1 ms     | 30 ms        | 30      |
| 100 million | 23 ms    | 428 ms       | 20      |

## License
This quadtree implementation is provided under the MIT License. Feel free to use and modify it according to your needs.

## Contributing
Contributions are welcome! If you find any issues or want to enhance the implementation, please open an issue or submit a pull request.


