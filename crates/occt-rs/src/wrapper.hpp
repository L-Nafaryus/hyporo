//#pragma once

#include "rust/cxx.h"

#include <gp.hxx>
#include <gp_Ax2.hxx>
#include <gp_Ax3.hxx>
#include <gp_Circ.hxx>
#include <gp_Lin.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp_Vec.hxx>

#include <TopAbs_ShapeEnum.hxx>

#include <TopoDS.hxx>
#include <TopoDS_Vertex.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Wire.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Compound.hxx>
#include <TopoDS_CompSolid.hxx>
#include <TopoDS_Shell.hxx>
#include <TopoDS_Solid.hxx>
#include <TopExp_Explorer.hxx>

#include <BRepBuilderAPI_MakeVertex.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeShell.hxx>
#include <BRepBuilderAPI_MakeSolid.hxx>
#include <BRepBuilderAPI_Sewing.hxx>

#include <BRep_Tool.hxx>
#include <BRepTools.hxx>
#include <BRepGProp.hxx>
#include <GProp_GProps.hxx>
#include <ShapeUpgrade_UnifySameDomain.hxx>
#include <BRepExtrema_DistShapeShape.hxx>

#include <BRepBuilderAPI_Transform.hxx>
#include <BRepBuilderAPI_GTransform.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>

#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>

#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakePrism.hxx>

/*#include <STEPControl_Writer.hxx>
#include <Interface_Static.hxx>

#include <BRepMesh_IncrementalMesh.hxx>
#include <Poly_PolygonOnTriangulation.hxx>
#include <Poly_Triangulation.hxx>*/

#include <TopLoc_Location.hxx>

#include <NCollection_List.hxx>

#include <ShapeFix_Solid.hxx>
#include <ShapeFix_Shell.hxx>
#include <ShapeFix_Face.hxx>


#include <Geom_Surface.hxx>
#include <GeomLProp_SLProps.hxx>
#include <Poly_Triangle.hxx>


template <typename T, typename... Args> 
std::unique_ptr<T> constructor(Args... args) {
    return std::unique_ptr<T>(new T(args...));
}

using HStandard_Type = opencascade::handle<Standard_Type>;


inline const TopoDS_Vertex& cast_to_vertex(const TopoDS_Shape& shape) {
    return TopoDS::Vertex(shape);
}

inline const TopoDS_Edge& cast_to_edge(const TopoDS_Shape& shape) {
    return TopoDS::Edge(shape);
}

inline const TopoDS_Wire& cast_to_wire(const TopoDS_Shape& shape) {
    return TopoDS::Wire(shape);
}

inline const TopoDS_Face& cast_to_face(const TopoDS_Shape& shape) {
    return TopoDS::Face(shape);
}

inline const TopoDS_Shell& cast_to_shell(const TopoDS_Shape& shape) {
    return TopoDS::Shell(shape);
}

inline const TopoDS_Solid& cast_to_solid(const TopoDS_Shape& shape) {
    return TopoDS::Solid(shape);
}

inline const TopoDS_Compound& cast_to_compound(const TopoDS_Shape& shape) {
    return TopoDS::Compound(shape);
}

inline const TopoDS_CompSolid& cast_to_compsolid(const TopoDS_Shape& shape) {
    return TopoDS::CompSolid(shape);
}


inline const TopoDS_Shape& cast_from_vertex(const TopoDS_Vertex& vertex) {
    return vertex;
}

inline const TopoDS_Shape& cast_from_edge(const TopoDS_Edge& edge) {
    return edge;
}

inline const TopoDS_Shape& cast_from_wire(const TopoDS_Wire& wire) {
    return wire;
}

inline const TopoDS_Shape& cast_from_face(const TopoDS_Face& face) {
    return face;
}

inline const TopoDS_Shape& cast_from_shell(const TopoDS_Shell& shell) {
    return shell;
}

inline const TopoDS_Shape& cast_from_solid(const TopoDS_Solid& solid) {
    return solid;
}

inline const TopoDS_Shape& cast_from_compound(const TopoDS_Compound& compound) {
    return compound;
}

inline const TopoDS_Shape& cast_from_compsolid(const TopoDS_CompSolid& compsolid) {
    return compsolid;
}
