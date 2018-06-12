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
pub fn build_edges(nodes: &[Node]) -> Vec<Edge> {
    let mut res = Vec::new();
    if nodes.len() == 2 {
        res.push(Edge::new(&nodes[0], &nodes[1]));
        res.push(Edge::new(&nodes[1], &nodes[2]));
        res.push(Edge::new(&nodes[2], &nodes[0]));
    } else if nodes.len() == 3 {
        res.push(Edge::new(&nodes[0], &nodes[1]));
        res.push(Edge::new(&nodes[1], &nodes[2]));
        res.push(Edge::new(&nodes[2], &nodes[0]));
    }  else {
        panic!("More than 3 or less than 2 nodes!");
    }
    res
}

pub fn merge<'a>(vec1: Vec<Edge<'a>>, vec2: Vec<Edge<'a>>) ->  Vec<Edge<'a>> {
    let mut res = Vec::new();
    res.extend(vec1);
    res.extend(vec2);
    res
}

pub fn splitter(nodes: &[Node]) ->  usize {
    let index = if nodes.len() == 8 { 4 } else if nodes.len() < 12 { 3 } else { nodes.len() / 2 };
    index
}

pub fn triangulate(nodes: &[Node]) -> Vec<Edge> {    
    if nodes.len() < 3 { panic!("less than 3 nodes!!!"); }

    let mut edges = Vec::new();
 
    if nodes.len() == 3 || nodes.len() == 4 {
        edges.extend(build_edges(&nodes));
    } else {
        let (left, right) = nodes.split_at(splitter(&nodes));
        println!("{:?}", left);
        println!("{:?}", right);
        let edges = merge(triangulate(&left), triangulate(&right));
        //let triangles = merge(, triangulate(left));
        edges.extend(edges);
    } 
    edges
}

fn main() {
    let mut nodes = Vec::new();
    nodes.push(Node::new(1.0, 1.0));
    nodes.push(Node::new(0.0, 0.0));
    nodes.push(Node::new(0.0, 1.0));
    nodes.push(Node::new(0.5, 0.7));
    nodes.push(Node::new(1.1, 3.3));
    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for tr in triangulate(nodes.as_slice()){
        println!("{:?}", tr)
    }
    
}