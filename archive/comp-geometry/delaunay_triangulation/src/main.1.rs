use std::cmp::Ordering;

// Node start
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub x: f64,
    pub y: f64,
}


impl Node {
    pub fn new(x: f64, y: f64) -> Node {
        Node { x: x, y: y }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.x == other.x
    }
}
// Node end


// Edge start
#[derive(Debug)]
pub struct Edge<'a> {
    source_node: &'a Node,
    target_node: &'a Node,
}

impl <'a> Edge<'a> {
    pub fn new(x: &'a Node, y: &'a Node)-> Edge<'a> {
        Edge {source_node: x, target_node: y}
    }

    pub fn sign(p1: &'a Node, p2: &'a Node, p3: &'a Node,) -> f64 {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }
} 
// Edge end

// Triangle start
#[derive(Debug)]
pub struct Triangle<'a> {
    n1: &'a Node,
    n2: &'a Node,
    n3: &'a Node,
}

impl <'a> Triangle <'a> {
    pub fn new(x: &'a Node, y: &'a Node, z: &'a Node, ) -> Triangle<'a> {
        Triangle {n1: x, n2: y, n3: z}
    }

    pub fn has_inside(&self, x: &Node) -> bool {
        let b1 = Edge::sign(x, self.n1, self.n2) <= 0.0;
        let b2 = Edge::sign(x, self.n2, self.n3) <= 0.0;
        let b3 = Edge::sign(x, self.n3, self.n1) <= 0.0;
        (b1 == b2) && (b2 == b3)
    }

    pub fn is_line(&self) -> bool{
        (self.n2.y - self.n1.y) / (self.n2.x - self.n1.x) == (self.n3.y - self.n1.y) / (self.n3.x - self.n1.x)
    }

    
}
// Triangle end
pub fn build_edges(nodes: &Vec<Node>) -> Vec<Triangle> {
    let mut res = Vec::new();

    if nodes.len() == 3 {
        res.push(Triangle::new(&nodes[0], &nodes[1], &nodes[2]));
        res
    } else if nodes.len() == 4 {
        let nodes_length = nodes.len();

        for i in 0..nodes_length {
            let mut it = (0..nodes_length).filter(|&val| val != i);
            let (x, y, z) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
            let triangle = Triangle::new(&nodes[x], &nodes[y], &nodes[z]);

            if !triangle.has_inside(&nodes[i]) && !triangle.is_line(){
                res.push(triangle);
            }  
        }
        res
    } else {
        panic!("More than 4 or less than 3 nodes!");
    }
    
}

pub fn merge<'a>(vec1: Vec<Triangle<'a>>, vec2: Vec<Triangle<'a>>) ->  Vec<Triangle<'a>> {
    let mut res = Vec::new();
    res.extend(vec1);
    res.extend(vec2);
    res
}

pub fn splitter(nodes: &Vec<Node>) ->  (Vec<Node>, Vec<Node>) {
    let index = if nodes.len() == 8 { 4 } else if nodes.len() < 12 { 3 } else { nodes.len() / 2 };
    let mut left = Vec::new();
    let mut right = Vec::new();
    for i in 1..index {
        left.push(nodes[i].clone());
    }
    for i in index.. {
        right.push(nodes[i]);
    }
    (left, right)
}

fn split_vec(v: &Vec<Node>, index: usize) -> (Vec<Node>, Vec<Node>) {
    assert!(index <= v.len());
    let mut left = Vec::with_capacity(v.len() - index);
    let mut right = Vec::with_capacity(v.len() - left.len());
    (left, right)
}

pub fn triangulate(nodes: &Vec<Node>) -> Vec<Triangle> {    
    if nodes.len() < 3 { panic!("less than 3 nodes!!!"); }

    let mut edges = Vec::new();
 
    if nodes.len() == 3 || nodes.len() == 4 {
        edges.extend(build_edges(&nodes));
    } else {
        let (left, right) = splitter(&nodes);
        let triangles = merge(triangulate(&left), triangulate(&right));
        //let triangles = merge(, triangulate(left));
        edges.extend(triangles);
    } 
    edges
}

fn main() {
    let mut nodes = Vec::new();
    let (point1, point2, point3, point4) = (
        Node::new(1.0, 1.0), 
        Node::new(0.0, 0.0), 
        Node::new(0.0, 1.0),  
        Node::new(0.5, 0.7)
    );
    nodes.push(point1);
    nodes.push(point2);
    nodes.push(point3);
    nodes.push(point4);
    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for tr in triangulate(&nodes){
        println!("{:?}", tr)
    }
    
}