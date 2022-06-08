use crate::Document;

/// A camera's projection.
#[derive(Clone, Debug)]
pub enum Projection<'a, E: json::ThirdPartyExtensions> {
    /// Describes an orthographic projection.
    Orthographic(Orthographic<'a, E>),

    /// Describes a perspective projection.
    Perspective(Perspective<'a, E>),
}

/// A camera's projection.  A node can reference a camera to apply a transform to
/// place the camera in the scene.
#[derive(Clone, Debug)]
pub struct Camera<'a, E: json::ThirdPartyExtensions> {
    /// The parent `Document` struct.
    document: &'a Document<E>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::camera::Camera,
}

///  Values for an orthographic camera projection.
#[derive(Clone, Debug)]
pub struct Orthographic<'a, E: json::ThirdPartyExtensions> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document<E>,

    /// The corresponding JSON struct.
    json: &'a json::camera::Orthographic,
}

/// Values for a perspective camera projection.
#[derive(Clone, Debug)]
pub struct Perspective<'a, E: json::ThirdPartyExtensions> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document<E>,

    /// The corresponding JSON struct.
    json: &'a json::camera::Perspective,
}

impl<'a, E: json::ThirdPartyExtensions> Camera<'a, E> {
    /// Constructs a `Camera`.
    pub(crate) fn new(
        document: &'a Document<E>,
        index: usize,
        json: &'a json::camera::Camera,
    ) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the camera's projection.
    pub fn projection(&self) -> Projection<E> {
        match self.json.type_.unwrap() {
            json::camera::Type::Orthographic => {
                let json = self.json.orthographic.as_ref().unwrap();
                Projection::Orthographic(Orthographic::new(self.document, json))
            }
            json::camera::Type::Perspective => {
                let json = self.json.perspective.as_ref().unwrap();
                Projection::Perspective(Perspective::new(self.document, json))
            }
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a, E: json::ThirdPartyExtensions> Orthographic<'a, E> {
    /// Constructs a `Orthographic` camera projection.
    pub(crate) fn new(document: &'a Document<E>, json: &'a json::camera::Orthographic) -> Self {
        Self { document, json }
    }

    ///  The horizontal magnification of the view.
    pub fn xmag(&self) -> f32 {
        self.json.xmag
    }

    ///  The vertical magnification of the view.
    pub fn ymag(&self) -> f32 {
        self.json.ymag
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> f32 {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a, E: json::ThirdPartyExtensions> Perspective<'a, E> {
    /// Constructs a `Perspective` camera projection.
    pub(crate) fn new(document: &'a Document<E>, json: &'a json::camera::Perspective) -> Self {
        Self { document, json }
    }

    ///  Aspect ratio of the field of view.
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.json.aspect_ratio
    }

    ///  The vertical field of view in radians.
    pub fn yfov(&self) -> f32 {
        self.json.yfov
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> Option<f32> {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
