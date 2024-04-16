use std::io::Error as StdError;

use cxx::UniquePtr;
use nalgebra::{Scalar, Vector3};
//use opencascade_sys::ffi;
use occt_rs::ffi;

pub struct Point {
    pub data: UniquePtr<ffi::gp_Pnt>,
}

impl Point {
    pub fn with_coords(px: f64, py: f64, pz: f64) -> Self {
        Self {
            data: ffi::gp_Pnt_WithCoords(px, py, pz),
        }
    }

    pub fn set_x(&mut self, px: f64) {
        self.data.pin_mut().SetX(px);
    }

    pub fn set_y(&mut self, py: f64) {
        self.data.pin_mut().SetY(py);
    }

    pub fn set_z(&mut self, pz: f64) {
        self.data.pin_mut().SetZ(pz);
    }

    pub fn x(&self) -> f64 {
        self.data.X()
    }

    pub fn y(&self) -> f64 {
        self.data.Y()
    }

    pub fn z(&self) -> f64 {
        self.data.Z()
    }

    pub fn distance(&self, other: &Point) -> f64 {
        self.data.Distance(&other.data)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            data: ffi::gp_Pnt_Default(),
        }
    }
}

impl From<Vector3<f64>> for Point {
    fn from(vec: Vector3<f64>) -> Self {
        Point::with_coords(vec.x, vec.y, vec.z)
    }
}

#[derive(Debug)]
pub enum ShapeType {
    Compound,
    CompoundSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
    Unknown,
}

impl From<ffi::TopAbs_ShapeEnum> for ShapeType {
    fn from(shape_enum: ffi::TopAbs_ShapeEnum) -> Self {
        match shape_enum {
            ffi::TopAbs_ShapeEnum::TopAbs_SHAPE => Self::Shape,
            ffi::TopAbs_ShapeEnum::TopAbs_VERTEX => Self::Vertex,
            ffi::TopAbs_ShapeEnum::TopAbs_EDGE => Self::Edge,
            ffi::TopAbs_ShapeEnum::TopAbs_WIRE => Self::Wire,
            ffi::TopAbs_ShapeEnum::TopAbs_FACE => Self::Face,
            ffi::TopAbs_ShapeEnum::TopAbs_SHELL => Self::Shell,
            ffi::TopAbs_ShapeEnum::TopAbs_SOLID => Self::Solid,
            ffi::TopAbs_ShapeEnum::TopAbs_COMPSOLID => Self::CompoundSolid,
            ffi::TopAbs_ShapeEnum::TopAbs_COMPOUND => Self::Compound,
            ffi::TopAbs_ShapeEnum { repr: _ } => Self::Unknown,
        }
    }
}

pub struct Vertex {
    pub data: UniquePtr<ffi::TopoDS_Vertex>,
}

impl Vertex {
    pub fn new(point: Point) -> Self {
        let mut make_vertex = ffi::BRepBuilderAPI_MakeVertex_WithPoint(&point.data);
        let vertex = make_vertex.pin_mut().Vertex();

        Self {
            data: ffi::TopoDS_Vertex_ToOwned(vertex),
        }
    }
}
