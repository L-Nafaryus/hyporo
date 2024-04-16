#[cxx::bridge]
pub mod ffi {
    #[repr(u32)]
    enum TopAbs_ShapeEnum {
        TopAbs_COMPOUND,
        TopAbs_COMPSOLID,
        TopAbs_SOLID,
        TopAbs_SHELL,
        TopAbs_FACE,
        TopAbs_WIRE,
        TopAbs_EDGE,
        TopAbs_VERTEX,
        TopAbs_SHAPE,
    }

    unsafe extern "C++" {
        include!("wrapper.hpp");

        type HStandard_Type;

        /*
         *  Geometry processors
         *  gp
         */
        type gp_Pnt;

        #[cxx_name = "constructor"]
        #[rust_name = "gp_Pnt_Default"]
        pub fn gp_Pnt() -> UniquePtr<gp_Pnt>;

        #[cxx_name = "constructor"]
        #[rust_name = "gp_Pnt_WithCoords"]
        pub fn gp_Pnt(px: f64, py: f64, pz: f64) -> UniquePtr<gp_Pnt>;

        pub fn SetX(self: Pin<&mut gp_Pnt>, px: f64);

        pub fn SetY(self: Pin<&mut gp_Pnt>, py: f64);

        pub fn SetZ(self: Pin<&mut gp_Pnt>, pz: f64);

        pub fn X(self: &gp_Pnt) -> f64;

        pub fn Y(self: &gp_Pnt) -> f64;

        pub fn Z(self: &gp_Pnt) -> f64;

        pub fn Distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;

        /*
         *  Topology and builders
         *  TopAbs, TopoDS, BRepBuilderAPI
         */

        type TopAbs_ShapeEnum;

        type TopoDS_Vertex;
        type TopoDS_Edge;
        type TopoDS_Wire;
        type TopoDS_Face;
        type TopoDS_Shell;
        type TopoDS_Solid;
        type TopoDS_Compound;
        type TopoDS_CompSolid;
        type TopoDS_Shape;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Vertex_ToOwned"]
        pub fn TopoDS_Vertex(vertex: &TopoDS_Vertex) -> UniquePtr<TopoDS_Vertex>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Edge_ToOwned"]
        pub fn TopoDS_Edge(vertex: &TopoDS_Edge) -> UniquePtr<TopoDS_Edge>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Wire_ToOwned"]
        pub fn TopoDS_Wire(vertex: &TopoDS_Wire) -> UniquePtr<TopoDS_Wire>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Face_ToOwned"]
        pub fn TopoDS_Face(vertex: &TopoDS_Face) -> UniquePtr<TopoDS_Face>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Shell_ToOwned"]
        pub fn TopoDS_Shell(vertex: &TopoDS_Shell) -> UniquePtr<TopoDS_Shell>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Solid_ToOwned"]
        pub fn TopoDS_Solid(vertex: &TopoDS_Solid) -> UniquePtr<TopoDS_Solid>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_Compound_ToOwned"]
        pub fn TopoDS_Compound(vertex: &TopoDS_Compound) -> UniquePtr<TopoDS_Compound>;

        #[cxx_name = "constructor"]
        #[rust_name = "TopoDS_CompSolid_ToOwned"]
        pub fn TopoDS_CompSolid(vertex: &TopoDS_CompSolid) -> UniquePtr<TopoDS_CompSolid>;

        pub fn cast_to_vertex(shape: &TopoDS_Shape) -> &TopoDS_Vertex;
        pub fn cast_to_edge(shape: &TopoDS_Shape) -> &TopoDS_Edge;
        pub fn cast_to_wire(shape: &TopoDS_Shape) -> &TopoDS_Wire;
        pub fn cast_to_face(shape: &TopoDS_Shape) -> &TopoDS_Face;
        pub fn cast_to_shell(shape: &TopoDS_Shape) -> &TopoDS_Shell;
        pub fn cast_to_solid(shape: &TopoDS_Shape) -> &TopoDS_Solid;
        pub fn cast_to_compound(shape: &TopoDS_Shape) -> &TopoDS_Compound;
        pub fn cast_to_compsolid(shape: &TopoDS_Shape) -> &TopoDS_CompSolid;

        pub fn cast_from_vertex(vertex: &TopoDS_Vertex) -> &TopoDS_Shape;
        pub fn cast_from_edge(edge: &TopoDS_Edge) -> &TopoDS_Shape;
        pub fn cast_from_wire(wire: &TopoDS_Wire) -> &TopoDS_Shape;
        pub fn cast_from_face(face: &TopoDS_Face) -> &TopoDS_Shape;
        pub fn cast_from_shell(shell: &TopoDS_Shell) -> &TopoDS_Shape;
        pub fn cast_from_solid(solid: &TopoDS_Solid) -> &TopoDS_Shape;
        pub fn cast_from_compound(compound: &TopoDS_Compound) -> &TopoDS_Shape;
        pub fn cast_from_compsolid(compsolid: &TopoDS_CompSolid) -> &TopoDS_Shape;

        type BRepBuilderAPI_MakeVertex;
        type BRepBuilderAPI_MakeEdge;
        type BRepBuilderAPI_MakeWire;
        type BRepBuilderAPI_MakeFace;
        type BRepBuilderAPI_MakeShell;
        type BRepBuilderAPI_MakeSolid;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepBuilderAPI_MakeVertex_WithPoint"]
        pub fn BRepBuilderAPI_MakeVertex(p: &gp_Pnt) -> UniquePtr<BRepBuilderAPI_MakeVertex>;

        pub fn Vertex(self: Pin<&mut BRepBuilderAPI_MakeVertex>) -> &TopoDS_Vertex;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepBuilderAPI_MakeEdge_WithVertices"]
        pub fn BRepBuilderAPI_MakeEdge(
            v1: &TopoDS_Vertex,
            v2: &TopoDS_Vertex,
        ) -> UniquePtr<BRepBuilderAPI_MakeVertex>;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepBuilderAPI_MakeEdge_WithPoints"]
        pub fn BRepBuilderAPI_MakeEdge(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
        ) -> UniquePtr<BRepBuilderAPI_MakeVertex>;

        pub fn Edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Edge;

        pub fn Vertex1(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Vertex;

        pub fn Vertex2(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Vertex;

        /*
         *  Primitives
         *  BRepPrimAPI
         */

        type Message_ProgressRange;
        type BRepPrimAPI_MakeBox;

        #[cxx_name = "constructor"]
        #[rust_name = "Message_ProgressRange_Default"]
        pub fn Message_ProgressRange() -> UniquePtr<Message_ProgressRange>;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepPrimAPI_MakeBox_Default"]
        pub fn BRepPrimAPI_MakeBox() -> UniquePtr<BRepPrimAPI_MakeBox>;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepPrimAPI_MakeBox_WithSize"]
        pub fn BRepPrimAPI_MakeBox(dx: f64, dy: f64, dz: f64) -> UniquePtr<BRepPrimAPI_MakeBox>;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepPrimAPI_MakeBox_WithCorner"]
        pub fn BRepPrimAPI_MakeBox(
            p: &gp_Pnt,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeBox>;

        #[cxx_name = "constructor"]
        #[rust_name = "BRepPrimAPI_MakeBox_WithCorners"]
        pub fn BRepPrimAPI_MakeBox(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<BRepPrimAPI_MakeBox>;

        #[rust_name = "Init_WithSize"]
        pub fn Init(self: Pin<&mut BRepPrimAPI_MakeBox>, dx: f64, dy: f64, dz: f64);

        #[rust_name = "Init_WithCorner"]
        pub fn Init(self: Pin<&mut BRepPrimAPI_MakeBox>, p: &gp_Pnt, dx: f64, dy: f64, dz: f64);

        #[rust_name = "Init_WithCorners"]
        pub fn Init(self: Pin<&mut BRepPrimAPI_MakeBox>, p1: &gp_Pnt, p2: &gp_Pnt);

        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeBox>, theRange: &Message_ProgressRange);

        pub fn IsDone(self: &BRepPrimAPI_MakeBox) -> bool;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Shape;

        pub fn Shell(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Shell;

        pub fn Solid(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Solid;

        pub fn BottomFace(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Face;

        pub fn BackFace(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Face;

        pub fn FrontFace(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Face;

        pub fn LeftFace(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Face;

        pub fn RightFace(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Face;

        pub fn TopFace(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Face;

    }
}
