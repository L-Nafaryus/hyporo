use cxx::UniquePtr;
use opencascade_sys::ffi::{
    compute_normals, cylinder_to_surface, ellipse_to_HandleGeom2d_Curve, ellipse_value,
    gp_Ax2_ctor, gp_Ax2d_ctor, gp_Ax3_from_gp_Ax2, gp_DZ, gp_Dir2d_ctor, gp_OX,
    handle_geom_plane_location, new_HandleGeomCurve_from_HandleGeom_TrimmedCurve,
    new_HandleGeomPlane_from_HandleGeomSurface, new_list_of_shape, new_point, new_point_2d,
    new_transform, new_vec, shape_list_append_face, type_name, write_stl, BRepAlgoAPI_Fuse_ctor,
    BRepBuilderAPI_MakeEdge_CurveSurface2d, BRepBuilderAPI_MakeEdge_HandleGeomCurve,
    BRepBuilderAPI_MakeFace_wire, BRepBuilderAPI_MakeWire_ctor, BRepBuilderAPI_MakeWire_edge_edge,
    BRepBuilderAPI_MakeWire_edge_edge_edge, BRepBuilderAPI_Transform_ctor,
    BRepFilletAPI_MakeFillet_ctor, BRepLibBuildCurves3d, BRepMesh_IncrementalMesh_ctor,
    BRepOffsetAPI_MakeThickSolid_ctor, BRepOffsetAPI_ThruSections_ctor,
    BRepPrimAPI_MakeCylinder_ctor, BRepPrimAPI_MakePrism_ctor, BRep_Builder_ctor,
    BRep_Builder_upcast_to_topods_builder, BRep_Tool_Surface, BRep_Tool_Triangulation, DynamicType,
    ExplorerCurrentShape, GCE2d_MakeSegment_point_point, GC_MakeArcOfCircle_Value,
    GC_MakeArcOfCircle_point_point_point, GC_MakeSegment_Value, GC_MakeSegment_point_point,
    Geom2d_Ellipse_ctor, Geom2d_TrimmedCurve_ctor, Geom_CylindricalSurface_ctor,
    HandleGeom2d_TrimmedCurve_to_curve, Handle_Poly_Triangulation_Get, MakeThickSolidByJoin,
    Poly_Triangulation_Node, Poly_Triangulation_Normal, Poly_Triangulation_UV, StlAPI_Writer_ctor,
    TColgp_Array1OfDir_ctor, TopAbs_Orientation, TopAbs_ShapeEnum, TopExp_Explorer_ctor,
    TopLoc_Location_Transformation, TopLoc_Location_ctor, TopoDS_Compound_as_shape,
    TopoDS_Compound_ctor, TopoDS_Face, TopoDS_Face_to_owned, TopoDS_Shape_to_owned,
    TopoDS_cast_to_edge, TopoDS_cast_to_face, TopoDS_cast_to_wire,
};

use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};

#[derive(Debug)]
pub struct BMesh {
    pub vertices: Vec<[f32; 3]>, //Vec<Vec<f64>>,
    pub uv: Vec<[f32; 2]>,       //Vec<Vec<f64>>,
    pub normals: Vec<[f32; 3]>,  //Vec<Vec<f64>>,
    pub indices: Vec<usize>,     //Vec<usize>,
}

pub fn bottle_mesh() -> BMesh {
    let height = 70.0;
    let width = 50.0;
    let thickness = 30.0;

    // Define the points making up the bottle's profile.
    let point_1 = new_point(-width / 2.0, 0.0, 0.0);
    let point_2 = new_point(-width / 2.0, -thickness / 4.0, 0.0);
    let point_3 = new_point(0.0, -thickness / 2.0, 0.0);
    let point_4 = new_point(width / 2.0, -thickness / 4.0, 0.0);
    let point_5 = new_point(width / 2.0, 0.0, 0.0);

    // Define the arcs and segments of the profile.
    let arc = GC_MakeArcOfCircle_point_point_point(&point_2, &point_3, &point_4);
    let segment_1 = GC_MakeSegment_point_point(&point_1, &point_2);
    let segment_2 = GC_MakeSegment_point_point(&point_4, &point_5);

    let mut edge_1 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_1)),
    );

    let mut edge_2 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeArcOfCircle_Value(&arc)),
    );

    let mut edge_3 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_2)),
    );

    let mut wire = BRepBuilderAPI_MakeWire_edge_edge_edge(
        edge_1.pin_mut().Edge(),
        edge_2.pin_mut().Edge(),
        edge_3.pin_mut().Edge(),
    );

    let x_axis = gp_OX();

    let mut transform = new_transform();
    transform.pin_mut().set_mirror_axis(x_axis);

    // We're calling Shape() here instead of Wire(), hope that's okay.
    let mut brep_transform =
        BRepBuilderAPI_Transform_ctor(wire.pin_mut().Shape(), &transform, false);
    let mirrored_shape = brep_transform.pin_mut().Shape();
    let mirrored_wire = TopoDS_cast_to_wire(mirrored_shape);

    let mut make_wire = BRepBuilderAPI_MakeWire_ctor();
    make_wire.pin_mut().add_wire(wire.pin_mut().Wire());
    make_wire.pin_mut().add_wire(mirrored_wire);

    let wire_profile = make_wire.pin_mut().Wire();

    let mut face_profile = BRepBuilderAPI_MakeFace_wire(wire_profile, false);
    let prism_vec = new_vec(0.0, 0.0, height);
    // We're calling Shape here instead of Face(), hope that's also okay.
    let mut body =
        BRepPrimAPI_MakePrism_ctor(face_profile.pin_mut().Shape(), &prism_vec, false, true);

    let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(body.pin_mut().Shape());
    let mut edge_explorer =
        TopExp_Explorer_ctor(body.pin_mut().Shape(), TopAbs_ShapeEnum::TopAbs_EDGE);

    while edge_explorer.More() {
        let edge = TopoDS_cast_to_edge(edge_explorer.Current());
        make_fillet.pin_mut().add_edge(thickness / 12.0, edge);
        edge_explorer.pin_mut().Next();
    }

    let body_shape = make_fillet.pin_mut().Shape();

    // Make the bottle neck
    let neck_location = new_point(0.0, 0.0, height);
    let neck_axis = gp_DZ();
    let neck_coord_system = gp_Ax2_ctor(&neck_location, neck_axis);

    let neck_radius = thickness / 4.0;
    let neck_height = height / 10.0;

    let mut cylinder = BRepPrimAPI_MakeCylinder_ctor(&neck_coord_system, neck_radius, neck_height);
    let cylinder_shape = cylinder.pin_mut().Shape();

    let mut fuse_neck = BRepAlgoAPI_Fuse_ctor(body_shape, cylinder_shape);
    let body_shape = fuse_neck.pin_mut().Shape();

    // Make the bottle hollow
    let mut face_explorer = TopExp_Explorer_ctor(body_shape, TopAbs_ShapeEnum::TopAbs_FACE);
    let mut z_max = -1.0;
    let mut top_face: Option<UniquePtr<TopoDS_Face>> = None;

    while face_explorer.More() {
        let shape = ExplorerCurrentShape(&face_explorer);
        let face = TopoDS_cast_to_face(&shape);

        let surface = BRep_Tool_Surface(face);
        let dynamic_type = DynamicType(&surface);
        let name = type_name(dynamic_type);

        if name == "Geom_Plane" {
            let plane_handle = new_HandleGeomPlane_from_HandleGeomSurface(&surface);
            let plane_location = handle_geom_plane_location(&plane_handle);

            let plane_z = plane_location.Z();
            if plane_z > z_max {
                z_max = plane_z;
                top_face = Some(TopoDS_Face_to_owned(face));
            }
        }

        face_explorer.pin_mut().Next();
    }

    let top_face = top_face.unwrap();

    let mut faces_to_remove = new_list_of_shape();
    shape_list_append_face(faces_to_remove.pin_mut(), &top_face);

    let mut solid_maker = BRepOffsetAPI_MakeThickSolid_ctor();
    MakeThickSolidByJoin(
        solid_maker.pin_mut(),
        body_shape,
        &faces_to_remove,
        -thickness / 50.0,
        1.0e-3,
    );

    let body_shape = solid_maker.pin_mut().Shape();

    // Create the threading
    let cylinder_axis = gp_Ax3_from_gp_Ax2(&neck_coord_system);
    let cylinder_1 = Geom_CylindricalSurface_ctor(&cylinder_axis, neck_radius * 0.99);
    let cylinder_1 = cylinder_to_surface(&cylinder_1);
    let cylinder_2 = Geom_CylindricalSurface_ctor(&cylinder_axis, neck_radius * 1.05);
    let cylinder_2 = cylinder_to_surface(&cylinder_2);

    let a_pnt = new_point_2d(std::f64::consts::TAU, neck_height / 2.0);
    let a_dir = gp_Dir2d_ctor(std::f64::consts::TAU, neck_height / 4.0);
    let thread_axis = gp_Ax2d_ctor(&a_pnt, &a_dir);

    let a_major = std::f64::consts::TAU;
    let a_minor = neck_height / 10.0;

    let ellipse_1 = Geom2d_Ellipse_ctor(&thread_axis, a_major, a_minor);
    let ellipse_1_handle = ellipse_to_HandleGeom2d_Curve(&ellipse_1);
    let ellipse_2 = Geom2d_Ellipse_ctor(&thread_axis, a_major, a_minor / 4.0);
    let ellipse_2_handle = ellipse_to_HandleGeom2d_Curve(&ellipse_2);
    let arc_1 = Geom2d_TrimmedCurve_ctor(&ellipse_1_handle, 0.0, std::f64::consts::PI);
    let arc_1 = HandleGeom2d_TrimmedCurve_to_curve(&arc_1);
    let arc_2 = Geom2d_TrimmedCurve_ctor(&ellipse_2_handle, 0.0, std::f64::consts::PI);
    let arc_2 = HandleGeom2d_TrimmedCurve_to_curve(&arc_2);

    let ellipse_point_1 = ellipse_value(&ellipse_1, 0.0);
    let ellipse_point_2 = ellipse_value(&ellipse_1, std::f64::consts::PI);
    let thread_segment = GCE2d_MakeSegment_point_point(&ellipse_point_1, &ellipse_point_2);
    let thread_segment = HandleGeom2d_TrimmedCurve_to_curve(&thread_segment);

    let mut edge_1_on_surface_1 = BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_1, &cylinder_1);
    let mut edge_2_on_surface_1 =
        BRepBuilderAPI_MakeEdge_CurveSurface2d(&thread_segment, &cylinder_1);
    let mut edge_1_on_surface_2 = BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_2, &cylinder_2);
    let mut edge_2_on_surface_2 =
        BRepBuilderAPI_MakeEdge_CurveSurface2d(&thread_segment, &cylinder_2);

    let mut threading_wire_1 = BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_1.pin_mut().Edge(),
        edge_2_on_surface_1.pin_mut().Edge(),
    );
    let mut threading_wire_2 = BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_2.pin_mut().Edge(),
        edge_2_on_surface_2.pin_mut().Edge(),
    );

    // TODO - does calling Shape() work here instead of Wire()?
    BRepLibBuildCurves3d(threading_wire_1.pin_mut().Shape());
    BRepLibBuildCurves3d(threading_wire_2.pin_mut().Shape());

    let is_solid = true;
    let mut threading_loft = BRepOffsetAPI_ThruSections_ctor(is_solid);
    threading_loft
        .pin_mut()
        .AddWire(threading_wire_1.pin_mut().Wire());
    threading_loft
        .pin_mut()
        .AddWire(threading_wire_2.pin_mut().Wire());
    threading_loft.pin_mut().CheckCompatibility(false);

    let threading_shape = threading_loft.pin_mut().Shape();

    // Build the resulting compound
    let mut compound = TopoDS_Compound_ctor();
    let builder = BRep_Builder_ctor();
    let builder = BRep_Builder_upcast_to_topods_builder(&builder);
    builder.MakeCompound(compound.pin_mut());

    let mut compound_shape = TopoDS_Compound_as_shape(compound);
    builder.Add(compound_shape.pin_mut(), body_shape);
    builder.Add(compound_shape.pin_mut(), threading_shape);

    let final_shape = compound_shape;

    let mut triangulation = BRepMesh_IncrementalMesh_ctor(&final_shape, 0.01);

    let mut vertices = vec![];
    let mut uvs = vec![];
    let mut normals = vec![];
    let mut indices = vec![];

    let mut face_explorer = TopExp_Explorer_ctor(
        triangulation.pin_mut().Shape(),
        TopAbs_ShapeEnum::TopAbs_FACE,
    );

    while face_explorer.More() {
        let shape = ExplorerCurrentShape(&face_explorer);
        let face = TopoDS_cast_to_face(&shape);

        let mut location = TopLoc_Location_ctor();

        let triangulation_handle = BRep_Tool_Triangulation(face, location.pin_mut());

        let triangulation = Handle_Poly_Triangulation_Get(&triangulation_handle).unwrap();

        let index_offset = vertices.len();
        let face_point_count = triangulation.NbNodes();

        for i in 1..=face_point_count {
            let mut point = Poly_Triangulation_Node(triangulation, i);
            point
                .pin_mut()
                .Transform(&TopLoc_Location_Transformation(&location));
            vertices.push([point.X() as f32, point.Y() as f32, point.Z() as f32]);
        }

        let mut u_min = f64::INFINITY;
        let mut v_min = f64::INFINITY;

        let mut u_max = f64::NEG_INFINITY;
        let mut v_max = f64::NEG_INFINITY;

        for i in 1..=(face_point_count) {
            let uv = Poly_Triangulation_UV(triangulation, i);
            let (u, v) = (uv.X(), uv.Y());

            u_min = u_min.min(u);
            v_min = v_min.min(v);

            u_max = u_max.max(u);
            v_max = v_max.max(v);

            uvs.push([u as f32, v as f32]);
        }

        // Normalize the newly added UV coordinates.
        for uv in &mut uvs[index_offset..(index_offset + face_point_count as usize)] {
            uv[0] = (uv[0] - u_min as f32) / (u_max - u_min) as f32;
            uv[1] = (uv[1] - v_min as f32) / (v_max - v_min) as f32;

            if face.Orientation() != TopAbs_Orientation::TopAbs_FORWARD {
                uv[0] = 1.0 - uv[0];
            }
        }

        // Add in the normals.
        // TODO(bschwind) - Use `location` to transform the normals.
        let normal_array = TColgp_Array1OfDir_ctor(0, face_point_count);

        compute_normals(face, &triangulation_handle);

        // TODO(bschwind) - Why do we start at 1 here?
        for i in 1..(normal_array.Length() as usize) {
            let normal = Poly_Triangulation_Normal(triangulation, i as i32);
            normals.push([normal.X() as f32, normal.Y() as f32, normal.Z() as f32]);
        }

        for i in 1..=triangulation.NbTriangles() {
            let triangle = triangulation.Triangle(i);

            if face.Orientation() == TopAbs_Orientation::TopAbs_FORWARD {
                indices.push(index_offset + triangle.Value(1) as usize - 1);
                indices.push(index_offset + triangle.Value(2) as usize - 1);
                indices.push(index_offset + triangle.Value(3) as usize - 1);
            } else {
                indices.push(index_offset + triangle.Value(3) as usize - 1);
                indices.push(index_offset + triangle.Value(2) as usize - 1);
                indices.push(index_offset + triangle.Value(1) as usize - 1);
            }
        }

        face_explorer.pin_mut().Next();
    }

    BMesh {
        vertices,
        uv: uvs,
        normals,
        indices,
    }
}

#[derive(Component)]
struct CustomUV;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Import the custom texture.
    //let custom_texture_handle: Handle<Image> = asset_server.load("textures/array_texture.png");
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_bottle_mesh());

    // Render the mesh with the custom texture using a PbrBundle, add the marker.
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh_handle,
            material: materials.add(StandardMaterial {
                //base_color_texture: Some(custom_texture_handle),
                ..default()
            }),
            ..default()
        },
        CustomUV,
    ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(35.0, 35.0, 35.0).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    // Light up the scene.
    commands.spawn(PointLightBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    // Text to describe the controls.
    commands.spawn(
        TextBundle::from_section(
            "Controls:\nSpace: Change UVs\nX/Y/Z: Rotate\nR: Reset orientation",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mesh_query: Query<&Handle<Mesh>, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mesh_handle = mesh_query.get_single().expect("Query not successful");
        //let mesh = meshes.get_mut(mesh_handle).unwrap();
        //toggle_texture(mesh);
    }
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}

fn create_bottle_mesh() -> Mesh {
    let mesh = bottle_mesh();

    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        mesh.vertices,
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uv)
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals)
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_inserted_indices(Indices::U32(
        mesh.indices.into_iter().map(|x| x as u32).collect(),
    ))
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}
