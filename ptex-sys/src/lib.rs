#[cxx::bridge(namespace = "Ptex")]
pub mod ffi {
    /// How to handle mesh border when filtering.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u32)]
    enum BorderMode {
        /// texel access is clamped to border
        #[cxx_name = "m_clamp"]
        Clamp,
        /// texel beyond border are assumed to be black
        #[cxx_name = "m_black"]
        Black,
        /// texel access wraps to other side of face
        #[cxx_name = "m_periodic"]
        Periodic,
    }

    /// Type of data stored in texture file.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u32)]
    enum DataType {
        /// Unsigned, 8-bit integer.
        #[cxx_name = "dt_uint8"]
        UInt8,
        /// Unsigned, 16-bit integer.
        #[cxx_name = "dt_uint16"]
        UInt16,
        /// Half-precision (16-bit) floating point.
        #[cxx_name = "dt_half"]
        Half,
        /// Single-precision (32-bit) floating point.
        #[cxx_name = "dt_float"]
        Float,
    }

    /// How to handle transformation across edges when filtering.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u32)]
    enum EdgeFilterMode {
        /// Don't do anything with the values.
        #[cxx_name = "efm_none"]
        None,
        /// Values are vectors in tangent space; rotate values.
        #[cxx_name = "efm_tanvec"]
        TangentVector,
    }

    /// Edge IDs used in adjacency data in the Ptex::FaceInfo struct.
    /// Edge ID usage for triangle meshes is TBD.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u32)]
    enum EdgeId {
        /// Bottom edge, from UV (0,0) to (1,0)
        #[cxx_name = "e_bottom"]
        Bottom,
        /// Right edge, from UV (1,0) to (1,1)
        #[cxx_name = "e_right"]
        Right,
        /// Top edge, from UV (1,1) to (0,1)
        #[cxx_name = "e_top"]
        Top,
        /// Left edge, from UV (0,1) to (0,0)
        #[cxx_name = "e_left"]
        Left,
    }

    /// Type of meta data entry.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u32)]
    enum MetaDataType {
        /// Null-terminated string.
        #[cxx_name = "mdt_string"]
        String,
        // Signed 8-bit integer.
        #[cxx_name = "mdt_int8"]
        Int8,
        /// Signed 16-bit integer.
        #[cxx_name = "mdt_int16"]
        Int16,
        /// Signed 32-bit integer.
        #[cxx_name = "mdt_int32"]
        Int32,
        /// Single-precision (32-bit) floating point.
        #[cxx_name = "mdt_float"]
        Float,
        /// Double-precision (32-bit) floating point.
        #[cxx_name = "mdt_double"]
        Double,
    }

    /// Type of base mesh for which the textures are defined.  A mesh
    /// can be triangle-based (with triangular textures) or quad-based
    /// (with rectangular textures). */
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u32)]
    enum MeshType {
        /// Mesh is triangle-based.
        #[cxx_name = "mt_triangle"]
        Triangle,
        /// Mesh is quad-based.
        #[cxx_name = "mt_quad"]
        Quad,
    }

    /// Pixel resolution of a given texture.
    /// The resolution is stored in log form: ulog2 = log2(ures), vlog2 = log2(vres)).
    /// Note: negative ulog2 or vlog2 values are reserved for internal use.
    struct Res {
        /// log base 2 of u resolution, in texels
        ulog2: i8,
        /// log base 2 of v resolution, in texels
        vlog2: i8,
    }

    /// Information about a face, as stored in the Ptex file header.
    /// The FaceInfo data contains the face resolution and neighboring face
    /// adjacency information as well as a set of flags describing the face.
    ///
    /// The adjfaces data member contains the face ids of the four neighboring faces.
    /// The neighbors are accessed in EdgeId order, CCW, starting with the bottom edge.
    /// The adjedges data member contains the corresponding edge id for each neighboring face.
    ///
    /// If a face has no neighbor for a given edge, the adjface id should be -1, and the
    /// adjedge id doesn't matter (but is typically zero).
    ///
    /// If an adjacent face is a pair of subfaces, the id of the first subface as encountered
    /// in a CCW traversal should be stored as the adjface id.
    struct FaceInfo {
        /// Resolution of face.
        res: Res,
        ///< Adjacent edges, 2 bits per edge.
        adjedges: u8,
        /// Flags.
        flags: u8,
        /// Adjacent faces (-1 == no adjacent face).
        adjfaces: [u32; 4],
    }

    unsafe extern "C++" {
        include!("Ptexture.h");
        include!("ptex-sys.h");

        type BorderMode;
        type DataType;
        type EdgeFilterMode;
        type MeshType;
        type MetaDataType;
        type PtexCache;
        type PtexTexture;
        type PtexWriter;

        // struct Res

        /// Create a default-constructed Res.
        #[namespace = "Ptex::sys"]
        fn res_default() -> Res;

        /// Create a Res from u and v log2 sizes.
        #[namespace = "Ptex::sys"]
        fn res_from_uv(u: i8, v: i8) -> Res;

        /// Create a Res from a packed u16 value.
        #[namespace = "Ptex::sys"]
        fn res_from_value(value: u16) -> Res;

        /// Return the log2 resolution in the u direction.
        #[namespace = "Ptex::sys"]
        fn res_u(res: &Res) -> i32;

        /// Return the log2 resolution in the v direction.
        #[namespace = "Ptex::sys"]
        fn res_v(res: &Res) -> i32;

        /// Return the size for a Res object.
        #[namespace = "Ptex::sys"]
        fn res_size(res: &Res) -> i32;

        fn val(self: &Res) -> u16;

        /// Return a Res object with swapped u, v values
        #[namespace = "Ptex::sys"]
        fn res_swappeduv(res: &Res) -> Res;

        /// Swap the u, v values in-place on a Res.
        #[namespace = "Ptex::sys"]
        fn res_swapuv(res: &mut Res);

        /// Clamp the resolution value against the given value.
        #[namespace = "Ptex::sys"]
        fn res_clamp(res: &mut Res, clamp_res: &Res);

        /// Determine the number of tiles in the u direction for the given tile res.
        #[namespace = "Ptex::sys"]
        fn res_ntilesu(res: &Res, tileres: Res) -> i32;

        /// Determine the number of tiles in the v direction for the given tile res.
        #[namespace = "Ptex::sys"]
        fn res_ntilesv(res: &Res, tileres: Res) -> i32;

        /// Determine the total number of tiles for the given tile res.
        #[namespace = "Ptex::sys"]
        fn res_ntiles(res: &Res, tileres: Res) -> i32;

        // struct FaceInfo

        /// Create a default-constructed FaceInfo.
        #[namespace = "Ptex::sys"]
        fn faceinfo_default() -> FaceInfo;

        /// Create a FaceInfo from a Res.
        #[namespace = "Ptex::sys"]
        fn faceinfo_from_res(res: Res) -> FaceInfo;

        /// Create a FaceInfo from a Res and adjacency information.
        #[namespace = "Ptex::sys"]
        fn faceinfo_from_res_and_adjacency(
            res: Res,
            adjacent_faces: &[i32; 4],
            adjacent_edges: &[i32; 4],
            is_subface: bool,
        ) -> FaceInfo;

        /// Return true if the FaceInfo contains edits.
        #[namespace = "Ptex::sys"]
        fn faceinfo_has_edits(face_info: &FaceInfo) -> bool;

        /// Return true if the FaceInfo contains constant data.
        #[namespace = "Ptex::sys"]
        fn faceinfo_is_constant(face_info: &FaceInfo) -> bool;

        /// Return true if the FaceInfo is in a neighborhood of constant faces.
        #[namespace = "Ptex::sys"]
        fn faceinfo_is_neighborhood_constant(face_info: &FaceInfo) -> bool;

        /// Return true if the FaceInfo is a subface.
        #[namespace = "Ptex::sys"]
        fn faceinfo_is_subface(face_info: &FaceInfo) -> bool;

        /// Return the adjacent edge for this  face.
        #[namespace = "Ptex::sys"]
        fn faceinfo_adjacent_edge(face_info: &FaceInfo, edge_id: i32) -> EdgeId;

        /// Get the adjacent face for the specified face ID.
        #[namespace = "Ptex::sys"]
        fn faceinfo_adjacent_face(face_info: &FaceInfo, edge_id: i32) -> i32;

        /// Set the adjacent faces.
        #[namespace = "Ptex::sys"]
        fn faceinfo_set_adjacent_faces(
            face_info: &mut FaceInfo,
            f1: i32,
            f2: i32,
            f3: i32,
            f4: i32,
        );

        /// Set the adjacent edges.
        #[namespace = "Ptex::sys"]
        fn faceinfo_set_adjacent_edges(
            face_info: &mut FaceInfo,
            e1: EdgeId,
            e2: EdgeId,
            e3: EdgeId,
            e4: EdgeId,
        );

        /// Return the value of "1.0" for the specified DataType (1.0 (float), 255.0 (8bit), ...).
        #[cxx_name = "OneValue"]
        fn one_value(data_type: DataType) -> f32;

        /// Return the 1.0/value of "1.0" for the specified DataType (1/1.0 (float), 1/255.0 (8bit), ...).
        #[cxx_name = "OneValueInv"]
        fn one_value_inverse(data_type: DataType) -> f32;

        #[cxx_name = "DataSize"]
        fn data_size(data_type: DataType) -> i32;

        /// Allocate a new PtexCache instance.
        /// # Safety
        /// The value returned must be released using ptexcache_release.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexcache_create(
            max_files: i32,
            max_mem: usize,
            premultiply: bool,
        ) -> *mut PtexCache;

        // class PtexCache

        /// Release a PtexCache instance.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexcache_release(cache: *mut PtexCache);

        /// Set the search path on a PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexcache_set_search_path(cache: *mut PtexCache, path: &str);

        /// Get the search path for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexcache_get_search_path(cache: *const PtexCache) -> String;

        // class PtexTexture

        /// Release a PtexTexture instance.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_release(cache: *mut PtexTexture);

        // Return true if the PtexTexture contains edits.
        ///
        /// # Safety
        /// Must be called with a valid cache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_has_edits(cache: *const PtexTexture) -> bool;

        /// Return true if the PtexTexture has mip maps.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_has_mipmaps(cache: *const PtexTexture) -> bool;

        /// Get the alpha channel for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_alpha_channel(cache: *const PtexTexture) -> i32;

        /// Get the number of channels for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_num_channels(cache: *const PtexTexture) -> i32;

        /// Get the number of faces for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_num_faces(cache: *const PtexTexture) -> i32;

        /// Get the path for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_path(cache: *const PtexTexture) -> String;

        /// Get the MeshType for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_meshtype(cache: *const PtexTexture) -> MeshType;

        /// Get the DataType for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_datatype(cache: *const PtexTexture) -> DataType;

        /// Get the BorderMode for the specified PtexCache and direction.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_border_mode_u(cache: *const PtexTexture) -> BorderMode;

        /// Get the BorderMode for the specified PtexCache and direction.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_border_mode_v(cache: *const PtexTexture) -> BorderMode;

        /// Get the EdgeFilterMode for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_edge_filter_mode(cache: *const PtexTexture) -> EdgeFilterMode;

        /// Get the FaceInfo for the specified PtexTexture and faceid.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_face_info<'a>(
            cache: *const PtexTexture,
            faceid: i32,
        ) -> &'a FaceInfo;

        /// Get the pixel value for the specified PtexCache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptextexture_get_pixel(
            cache: *const PtexTexture,
            faceid: i32,
            u: i32,
            v: i32,
            first_channel: i32,
            num_channels: i32,
        ) -> f32;

        /// Open and create a PtexTexture from a PtexCache.
        ///
        /// # Safety
        /// The Texture must not outlive its owning Cache.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexcache_get(
            cache: *mut PtexCache,
            filename: &str,
            error_str: *mut CxxString,
        ) -> *mut PtexTexture;

        /// Create a PtexWriter.
        ///
        /// # Safety
        /// Should not be called outside of the ptex::ffi::sys crate.
        #[allow(clippy::too_many_arguments)]
        #[namespace = "Ptex::sys"]
        unsafe fn ptexwriter_open(
            filename: &str,
            meshtype: MeshType,
            datatype: DataType,
            num_channels: i32,
            alpha_channel: i32,
            num_faces: i32,
            genmipmaps: bool,
            error_str: *mut CxxString,
        ) -> *mut PtexWriter;

        /// Release a PtexWriter.
        ///
        /// # Safety
        /// Must only be called on pointers returned from ptexwriter_open().
        #[namespace = "Ptex::sys"]
        unsafe fn ptexwriter_release(writer: *mut PtexWriter);

        /// Close an open PtexWriter.
        ///
        /// # Safety
        /// Must only be called on valid PtexWriter pointers.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexwriter_close(writer: *mut PtexWriter) -> String;

        /// Writer a face to a PtexWriter
        ///
        /// # Safety
        /// Must only be called on valid PtexWriter pointers.
        #[namespace = "Ptex::sys"]
        unsafe fn ptexwriter_write_face(
            writer: *mut PtexWriter,
            face_id: i32,
            face_info: &FaceInfo,
            data: *const u8,
            stride: i32,
        ) -> bool;
    }
}

impl Copy for Res {}

impl Clone for Res {
    /// Clone a Res to a new instance.
    fn clone(&self) -> Self {
        Res {
            ulog2: self.ulog2,
            vlog2: self.vlog2,
        }
    }
}

impl std::fmt::Debug for Res {
    /// Format a Res for debug display.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Res")
            .field("ulog2", &self.ulog2)
            .field("vlog2", &self.vlog2)
            .finish()
    }
}

impl Default for Res {
    fn default() -> Self {
        ffi::res_default()
    }
}

impl Eq for Res {}

impl PartialEq for Res {
    fn eq(&self, res: &ffi::Res) -> bool {
        self.ulog2 == res.ulog2 && self.vlog2 == res.vlog2
    }
}

impl Res {
    /// Return the log2 resolution in the u direction.
    pub fn u(&self) -> i32 {
        ffi::res_u(self)
    }

    /// Return the log2 resolution in the v direction.
    pub fn v(&self) -> i32 {
        ffi::res_v(self)
    }

    /// Return the size of the FaceInfo.
    pub fn size(&self) -> i32 {
        ffi::res_size(self)
    }

    /// Return a Res instance with swapped u, v values.
    pub fn swapped_uv(&self) -> i32 {
        ffi::res_size(self)
    }
}

impl Default for FaceInfo {
    fn default() -> Self {
        ffi::faceinfo_default()
    }
}

impl FaceInfo {
    pub fn from_res_and_adjacency<T: Into<Res>>(
        res: T,
        adjacent_faces: &[i32; 4],
        adjacent_edges: &[i32; 4],
        is_subface: bool,
    ) -> Self {
        ffi::faceinfo_from_res_and_adjacency(res.into(), adjacent_faces, adjacent_edges, is_subface)
    }

    /// Return a Res resolution struct.
    pub fn resolution(&self) -> Res {
        self.res
    }

    /// Set the resolution for this face.
    pub fn set_resolution<T: Into<Res>>(&mut self, res: T) {
        self.res = res.into();
    }

    /// Return the adjacent edge for this  face.
    pub fn adjacent_edge(&self, edge_id: i32) -> EdgeId {
        ffi::faceinfo_adjacent_edge(self, edge_id)
    }

    /// Set the adjacent edges.
    pub fn set_adjacent_edges(&mut self, e1: EdgeId, e2: EdgeId, e3: EdgeId, e4: EdgeId) {
        ffi::faceinfo_set_adjacent_edges(self, e1, e2, e3, e4);
    }

    /// Get the adjacent face for the specified face ID.
    pub fn adjacent_face(&self, face_id: i32) -> i32 {
        ffi::faceinfo_adjacent_face(self, face_id)
    }

    /// Set the adjacent faces.
    pub fn set_adjacent_faces(&mut self, f1: i32, f2: i32, f3: i32, f4: i32) {
        ffi::faceinfo_set_adjacent_faces(self, f1, f2, f3, f4);
    }

    /// Return true if the FaceInfo contains edits.
    pub fn has_edits(&self) -> bool {
        ffi::faceinfo_has_edits(self)
    }

    /// Return true if the FaceInfo contains constant data.
    pub fn is_constant(&self) -> bool {
        ffi::faceinfo_is_constant(self)
    }

    /// Return true if the FaceInfo is in a neighborhood of constant faces.
    pub fn is_neighborhood_constant(&self) -> bool {
        ffi::faceinfo_is_neighborhood_constant(self)
    }

    /// Return true if the FaceInfo represents a subface.
    pub fn is_subface(&self) -> bool {
        ffi::faceinfo_is_subface(self)
    }
}

impl Copy for FaceInfo {}

impl Clone for FaceInfo {
    /// Clone a FaceInfo into a new instance.
    fn clone(&self) -> Self {
        FaceInfo {
            res: self.res,
            adjedges: self.adjedges,
            flags: self.flags,
            adjfaces: self.adjfaces,
        }
    }
}

pub use ffi::*;
