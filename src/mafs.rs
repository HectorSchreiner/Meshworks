pub struct vec3d<T> {
    x: T,
    y: T,
    z: T,
}

pub struct triangle {
    pub p: [vec3d; 3],
}

pub struct mesh {
    pub tris: Vec<triangle>,
}
